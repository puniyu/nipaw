mod commit;
#[doc(inline)]
pub use commit::CnbCommit;
mod error;
#[doc(inline)]
pub use error::{Error, Result};
mod issue;
#[doc(inline)]
pub use issue::CnbIssue;
mod json;
pub(crate) use json::Json;
mod middleware;
mod org;
#[doc(inline)]
pub use org::CnbOrg;
mod release;
#[doc(inline)]
pub use release::CnbRelease;
mod repo;
#[doc(inline)]
pub use repo::CnbRepo;
mod time;
pub(crate) use time::format_datetime;
mod user;
#[doc(inline)]
pub use user::CnbUser;

use nipaw_core::user::UserInfo;
pub use nipaw_core::{Commit, Issue, Org, Provider, Release, Repo, User};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde_json::Value;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub(crate) struct CnbConfig {
	pub(crate) token: Option<String>,
	pub(crate) api_url: String,
	pub(crate) base_url: String,
}

#[derive(Debug)]
pub(crate) struct CnbClientInner {
	pub(crate) config: RwLock<Arc<CnbConfig>>,
	pub(crate) client: RwLock<Arc<ClientWithMiddleware>>,
}

impl CnbClientInner {
	/// 取出配置快照
	pub(crate) fn config(&self) -> Arc<CnbConfig> {
		Arc::clone(&self.config.read().unwrap())
	}

	/// 取出 http 客户端快照
	pub(crate) fn http(&self) -> Arc<ClientWithMiddleware> {
		Arc::clone(&self.client.read().unwrap())
	}
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
	user: CnbUser,
	org: CnbOrg,
	repo: CnbRepo,
	commit: CnbCommit,
	issue: CnbIssue,
	release: CnbRelease,
}

impl Default for CnbClient {
	fn default() -> Self {
		let inner = Arc::new(CnbClientInner {
			config: RwLock::new(Arc::new(CnbConfig::default())),
			client: RwLock::new(Arc::new(
				ClientBuilder::new(reqwest::Client::new())
					.with(middleware::HeaderMiddleware)
					.with(middleware::ResponseMiddleware)
					.build(),
			)),
		});
		Self {
			user: CnbUser(Arc::clone(&inner)),
			org: CnbOrg(Arc::clone(&inner)),
			repo: CnbRepo(Arc::clone(&inner)),
			commit: CnbCommit(Arc::clone(&inner)),
			issue: CnbIssue(Arc::clone(&inner)),
			release: CnbRelease(Arc::clone(&inner)),
			inner,
		}
	}
}

impl CnbClient {
	pub fn new() -> Self {
		Self::default()
	}

	/// 设置访问令牌
	pub fn set_token(&self, token: &str) -> Result<()> {
		if token.is_empty() {
			return Err(Error::TokenEmpty);
		}
		let mut guard = self.inner.config.write().unwrap();
		Arc::make_mut(&mut *guard).set_token(token);
		Ok(())
	}

	/// 设置代理
	pub fn set_proxy(&self, proxy: &str) -> Result<()> {
		let client = reqwest::Client::builder().proxy(reqwest::Proxy::all(proxy)?).build()?;
		let client = Arc::new(
			ClientBuilder::new(client)
				.with(middleware::HeaderMiddleware)
				.with(middleware::ResponseMiddleware)
				.build(),
		);
		*self.inner.client.write().unwrap() = client;
		Ok(())
	}
}

impl Provider for CnbClient {
	type Err = Error;
	type User = CnbUser;
	type Org = CnbOrg;
	type Repo = CnbRepo;
	type Commit = CnbCommit;
	type Issue = CnbIssue;
	type Release = CnbRelease;

	fn user(&self) -> &CnbUser {
		&self.user
	}

	fn org(&self) -> &CnbOrg {
		&self.org
	}

	fn repo(&self) -> &CnbRepo {
		&self.repo
	}

	fn commit(&self) -> &CnbCommit {
		&self.commit
	}

	fn issue(&self) -> &CnbIssue {
		&self.issue
	}

	fn release(&self) -> &CnbRelease {
		&self.release
	}
}

pub(crate) async fn get_repo_default_branch(
	client: Arc<ClientWithMiddleware>,
	config: &CnbConfig,
	repo_info: &Json<'_>,
	token: Option<String>,
) -> Result<String> {
	let is_public = repo_info
		.as_inner()
		.get("visibility_level")
		.and_then(|v| v.as_str())
		.map(|s| s.to_lowercase() == "public")
		.unwrap_or(false);

	let (owner, repo) = (
		repo_info
			.as_inner()
			.get("owner")
			.and_then(|v| v.get("login"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string(),
		repo_info.as_inner().get("name").and_then(|v| v.as_str()).unwrap().to_string(),
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
		let repo_info: Json = resp.json().await?;
		Ok(repo_info.as_inner().get("name").and_then(|v| v.as_str()).unwrap().to_string())
	}
}

pub(crate) async fn get_user_info(
	client: Arc<ClientWithMiddleware>,
	config: &CnbConfig,
	user_name: &str,
) -> Result<UserInfo> {
	let url = format!("{}/users/{}", config.base_url, user_name);
	let request = client.get(url);
	let resp = request.send().await?.json::<Json>().await?;
	Ok(resp.into())
}
