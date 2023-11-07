use super::error::CancellationTokenError;
use std::sync::Mutex;

#[derive(Debug)]
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
            Err(err) => Err(CancellationTokenError::PoisonError(err.to_string())),
        }
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for CancellationToken {}
unsafe impl Sync for CancellationToken {}

#[cfg(test)]
mod tests {

    use super::super::error::CancellationTokenError;
    use crate::common::threading::cancellation_token::CancellationToken;

    #[test]
    fn test_token_initializes_uncancelled() {
        let cancellation_token = CancellationToken::new();

        assert!(!cancellation_token
            .is_cancelled()
            .unwrap_or_else(|err| panic!(
                "Error while asserting that the CancellationToken initializes as uncancelled: {}",
                err
            )));
    }

    #[test]
    fn test_token_cancelled_after_cancel() {
        let cancellation_token = CancellationToken::new();

        cancellation_token
            .cancel()
            .unwrap_or_else(|err| panic!("Error while cancelling the CancellationToken: {}", err));

        assert!(cancellation_token.is_cancelled().unwrap_or_else(|err| {
            panic!(
                "Error while asserting that the CancellationToken was cancelled: {}",
                err
            );
        }));
    }

    #[test]
    fn test_token_cant_be_double_cancelled() {
        let cancellation_token = CancellationToken::new();

        cancellation_token.cancel().unwrap_or_else(|err| {
            panic!(
                "Error while cancelling the CancellationToken the first time: {}",
                err
            )
        });

        let failed = match cancellation_token.cancel() {
            Ok(()) => panic!("CancellationToken was cancelled twice successfully"),
            Err(err) => match err {
                CancellationTokenError::AlreadyCancelled => true,
                CancellationTokenError::PoisonError(reason) => panic!(
                    "When cancelling the CancellationToken the second time, it was poinsoned: {}",
                    reason
                ),
            },
        };

        assert!(failed);
    }
}
