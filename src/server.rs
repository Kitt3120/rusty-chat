pub mod args;
pub mod client_handle;
pub mod client_handler;
pub mod command;

pub use args::ServerArgs;
pub use client_handle::ClientHandle;
pub use client_handler::ClientHandler;

use std::net::TcpListener;

use crate::{
    common::{
        cli::CommandHandlerBuilder, threading::cancellation_token_source::CancellationTokenSource,
    },
    server::command::exit::ExitCommand,
};

//TODO: Clean up
pub fn run(server_args: ServerArgs) -> Result<(), String> {
    println!(
        "Binding TcpListener to {}:{}",
        server_args.address, server_args.port
    );
    let address = format!("{}:{}", server_args.address, server_args.port);
    let tcp_listener = match TcpListener::bind(address) {
        Ok(tcp_listener) => tcp_listener,
        Err(err) => return Err(format!("Error binding to address: {}", err)),
    };

    println!("Initializing CommandHandler");
    let mut cancellation_token_source = CancellationTokenSource::new();
    let cancellation_token = match cancellation_token_source.new_token() {
        Ok(cancellation_token) => cancellation_token,
        Err(err) => return Err(format!("Error creating CancellationToken: {}", err)),
    };

    let command_handler = CommandHandlerBuilder::new()
        .add_command(Box::new(ExitCommand::new(cancellation_token_source)))
        .build();

    println!("Intitializing ClientHandler");
    let client_handler = match ClientHandler::new(tcp_listener) {
        Ok(client_handler) => client_handler,
        Err(err) => return Err(format!("Error creating ClientHandler: {}", err)),
    };

    println!("Server started");

    let command_handler_result = command_handler.handle_stdin(cancellation_token);
    if let Err(err) = command_handler_result {
        return Err(format!("Error handling stdin: {}", err));
    }

    let client_handler_result = client_handler.stop();
    if let Err(err) = client_handler_result {
        return Err(format!("Error stopping ClientHandler: {}", err));
    }

    println!("Server stopped");
    Ok(())
}
