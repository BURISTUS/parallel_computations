use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum GeneralErrors {
    #[error("Length can't be lower than 1")]
    LengthError,
}
