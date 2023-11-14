pub struct ClientArgs {
    pub username: String,
}

impl ClientArgs {
    pub fn new(username: String) -> ClientArgs {
        ClientArgs { username }
    }

    pub fn parse(args: Vec<String>) -> Result<ClientArgs, String> {
        let mut iterator = args.into_iter();

        let username = match iterator.next() {
            Some(username) => username,
            None => return Err("No username was provided".to_string()),
        };

        Ok(ClientArgs::new(username))
    }
}
