use crate::{
    common::{
        message_stream::MessageStream,
        protocol::{
            error::HandshakeError::{self, AuthenticationFailed, UnexpectedMessage},
            handshake::server::{Handshake, HandshakeArguments},
        },
        threading::{CancellationToken, CancellationTokenError, CancellationTokenSource},
    },
    server::ClientHandle,
};
use std::{
    any::Any,
    boxed::Box,
    marker::Send,
    net::TcpListener,
    result::Result,
    string::String,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

#[derive(Debug)]
pub struct ClientHandler {
    cancellation_token_source: CancellationTokenSource,
    client_acceptance_thread: JoinHandle<Result<(), String>>,
    client_handles: Arc<Mutex<Vec<ClientHandle>>>,
}

impl ClientHandler {
    pub fn new(tcp_listener: TcpListener) -> Result<ClientHandler, CancellationTokenError> {
        let client_handles = Arc::new(Mutex::new(Vec::new()));
        let client_handles_clone = Arc::clone(&client_handles);

        let mut cancellation_token_source = CancellationTokenSource::new();
        let thread_cancellation_token = cancellation_token_source.new_token()?;
        let client_acceptance_thread = thread::spawn(move || {
            handle_clients(
                client_handles_clone,
                tcp_listener,
                thread_cancellation_token,
            )
        });

        Ok(ClientHandler {
            cancellation_token_source,
            client_acceptance_thread,
            client_handles,
        })
    }

    pub fn get_client_handles(&self) -> Arc<Mutex<Vec<ClientHandle>>> {
        Arc::clone(&self.client_handles)
    }

    pub fn cancel(&self) -> Result<(), CancellationTokenError> {
        self.cancellation_token_source.cancel()
    }

    pub fn is_cancelled(&self) -> Result<bool, CancellationTokenError> {
        self.cancellation_token_source.is_cancelled()
    }

    pub fn join(self) -> Result<Result<(), String>, Box<dyn Any + Send>> {
        self.client_acceptance_thread.join()
    }

    pub fn stop(self) -> Result<(), String> {
        match self.cancel() {
            Ok(_) => (),
            Err(err) => match err {
                CancellationTokenError::AlreadyCancelled => (), // We use these match statements to ignore this error and being exhaustive, so that we have to adapt the code if we change the error type
                CancellationTokenError::PoisonError(_) => {
                    return Err(format!(
                        "Error while cancelling the enclosed CancellationTokenSource: {}",
                        err
                    ))
                }
            },
        };

        match self.join() {
            Ok(join_result) => match join_result {
                Ok(_) => (),
                Err(err) => {
                    return Err(format!("Error while joining the enclosed thread: {}", err))
                }
            },
            Err(_) => {
                return Err(String::from(
                    "Unable to join the enclosed thread. The thread panicked.",
                ))
            }
        };

        Ok(())
    }
}

//TODO: Custom error enums instead of String
fn handle_clients(
    client_handles: Arc<Mutex<Vec<ClientHandle>>>,
    tcp_listener: TcpListener,
    cancellation_token: Arc<CancellationToken>,
) -> Result<(), String> {
    // We set the TcpListener to non-blocking, so that we can check the cancellation token periodically instead of blocking the thread
    tcp_listener.set_nonblocking(true).map_err(|err| {
        format!(
            "Error while setting the TcpListener to non-blocking: {}",
            err
        )
    })?;

    loop {
        let cancelled = match cancellation_token.is_cancelled() {
            Ok(cancelled) => cancelled,
            Err(err) => {
                return Err(format!(
                    "Error while checking the CancellationToken: {}",
                    err
                ))
            }
        };

        if cancelled {
            println!("Cancel has been requested, shutting down the ClientHandler");
            break;
        }

        //TODO: Currently, clients are handled on one thread. A possible DOS attack would be to open a connection and not send any data. This would block the thread until timeout and prevent other clients from connecting. We should spawn a new thread for each client, so that we can handle multiple clients at once.

        let (tcp_stream, socket_addr) = match tcp_listener.accept() {
            Ok(tcp_result) => tcp_result,
            Err(err) => match err.kind() {
                std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(250));
                    continue;
                }
                std::io::ErrorKind::ConnectionAborted => {
                    eprintln!("A client tried to connect but the connection was aborted by the remote host. Dropping its connection and continuing to accept clients.");
                    continue;
                }
                std::io::ErrorKind::ConnectionRefused => {
                    eprintln!("A client tried to connect but the connection was refused by the remote host. Dropping its connection and continuing to accept clients.");
                    continue;
                }
                std::io::ErrorKind::ConnectionReset => {
                    eprintln!("A client tried to connect but the connection was reset by the remote host. Dropping its connection and continuing to accept clients.");
                    continue;
                }
                std::io::ErrorKind::Interrupted => {
                    eprintln!("A client tried to connect but the connection was interrupted. Dropping its connection and continuing to accept clients.");
                    continue;
                }
                std::io::ErrorKind::BrokenPipe => {
                    eprintln!("A client tried to connect but a pipe was closed/broken. Dropping its connection and continuing to accept clients.");
                    continue;
                }
                std::io::ErrorKind::TimedOut => {
                    eprintln!("A client tried to connect but the connection timed out. Dropping its connection and continuing to accept clients.");
                    continue;
                }
                _ => {
                    return Err(format!(
                        "Critical error while accepting a client, server has to shut down: {}",
                        err
                    ))
                }
            },
        };

        let mut client_handles = match client_handles.lock() {
            Ok(vector) => vector,
            Err(err) => {
                return Err(format!(
                    "Critical error while accepting a client, server has to shut down: Failed to lock client_handles vector: {}",
                    err
                ))
            }
        };

        let taken_usernames = client_handles
            .iter()
            .map(|client_handle| client_handle.handshake.username.as_str())
            .collect::<Vec<&str>>();

        let ip_addr = socket_addr.ip();
        let mut message_stream = MessageStream::new(tcp_stream);

        let handshake_arguments = HandshakeArguments::new(taken_usernames);
        let handshake_result = Handshake::perform(&mut message_stream, handshake_arguments);

        let handshake = match handshake_result {
            Ok(handshake) => handshake,
            Err(err) => match err {
                AuthenticationFailed(reason) => {
                    eprintln!(
                        "While handshaking, client {} tried to authenticate but the authentication failed: {}",
                        ip_addr, reason
                    );
                    continue;
                }
                UnexpectedMessage(message) => {
                    eprintln!(
                        "While handshaking, client {} tried to authenticat but sent an unexpected message instead: {}",
                        ip_addr, message
                    );
                    continue;
                }
                HandshakeError::MessageStreamError(err) => {
                    eprintln!("While handshaking, an error occured while streaming the response of the client {}: {}", ip_addr, err);
                    continue;
                }
            },
        };

        let client_handle = ClientHandle::new(socket_addr, message_stream, handshake);
        client_handles.push(client_handle);
    }
    Ok(())
}
