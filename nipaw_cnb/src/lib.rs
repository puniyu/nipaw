mod commit;
mod common;
mod issue;
mod middleware;
mod org;
mod release;
mod repo;
mod user;

use crate::{
	commit::CnbCommit,
	common::JsonValue,
	issue::CnbIssue,
	middleware::{HeaderMiddleware, ResponseMiddleware},
	org::CnbOrg,
	release::CnbRelease,
	repo::CnbRepo,
	user::CnbUser,
};
use async_trait::async_trait;
use nipaw_core::types::user::UserInfo;
pub use nipaw_core::{Client, Commit, Error, Issue, Org, Release, Repo, Result, User};
use reqwest::Proxy;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub(crate) struct CnbConfig {
	pub(crate) token: Option<String>,
	pub(crate) api_url: String,
	pub(crate) base_url: String,
}

#[derive(Debug, Clone)]
pub(crate) struct CnbClientInner {
	pub(crate) config: CnbConfig,
	pub(crate) client: Arc<RwLock<Arc<ClientWithMiddleware>>>,
}

impl Default for CnbConfig {
	fn default() -> Self {
		Self {
			token: None,
			base_url: "https://cnb.cool".to_string(),
			api_url: "https://api.cnb.cool".to_string(),
		}
	}
}

impl CnbConfig {
	/// 设置访问令牌
	pub fn set_token(&mut self, token: &str) {
		self.token = Some(token.to_string());
	}
}

#[derive(Debug, Clone)]
pub struct CnbClient {
	pub(crate) inner: Arc<CnbClientInner>,
}

impl Default for CnbClient {
	fn default() -> Self {
		let client = reqwest::Client::new();
		Self {
			inner: Arc::new(CnbClientInner {
				config: CnbConfig::default(),
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

impl CnbClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for CnbClient {
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
		Box::new(CnbUser(self.inner.clone()))
	}

	fn org(&self) -> Box<dyn Org> {
		Box::new(CnbOrg(self.inner.clone()))
	}

	fn repo(&self) -> Box<dyn Repo> {
		Box::new(CnbRepo(self.inner.clone()))
	}

	fn commit(&self) -> Box<dyn Commit> {
		Box::new(CnbCommit(self.inner.clone()))
	}

	fn issue(&self) -> Box<dyn Issue> {
		Box::new(CnbIssue(self.inner.clone()))
	}

	fn release(&self) -> Box<dyn Release> {
		Box::new(CnbRelease(self.inner.clone()))
	}
}

pub(crate) async fn get_repo_default_branch(
	client: Arc<ClientWithMiddleware>,
	config: &CnbConfig,
	repo_info: &JsonValue,
	token: Option<String>,
) -> Result<String> {
	let is_public = repo_info
		.0
		.get("visibility_level")
		.and_then(|v| v.as_str())
		.map(|s| s.to_lowercase() == "public")
		.unwrap_or(false);

	let (owner, repo) = (
		repo_info
			.0
			.get("owner")
			.and_then(|v| v.get("login"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string(),
		repo_info.0.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
	);
	if is_public {
		let url = format!(
			"{}/{}/{}/-/git/refs?page=1&page_size=5000&prefix=branch",
			config.base_url, owner, repo
		);

		let request = client.get(url).header("Accept", "application/vnd.cnb.web+json");
		let res = request.send().await?.json::<Vec<Value>>().await?;
		let default_branch = res
			.into_iter()
			.find(|branch| branch.get("is_head").and_then(|v| v.as_bool()).unwrap_or(false))
			.and_then(|branch| branch.get("ref").and_then(|v| v.as_str()).map(|s| s.to_string()))
			.map(|ref_str| ref_str.trim_start_matches("refs/heads/").to_string());
		Ok(default_branch.unwrap())
	} else {
		let url = format!("{}/repos/{}/{}/-/git/head", config.api_url, owner, repo);
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let repo_info: JsonValue = resp.json().await?;
		Ok(repo_info.0.get("name").and_then(|v| v.as_str()).unwrap().to_string())
	}
}

pub(crate) async fn get_user_info(
	client: Arc<ClientWithMiddleware>,
	config: &CnbConfig,
	user_name: &str,
) -> Result<UserInfo> {
	let url = format!("{}/users/{}", config.base_url, user_name);
	let request = client.get(url);
	let resp = request.send().await?.json::<JsonValue>().await?;
	Ok(resp.into())
}
