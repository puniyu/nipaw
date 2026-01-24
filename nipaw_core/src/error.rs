use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("set token is empty")]
	TokenEmpty,

	#[error("invalid param: {param} - {reason}")]
	InvalidParam {
		param: String,
		reason: String
	},

	#[error("request error: {0}")]
	RequestError(#[from] reqwest::Error),

	#[error("middleware error: {0}")]
	MiddlewareError(#[from] reqwest_middleware::Error),

	#[error("url parse error: {0}")]
	URLParseError(#[from] url::ParseError),

	#[error("json serialize/deserialize error: {0}")]
	JsonError(#[from] serde_json::Error),

	#[error("not found: {resource}")]
	NotFound {
		resource: String
	},

	#[error("forbidden: {message}")]
	Forbidden {
		message: String
	},

	#[error("unauthorized")]
	Unauthorized,

	#[error("rate limit exceeded")]
	RateLimit,

	#[error("timeout: {duration:?}")]
	Timeout {
		duration: std::time::Duration
	},
}

