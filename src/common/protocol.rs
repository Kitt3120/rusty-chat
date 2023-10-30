pub mod client;
pub mod error;
pub mod message;
pub mod serializable;
pub mod server;

pub use error::MessageParseError;
pub use message::Message;
pub use serializable::Serializable;

//TODO:
// - [ ] Add debug, display and clone traits to remaining enums
//  - [ ] CancelationToken
//  - [ ] CancelationTokenError
//  - [ ] CancellationTokenSource
// - [ ] Split Serializable implementations of Server and Client messages into separate files (handlers)
// - [ ] Add MessageStream implementation building on top of TcpStream
// - [ ] Derefs for Messages?
