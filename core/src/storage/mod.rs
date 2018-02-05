pub mod db;
mod error;
pub use self::error::Error;

/// A specialized `Result` type for I/O operations with storage.
pub type Result<T> = ::std::result::Result<T, Error>;

