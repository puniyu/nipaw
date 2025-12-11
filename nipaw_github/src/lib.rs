mod commit;
mod common;
mod isssue;
mod middleware;
mod org;
mod repo;
mod user;

pub use nipaw_core::{Client, Commit, Error, Issue, Org, Repo, Result, User};

use crate::{
	commit::GitHubCommit,
	isssue::GitHubIssue,
	middleware::{HeaderMiddleware, ResponseMiddleware},
	org::GitHubOrg,
	repo::GitHubRepo,
	user::GitHubUser,
};
use async_trait::async_trait;
use reqwest::Proxy;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub(crate) struct GitHubConfig {
	pub(crate) token: Option<String>,
	pub(crate) api_url: String,
	pub(crate) base_url: String,
}

#[derive(Debug, Clone)]
pub(crate) struct GitHubClientInner {
	pub(crate) config: GitHubConfig,
	pub(crate) client: Arc<RwLock<Arc<ClientWithMiddleware>>>,
}

impl Default for GitHubConfig {
	fn default() -> Self {
		Self {
			token: None,
			base_url: "https://github.com".to_string(),
			api_url: "https://api.github.com".to_string(),
		}
	}
}

impl GitHubConfig {
	/// 设置访问令牌
	pub fn set_token(&mut self, token: &str) {
		self.token = Some(token.to_string());
	}
	/// 设置 GitHub API 的 URL
	pub fn set_api_url(&mut self, api_url: String) {
		self.api_url = api_url;
	}

	/// 设置 GitHub 基础 URL
	pub fn set_base_url(&mut self, base_url: String) {
		self.base_url = base_url;
	}
}

#[derive(Debug, Clone)]
pub struct GitHubClient {
	pub(crate) inner: Arc<GitHubClientInner>,
}

impl Default for GitHubClient {
	fn default() -> Self {
		let client = reqwest::Client::new();
		Self {
			inner: Arc::new(GitHubClientInner {
				config: GitHubConfig::default(),
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
impl GitHubClient {
	pub fn new() -> Self {
		Self::default()
	}

	/// 设置反向代理
	pub fn set_reverse_proxy(&mut self, url: &str) {
		let clean_url = url.trim_end_matches('/');
		let inner = Arc::make_mut(&mut self.inner);
		let api_url = format!("{}/{}", clean_url, inner.config.api_url);
		let base_url = format!("{}/{}", clean_url, inner.config.base_url);
		inner.config.set_api_url(api_url);
		inner.config.set_base_url(base_url);
	}
}

#[async_trait]
impl Client for GitHubClient {
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
		Box::new(GitHubUser(self.inner.clone()))
	}

	fn org(&self) -> Box<dyn Org> {
		Box::new(GitHubOrg(self.inner.clone()))
	}

	fn repo(&self) -> Box<dyn Repo> {
		Box::new(GitHubRepo(self.inner.clone()))
	}

	fn commit(&self) -> Box<dyn Commit> {
		Box::new(GitHubCommit(self.inner.clone()))
	}

	fn issue(&self) -> Box<dyn Issue> {
		Box::new(GitHubIssue(self.inner.clone()))
	}
}
