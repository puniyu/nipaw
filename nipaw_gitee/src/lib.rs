mod commit;
mod common;
mod issue;
mod middleware;
mod org;
mod repo;
mod user;

pub use nipaw_core::{Client, Commit, Error, Issue, Org, Repo, Result, User};

use crate::{
	commit::GiteeCommit,
	issue::GiteeIssue,
	middleware::{HeaderMiddleware, ResponseMiddleware},
	org::GiteeOrg,
	repo::GiteeRepo,
	user::GiteeUser,
};
use async_trait::async_trait;
use reqwest::Proxy;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub(crate) struct GiteeConfig {
	pub(crate) token: Option<String>,
	pub(crate) base_url: String,
	pub(crate) api_url: String,
}

#[derive(Debug, Clone)]
pub(crate) struct GiteeClientInner {
	pub(crate) config: GiteeConfig,
	pub(crate) client: Arc<RwLock<Arc<ClientWithMiddleware>>>,
}

impl Default for GiteeConfig {
	fn default() -> Self {
		Self {
			token: None,
			base_url: "https://gitee.com".to_string(),
			api_url: "https://gitee.com/api/v5".to_string(),
		}
	}
}

impl GiteeConfig {
	/// 设置访问令牌
	pub fn set_token(&mut self, token: &str) {
		self.token = Some(token.to_string());
	}
}

#[derive(Debug, Clone)]
pub struct GiteeClient {
	pub(crate) inner: Arc<GiteeClientInner>,
}

impl Default for GiteeClient {
	fn default() -> Self {
		let client = reqwest::Client::new();
		Self {
			inner: Arc::new(GiteeClientInner {
				config: GiteeConfig::default(),
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

impl GiteeClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for GiteeClient {
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
		Box::new(GiteeUser(self.inner.clone()))
	}

	fn org(&self) -> Box<dyn Org> {
		Box::new(GiteeOrg(self.inner.clone()))
	}

	fn repo(&self) -> Box<dyn Repo> {
		Box::new(GiteeRepo(self.inner.clone()))
	}

	fn commit(&self) -> Box<dyn Commit> {
		Box::new(GiteeCommit(self.inner.clone()))
	}

	fn issue(&self) -> Box<dyn Issue> {
		Box::new(GiteeIssue(self.inner.clone()))
	}
}
