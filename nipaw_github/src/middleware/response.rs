use async_trait::async_trait;
use http::Extensions;
use crate::Error;
use reqwest::{Request, Response, StatusCode};
use reqwest_middleware::{Middleware, Next, Result};
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
		match res.status() {
			StatusCode::OK => Ok(res),
			StatusCode::UNAUTHORIZED => Err(reqwest_middleware::Error::Middleware(Error::Unauthorized.into())),
			StatusCode::NOT_FOUND => {
				let path = res.url().path();
				let resource_type = path
					.split('/')
					.rev()
					.find(|&seg| ["commit", "issue", "release", "user", "repo", "org"]
						.iter()
						.any(|&pattern| seg.contains(pattern)))
					.map(|seg| {
						match seg {
							"commits" | "commit" => "Commit",
							"issues" | "issue" => "Issue",
							"releases" | "release" => "Release",
							s if s.contains("user") => "User",
							s if s.contains("repo") => "Repo",
							s if s.contains("org") => "Org",
							_ => "Resource",
						}
					})
					.unwrap_or("Resource")
					.to_string();
				Err(reqwest_middleware::Error::Middleware(Error::NotFound { resource: resource_type }.into()))
			}
			StatusCode::FORBIDDEN | StatusCode::UNPROCESSABLE_ENTITY => {
				let message = res.json::<ErrorResponse>().await?.message;
				Err(reqwest_middleware::Error::Middleware(Error::Forbidden { message }.into()))
			}

			StatusCode::TOO_MANY_REQUESTS => {
				let is_rate_limited = res
					.headers()
					.get("x-ratelimit-remaining")
					.and_then(|value| value.to_str().ok())
					.map(|value| value == "0")
					.unwrap_or(false);
				if is_rate_limited {
					Err(reqwest_middleware::Error::Middleware(Error::RateLimit.into()))
				} else {
					Ok(res)
				}
			}
			_ => Ok(res),
		}
	}
}
