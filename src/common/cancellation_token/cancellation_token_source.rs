use super::{cancellation_token::CancellationToken, error::CancellationTokenError};
use core::panic;
use std::{
    sync::{Arc, Mutex},
    vec,
};

#[derive(Debug)]
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

#[cfg(test)]
mod tests {

    use super::super::error::CancellationTokenError;
    use super::CancellationTokenSource;

    #[test]
    fn test_source_initializes_as_uncancelled() {
        let mut cancellation_token_source = CancellationTokenSource::new();

        assert!(!cancellation_token_source
            .is_cancelled()
            .unwrap_or_else(|err| panic!(
                "Error while asserting that the CancellationTokenSource initialized as uncancelled: {}",
                err
            )));
    }

    #[test]
    fn test_source_initializes_empty() {
        let cancellation_token_source = CancellationTokenSource::new();

        assert!(cancellation_token_source
            .tokens
            .lock()
            .unwrap_or_else(|err| panic!(
                "Error while locking the mutex for the CancellationTokenSource token vector: {}",
                err
            ))
            .is_empty());
    }

    #[test]
    fn test_source_initializes_token_as_uncancelled() {
        let mut cancellation_token_source = CancellationTokenSource::new();

        let cancellation_token = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the CancellationToken: {}", err),
        };

        assert!(!cancellation_token
            .is_cancelled()
            .unwrap_or_else(|err| panic!(
                "Error while asserting that the CancellationToken initialized as uncancelled: {}",
                err
            )));
    }

    #[test]
    fn test_source_cancelled_after_cancel_empty() {
        let mut cancellation_token_source: CancellationTokenSource = CancellationTokenSource::new();

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
                    "Error while asserting that the CancellationTokenSource was cancelled: {}",
                    err
                );
            }));
    }

    #[test]
    fn test_source_cancelled_after_cancel_nonempty() {
        let mut cancellation_token_source: CancellationTokenSource = CancellationTokenSource::new();

        let _ = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the CancellationToken: {}", err),
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
                    "Error while asserting that the CancellationTokenSource was cancelled: {}",
                    err
                );
            }));
    }

    #[test]
    fn test_source_cant_be_double_cancelled() {
        let mut cancellation_token_source = CancellationTokenSource::new();

        cancellation_token_source.cancel().unwrap_or_else(|err| {
            panic!(
                "Error while cancelling the CancellationTokenSource the first time: {}",
                err
            )
        });

        let failed = match cancellation_token_source.cancel() {
            Ok(()) => panic!("CancellationTokenSource was cancelled twice successfully"),
            Err(err) => match err {
                CancellationTokenError::AlreadyCancelled => true,
                CancellationTokenError::PoisonError(reason) => panic!(
                    "When cancelling the CancellationTokenSource the second time, it was poinsoned: {}",
                    reason
                ),
            },
        };

        assert!(failed);
    }

    #[test]
    fn test_source_all_tokens_cancelled_after_cancel() {
        let mut cancellation_token_source = CancellationTokenSource::new();

        let cancellation_token_first = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the first CancellationToken: {}", err),
        };

        let cancellation_token_second = match cancellation_token_source.new_token() {
            Ok(cancellation_token) => cancellation_token,
            Err(err) => panic!("Error while creating the second CancellationToken: {}", err),
        };

        cancellation_token_source.cancel().unwrap_or_else(|err| {
            panic!(
                "Error while cancelling the CancellationTokenSource: {}",
                err
            )
        });

        assert!(cancellation_token_first
            .is_cancelled()
            .unwrap_or_else(|err| panic!(
                "Error while asserting that the first CancellationToken was cancelled: {}",
                err
            )));

        assert!(cancellation_token_second
            .is_cancelled()
            .unwrap_or_else(|err| panic!(
                "Error while asserting that the second CancellationToken was cancelled: {}",
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
            "Error while asserting that the first CancellationToken was cancelled: {}",
            err
        )));

        assert!(token_second.is_cancelled().unwrap_or_else(|err| panic!(
            "Error while asserting that the second CancellationToken was cancelled: {}",
            err
        )));
    }
}
