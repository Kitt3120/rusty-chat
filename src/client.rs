pub mod args;

pub use args::ClientArgs;

pub fn run(client_args: ClientArgs) -> Result<(), String> {
    println!("Starting client as user {}", client_args.username);
    Ok(())
}
