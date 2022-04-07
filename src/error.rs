use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Host name inconvertible to UTF-8")]
    InvalidHostName,
}
