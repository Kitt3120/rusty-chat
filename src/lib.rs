pub mod client;
pub mod common;
pub mod server;

pub fn run(args: Vec<String>) -> Result<(), String> {
    match args.len() {
        1 => {
            let client_args = client::args::ClientArgs::parse(args)?;
            client::run(client_args)
        },
        3 => {
            let server_args = server::args::ServerArgs::parse(args)?;
            server::run(server_args)
        }
        _ => Err(String::from("Invalid amount of arguments provided\nUsage:\nrusty_chat <username>: Start client\nrusty_chat <address> <port> <announcement interval>: Start server")),
    }
}
