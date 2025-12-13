mod client;
pub mod error;

pub use client::{Client, Commit, Issue, Org, Release, Repo, User};
pub use error::Error;
pub mod option;
pub mod types;
pub type Result<T> = std::result::Result<T, Error>;
