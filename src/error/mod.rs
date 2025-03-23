use thiserror::Error;
use networks::libp2p::DialError;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync >>;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Dial error: {0}")]
    Dial(#[from] DialError),
    // Add other error variants as needed
}

#[derive(Error, Debug)]
#[error("{0:?}")]
pub struct MyDialError(DialError);
