use std::env;

fn main() {
    let mut arguments = env::args().collect::<Vec<String>>();
    arguments.remove(0);
    let status = rusty_chat::run(arguments);
    if let Err(err) = status {
        eprintln!("Error while running rusty_chat: {}", err);
    }
}
