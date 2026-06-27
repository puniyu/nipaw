mod client;
pub mod error;

pub use client::{Client, Commit, Config, Issue, Org, Provider, Proxy, Release, Repo, Token, User};
pub use error::Error;
pub mod option;
pub mod types;
pub type Result<T> = std::result::Result<T, Error>;
