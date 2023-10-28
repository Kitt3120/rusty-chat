use std::fmt::Display;

pub enum CancellationTokenError {
    AlreadyCancelled,
    PoisonError(String),
}

impl Display for CancellationTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CancellationTokenError::AlreadyCancelled => {
                write!(f, "The ressource was already cancelled")
            }
            CancellationTokenError::PoisonError(reason) => {
                write!(f, "The ressource was poisoned: {}", reason)
            }
        }
    }
}
