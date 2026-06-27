mod common;
mod error;
mod option;
mod platform;
mod types;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;