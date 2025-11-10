use async_trait::async_trait;
use http::{Extensions, HeaderName, HeaderValue};
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};

pub struct HeaderMiddleware;

#[async_trait]
impl Middleware for HeaderMiddleware {
	async fn handle(
		&self,
		mut req: Request,
		extensions: &mut Extensions,
		next: Next<'_>,
	) -> reqwest_middleware::Result<Response> {
		req.headers_mut().insert(
			HeaderName::from_static("accept"),
			HeaderValue::from_static("application/json"),
		);
		req.headers_mut()
			.insert(HeaderName::from_static("user-agent"), HeaderValue::from_static("nipaw"));
		Ok(next.run(req, extensions).await?)
	}
}
