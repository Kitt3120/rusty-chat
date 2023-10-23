use core::panic;
use std::{
    fmt::Display,
    sync::{Arc, Mutex},
    vec,
};

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

pub struct CancellationTokenSource {
    tokens: Mutex<Vec<Arc<CancellationToken>>>,
    cancelled: Mutex<bool>,
}

impl CancellationTokenSource {
    pub fn new() -> CancellationTokenSource {
        CancellationTokenSource {
            tokens: Mutex::new(vec![]),
            cancelled: Mutex::new(false),
        }
    }

    pub fn new_token(&mut self) -> Result<Arc<CancellationToken>, CancellationTokenError> {
        let cancelled = match self.cancelled.lock() {
            Ok(mutex) => mutex,
            Err(err) => return Err(CancellationTokenError::PoisonError(err.to_string())),
        };

        if *cancelled {
            return Err(CancellationTokenError::AlreadyCancelled);
        }

        let mut vector = match self.tokens.lock() {
            Ok(mutex) => mutex,
            Err(err) => return Err(CancellationTokenError::PoisonError(err.to_string())),
        };

        let token = Arc::new(CancellationToken::new());
        vector.push(Arc::clone(&token));
        Ok(token)
    }

    pub fn cancel(&self) -> Result<(), CancellationTokenError> {
        let mut cancelled = match self.cancelled.lock() {
            Ok(mutex) => mutex,
            Err(err) => return Err(CancellationTokenError::PoisonError(err.to_string())),
        };

        if *cancelled {
            return Err(CancellationTokenError::AlreadyCancelled);
        }

        let vector = match self.tokens.lock() {
            Ok(mutex) => mutex,
            Err(err) => return Err(CancellationTokenError::PoisonError(err.to_string())),
        };

        for (index, cancellation_token) in vector.iter().enumerate() {
            if let Err(CancellationTokenError::PoisonError(reason)) = cancellation_token.cancel() {
                return Err(CancellationTokenError::PoisonError(format!("While cancelling CancellationTokenSource, the contained CancellationToken at index {} was poisoned: {}", index, reason)));
            }
        }

        *cancelled = true;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, CancellationTokenError> {
        match self.cancelled.lock() {
            Ok(mutex) => Ok(*mutex),
            Err(err) => Err(CancellationTokenError::PoisonError(err.to_string())),
        }
    }
}

impl Drop for CancellationTokenSource {
    fn drop(&mut self) {
        if let Err(CancellationTokenError::PoisonError(reason)) = self.cancel() {
            panic!("Unable to drop CancellationTokenSource: {}", reason);
        }
    }
}

unsafe impl Send for CancellationTokenSource {}
unsafe impl Sync for CancellationTokenSource {}

pub struct CancellationToken {
    cancelled: Mutex<bool>,
}

impl CancellationToken {
    pub fn new() -> CancellationToken {
        CancellationToken {
            cancelled: Mutex::new(false),
        }
    }

    pub fn cancel(&self) -> Result<(), CancellationTokenError> {
        let mut mutex = match self.cancelled.lock() {
            Ok(mutex) => mutex,
            Err(err) => return Err(CancellationTokenError::PoisonError(err.to_string())),
        };

        if *mutex {
            return Err(CancellationTokenError::AlreadyCancelled);
        }

        *mutex = true;
        Ok(())
    }

    pub fn is_cancelled(&self) -> Result<bool, CancellationTokenError> {
        match self.cancelled.lock() {
            Ok(mutex) => Ok(*mutex),
            Err(err) => return Err(CancellationTokenError::PoisonError(err.to_string())),
        }
    }
}

unsafe impl Send for CancellationToken {}
unsafe impl Sync for CancellationToken {}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::common::cancellation_token::CancellationTokenError;

    use super::CancellationTokenSource;

    #[test]
    fn test_token_initializes_uncancelled() {
        let mut cancellation_token_source = CancellationTokenSource::new();

        let token = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the CancellationToken: {}", err),
        };

        assert!(!token.is_cancelled().unwrap_or_else(|err| panic!(
            "Error while asserting that the CancellationToken initializes as uncancelled: {}",
            err
        )));
    }

    #[test]
    fn test_token_cancelled_after_cancel() {
        let mut cancellation_token_source = CancellationTokenSource::new();

        let token = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the CancellationToken: {}", err),
        };

        token
            .cancel()
            .unwrap_or_else(|err| panic!("Error while cancelling the CancellationToken: {}", err));

        assert!(token.is_cancelled().unwrap_or_else(|err| {
            panic!(
                "Error while asserting that the CancellationToken is cancelled: {}",
                err
            );
        }));
    }

    #[test]
    fn test_token_cant_be_double_cancelled() {
        let mut cancellation_token_source = CancellationTokenSource::new();

        let token = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the CancellationToken: {}", err),
        };

        token.cancel().unwrap_or_else(|err| {
            panic!(
                "Error while cancelling the CancellationToken the first time: {}",
                err
            )
        });

        let failed = match token.cancel() {
            Ok(()) => panic!("CancellationToken was cancelled twice successfully"),
            Err(err) => match err {
                CancellationTokenError::AlreadyCancelled => true,
                CancellationTokenError::PoisonError(reason) => panic!(
                    "When cancelling CancellationToken the second time, it was poinsoned: {}",
                    reason
                ),
            },
        };

        assert!(failed);
    }

    #[test]
    fn test_source_cancelled_after_cancel() {
        let mut cancellation_token_source: CancellationTokenSource = CancellationTokenSource::new();

        match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the dummy CancellationToken: {}", err),
        };

        cancellation_token_source.cancel().unwrap_or_else(|err| {
            panic!(
                "Error while cancelling the CancellationTokenSource: {}",
                err
            )
        });

        assert!(cancellation_token_source
            .is_cancelled()
            .unwrap_or_else(|err| {
                panic!(
                    "Error while asserting that the CancellationTokenSource is cancelled: {}",
                    err
                );
            }));
    }

    #[test]
    fn test_source_all_tokens_cancelled_after_cancel() {
        let mut cancellation_token_source = CancellationTokenSource::new();

        let token_first = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the first CancellationToken: {}", err),
        };

        let token_second = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the second CancellationToken: {}", err),
        };

        cancellation_token_source.cancel().unwrap_or_else(|err| {
            panic!(
                "Error while cancelling the CancellationTokenSource: {}",
                err
            )
        });

        assert!(token_first.is_cancelled().unwrap_or_else(|err| panic!(
            "Error while asserting that the first CancellationToken is cancelled: {}",
            err
        )));

        assert!(token_second.is_cancelled().unwrap_or_else(|err| panic!(
            "Error while asserting that the second CancellationToken is cancelled: {}",
            err
        )));
    }

    #[test]
    fn test_source_all_tokens_cancelled_after_drop() {
        let mut cancellation_token_source = CancellationTokenSource::new();

        let token_first = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the first CancellationToken: {}", err),
        };

        let token_second = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the second CancellationToken: {}", err),
        };

        drop(cancellation_token_source);

        assert!(token_first.is_cancelled().unwrap_or_else(|err| panic!(
            "Error while asserting that the first CancellationToken is cancelled: {}",
            err
        )));

        assert!(token_second.is_cancelled().unwrap_or_else(|err| panic!(
            "Error while asserting that the second CancellationToken is cancelled: {}",
            err
        )));
    }
}
