pub mod args;
pub mod client_handle;
pub mod client_handler;

pub use args::ServerArgs;
pub use client_handle::ClientHandle;
pub use client_handler::ClientHandler;

use std::net::TcpListener;

pub fn run(server_args: ServerArgs) -> Result<(), String> {
    println!(
        "Starting Server on {}:{}",
        server_args.address, server_args.port
    );

    let address = format!("{}:{}", server_args.address, server_args.port);
    let tcp_listener = match TcpListener::bind(address) {
        Ok(tcp_listener) => tcp_listener,
        Err(err) => return Err(format!("Error binding to address: {}", err)),
    };
    println!(
        "Bound TcpListener on {}:{}",
        server_args.address, server_args.port
    );

    let client_handler = match ClientHandler::new(tcp_listener) {
        Ok(client_handler) => client_handler,
        Err(err) => return Err(format!("Error creating ClientHandler: {}", err)),
    };
    println!("Started ClientHandler");

    //TODO: Implement CLI commands

    if let Err(err) = client_handler.join() {
        return Err(format!("Error joining ClientHandler: {:?}", err));
    }
    println!("Server finished");
    Ok(())
}
