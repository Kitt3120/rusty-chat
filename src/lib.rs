pub mod client;
pub mod common;
pub mod server;

pub fn run(args: Vec<String>) -> Result<(), String> {
    if args.len() < 1 {
        return Err(String::from("Usage:\nrusty_chat <username>: Start client\nrusty_chat <address> <port> <announcement interval>: Start server"));
    }

    let username = &args[0];

    if username == "server" {
        let server_args = server::args::ServerArgs::parse(args)?;
        return server::run(server_args);
    } else {
        let client_args = client::args::ClientArgs::parse(args)?;
        return client::run(client_args);
    }
}
