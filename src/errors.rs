use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum GeneralErrors {
    #[error("Thread error")]
    ThreadError,
    #[error("Length can't be lower than 1")]
    LengthError,
    #[error("Error joining thread")]
    JoinError,
    #[error("Error locking thread")]
    LockError,
}
