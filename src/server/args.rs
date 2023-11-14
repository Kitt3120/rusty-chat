#[derive(Debug, Clone, PartialEq)]
pub struct ServerArgs {
    pub address: String,
    pub port: u8,
    pub announce_interval: u8,
}

impl ServerArgs {
    pub fn new(address: String, port: u8, announce_interval: u8) -> ServerArgs {
        ServerArgs {
            address,
            port,
            announce_interval,
        }
    }

    pub fn parse(args: Vec<String>) -> Result<ServerArgs, String> {
        let mut iterator = args.into_iter();

        let address: String = match iterator.next() {
            Some(address) => address,
            None => return Err(String::from("No address was provided")),
        };

        let port: String = match iterator.next() {
            Some(port) => port,
            None => return Err(String::from("No port was provided")),
        };

        let port: u8 = match port.parse() {
            Ok(port) => port,
            Err(err) => return Err(format!("Error parsing port to number: {}", err.to_string())),
        };

        let announce_interval: String = match iterator.next() {
            Some(port) => port,
            None => return Err(String::from("No announce interval was provided")),
        };

        let announce_interval: u8 = match announce_interval.parse() {
            Ok(port) => port,
            Err(err) => {
                return Err(format!(
                    "Error parsing announce interval to number: {}",
                    err.to_string()
                ))
            }
        };

        Ok(ServerArgs::new(address, port, announce_interval))
    }
}
