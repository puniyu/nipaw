use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("set token is empty")]
	TokenEmpty,

	#[error("request error: {0}")]
	Request(#[from] reqwest::Error),

	#[error("middleware error: {0}")]
	Middleware(#[from] reqwest_middleware::Error),

	#[error("json serialize/deserialize error: {0}")]
	Json(#[from] serde_json::Error),

	#[error("not found: {resource}")]
	NotFound { resource: String },

	#[error("forbidden: {message}")]
	Forbidden { message: String },

	#[error("unauthorized")]
	Unauthorized,

	#[error("rate limit exceeded")]
	RateLimit,
}

pub type Result<T> = std::result::Result<T, Error>;
