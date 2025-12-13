mod commit;
mod common;
mod issue;
mod middleware;
mod org;
mod release;
mod repo;
mod user;

use crate::{
	commit::GitCodeCommit,
	common::JsonValue,
	issue::GitCodeIssue,
	middleware::{HeaderMiddleware, ResponseMiddleware},
	org::GitCodeOrg,
	release::GitCodeRelease,
	repo::GitCodeRepo,
	user::GitCodeUser,
};
use async_trait::async_trait;
pub use nipaw_core::{Client, Commit, Error, Issue, Org, Release, Repo, Result, User};
use reqwest::Proxy;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub(crate) struct GitCodeConfig {
	pub(crate) token: Option<String>,
	pub(crate) base_url: String,
	pub(crate) api_url: String,
	pub(crate) web_api_url: String,
}

#[derive(Debug, Clone)]
pub(crate) struct GitCodeClientInner {
	pub(crate) config: GitCodeConfig,
	pub(crate) client: Arc<RwLock<Arc<ClientWithMiddleware>>>,
}

impl Default for GitCodeConfig {
	fn default() -> Self {
		Self {
			token: None,
			base_url: "https://gitcode.com".to_string(),
			api_url: "https://api.gitcode.com/api/v5".to_string(),
			web_api_url: "https://web-api.gitcode.com".to_string(),
		}
	}
}

impl GitCodeConfig {
	/// 设置访问令牌
	pub fn set_token(&mut self, token: &str) {
		self.token = Some(token.to_string());
	}
}

#[derive(Debug, Clone)]
pub struct GitCodeClient {
	pub(crate) inner: Arc<GitCodeClientInner>,
}

impl Default for GitCodeClient {
	fn default() -> Self {
		let client = reqwest::Client::new();
		Self {
			inner: Arc::new(GitCodeClientInner {
				config: GitCodeConfig::default(),
				client: Arc::new(RwLock::new(Arc::new(
					ClientBuilder::new(client)
						.with(HeaderMiddleware)
						.with(ResponseMiddleware)
						.build(),
				))),
			}),
		}
	}
}

impl GitCodeClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for GitCodeClient {
	fn set_token(&mut self, token: &str) -> Result<()> {
		if token.is_empty() {
			return Err(Error::TokenEmpty);
		}
		Arc::make_mut(&mut self.inner).config.set_token(token);
		Ok(())
	}

	fn set_proxy(&mut self, proxy: &str) -> Result<()> {
		let client = reqwest::Client::builder().proxy(Proxy::all(proxy)?).build()?;
		*self.inner.client.try_write().unwrap() = Arc::new(
			ClientBuilder::new(client).with(HeaderMiddleware).with(ResponseMiddleware).build(),
		);
		Ok(())
	}

	fn user(&self) -> Box<dyn User> {
		Box::new(GitCodeUser(self.inner.clone()))
	}

	fn org(&self) -> Box<dyn Org> {
		Box::new(GitCodeOrg(self.inner.clone()))
	}

	fn repo(&self) -> Box<dyn Repo> {
		Box::new(GitCodeRepo(self.inner.clone()))
	}

	fn commit(&self) -> Box<dyn Commit> {
		Box::new(GitCodeCommit(self.inner.clone()))
	}

	fn issue(&self) -> Box<dyn Issue> {
		Box::new(GitCodeIssue(self.inner.clone()))
	}

	fn release(&self) -> Box<dyn Release> {
		Box::new(GitCodeRelease(self.inner.clone()))
	}
}

pub(crate) async fn get_user_repo_count(
	client: Arc<ClientWithMiddleware>,
	config: &GitCodeConfig,
	user_name: &str,
) -> Result<u64> {
	let (base_url, web_api_url) = (&config.base_url, &config.web_api_url);
	let url = format!(
		"{}/uc/api/v1/events/{}/contributions?username={}",
		web_api_url, user_name, user_name
	);
	let request = client.get(url).header("Referer", base_url);
	let resp = request.send().await?;
	let repo_info: JsonValue = resp.json().await?;
	let repo_count = repo_info.0.get("total").and_then(|total| total.as_u64()).unwrap_or(0);
	Ok(repo_count)
}
