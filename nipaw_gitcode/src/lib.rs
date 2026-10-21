mod commit;
#[doc(inline)]
pub use commit::GitCodeCommit;
mod error;
#[doc(inline)]
pub use error::{Error, Result};
mod issue;
#[doc(inline)]
pub use issue::GitCodeIssue;
mod json;
pub(crate) use json::Json;
mod middleware;
mod org;
#[doc(inline)]
pub use org::GitCodeOrg;
mod release;
#[doc(inline)]
pub use release::GitCodeRelease;
mod repo;
#[doc(inline)]
pub use repo::GitCodeRepo;
mod time;
pub(crate) use time::format_datetime;
mod user;
#[doc(inline)]
pub use user::GitCodeUser;

pub use nipaw_core::{Commit, Issue, Org, Provider, Release, Repo, User};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub(crate) struct GitCodeConfig {
	pub(crate) token: Option<String>,
	pub(crate) base_url: String,
	pub(crate) api_url: String,
	pub(crate) web_api_url: String,
}

#[derive(Debug)]
pub(crate) struct GitCodeClientInner {
	pub(crate) config: RwLock<Arc<GitCodeConfig>>,
	pub(crate) client: RwLock<Arc<ClientWithMiddleware>>,
}

impl GitCodeClientInner {
	/// 取出配置快照
	pub(crate) fn config(&self) -> Arc<GitCodeConfig> {
		Arc::clone(&self.config.read().unwrap())
	}

	/// 取出 http 客户端快照
	pub(crate) fn http(&self) -> Arc<ClientWithMiddleware> {
		Arc::clone(&self.client.read().unwrap())
	}
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
	user: GitCodeUser,
	org: GitCodeOrg,
	repo: GitCodeRepo,
	commit: GitCodeCommit,
	issue: GitCodeIssue,
	release: GitCodeRelease,
}

impl Default for GitCodeClient {
	fn default() -> Self {
		let inner = Arc::new(GitCodeClientInner {
			config: RwLock::new(Arc::new(GitCodeConfig::default())),
			client: RwLock::new(Arc::new(
				ClientBuilder::new(reqwest::Client::new())
					.with(middleware::HeaderMiddleware)
					.with(middleware::ResponseMiddleware)
					.build(),
			)),
		});
		Self {
			user: GitCodeUser(Arc::clone(&inner)),
			org: GitCodeOrg(Arc::clone(&inner)),
			repo: GitCodeRepo(Arc::clone(&inner)),
			commit: GitCodeCommit(Arc::clone(&inner)),
			issue: GitCodeIssue(Arc::clone(&inner)),
			release: GitCodeRelease(Arc::clone(&inner)),
			inner,
		}
	}
}

impl GitCodeClient {
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

impl Provider for GitCodeClient {
	type Err = Error;
	type User = GitCodeUser;
	type Org = GitCodeOrg;
	type Repo = GitCodeRepo;
	type Commit = GitCodeCommit;
	type Issue = GitCodeIssue;
	type Release = GitCodeRelease;

	fn user(&self) -> &GitCodeUser {
		&self.user
	}

	fn org(&self) -> &GitCodeOrg {
		&self.org
	}

	fn repo(&self) -> &GitCodeRepo {
		&self.repo
	}

	fn commit(&self) -> &GitCodeCommit {
		&self.commit
	}

	fn issue(&self) -> &GitCodeIssue {
		&self.issue
	}

	fn release(&self) -> &GitCodeRelease {
		&self.release
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
	let repo_info: Json = resp.json().await?;
	let repo_count =
		repo_info.as_inner().get("total").and_then(|total| total.as_u64()).unwrap_or(0);
	Ok(repo_count)
}
