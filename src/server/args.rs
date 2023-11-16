#[derive(Debug, Clone, PartialEq)]
pub struct ServerArgs {
    pub address: String,
    pub port: u16,
    pub announce_interval: u16,
}

impl ServerArgs {
    pub fn new(address: String, port: u16, announce_interval: u16) -> ServerArgs {
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

        let port: u16 = match port.parse() {
            Ok(port) => port,
            Err(err) => return Err(format!("Error parsing port to number: {}", err)),
        };

        let announce_interval: String = match iterator.next() {
            Some(port) => port,
            None => return Err(String::from("No announce interval was provided")),
        };

        let announce_interval: u16 = match announce_interval.parse() {
            Ok(port) => port,
            Err(err) => {
                return Err(format!(
                    "Error parsing announce interval to number: {}",
                    err
                ))
            }
        };

        Ok(ServerArgs::new(address, port, announce_interval))
    }
}

//TODO: Tests
