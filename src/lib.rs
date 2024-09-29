pub mod com;
pub mod connection;
pub mod frame;
pub mod proxy;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
