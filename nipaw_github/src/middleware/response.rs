use async_trait::async_trait;
use http::Extensions;
use nipaw_core::Error as CoreError;
use reqwest::{Request, Response, StatusCode};
use reqwest_middleware::{Error, Middleware, Next, Result};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ErrorResponse {
	status: String,
	message: String,
	documentation_url: String,
}

pub struct ResponseMiddleware;

#[async_trait]
impl Middleware for ResponseMiddleware {
	async fn handle(
		&self,
		req: Request,
		extensions: &mut Extensions,
		next: Next<'_>,
	) -> Result<Response> {
		let res = next.run(req, extensions).await?;
		println!("res: {:#?}", res);
		match res.status() {
			StatusCode::OK => Ok(res),
			StatusCode::UNAUTHORIZED => Err(Error::Middleware(CoreError::Unauthorized.into())),
			StatusCode::NOT_FOUND => Err(Error::Middleware(CoreError::NotFound.into())),
			StatusCode::FORBIDDEN => {
				let message = res.json::<ErrorResponse>().await?.message;
				Err(Error::Middleware(CoreError::Forbidden(message).into()))
			}
			StatusCode::TOO_MANY_REQUESTS => {
				let is_rate_limited = res
					.headers()
					.get("x-ratelimit-remaining")
					.and_then(|value| value.to_str().ok())
					.map(|value| value == "0")
					.unwrap_or(false);
				if is_rate_limited {
					Err(Error::Middleware(CoreError::RateLimit.into()))
				} else {
					Ok(res)
				}
			}
			_ => Ok(res),
		}
	}
}
