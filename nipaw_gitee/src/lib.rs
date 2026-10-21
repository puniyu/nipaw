mod commit;
#[doc(inline)]
pub use commit::GiteeCommit;
mod error;
#[doc(inline)]
pub use error::{Error, Result};
mod html;
pub(crate) use html::Html;
mod issue;
#[doc(inline)]
pub use issue::GiteeIssue;
mod json;
pub(crate) use json::Json;
mod middleware;
mod org;
#[doc(inline)]
pub use org::GiteeOrg;
mod release;
#[doc(inline)]
pub use release::GiteeRelease;
mod repo;
#[doc(inline)]
pub use repo::GiteeRepo;
mod time;
pub(crate) use time::format_datetime;
mod user;
#[doc(inline)]
pub use user::GiteeUser;

pub use nipaw_core::{Commit, Issue, Org, Provider, Release, Repo, User};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub(crate) struct GiteeConfig {
	pub(crate) token: Option<String>,
	pub(crate) base_url: String,
	pub(crate) api_url: String,
}

#[derive(Debug)]
pub(crate) struct GiteeClientInner {
	pub(crate) config: RwLock<Arc<GiteeConfig>>,
	pub(crate) client: RwLock<Arc<ClientWithMiddleware>>,
}

impl GiteeClientInner {
	/// 取出配置快照
	pub(crate) fn config(&self) -> Arc<GiteeConfig> {
		Arc::clone(&self.config.read().unwrap())
	}

	/// 取出 http 客户端快照
	pub(crate) fn http(&self) -> Arc<ClientWithMiddleware> {
		Arc::clone(&self.client.read().unwrap())
	}
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
	user: GiteeUser,
	org: GiteeOrg,
	repo: GiteeRepo,
	commit: GiteeCommit,
	issue: GiteeIssue,
	release: GiteeRelease,
}

impl Default for GiteeClient {
	fn default() -> Self {
		let inner = Arc::new(GiteeClientInner {
			config: RwLock::new(Arc::new(GiteeConfig::default())),
			client: RwLock::new(Arc::new(
				ClientBuilder::new(reqwest::Client::new())
					.with(middleware::HeaderMiddleware)
					.with(middleware::ResponseMiddleware)
					.build(),
			)),
		});
		Self {
			user: GiteeUser(Arc::clone(&inner)),
			org: GiteeOrg(Arc::clone(&inner)),
			repo: GiteeRepo(Arc::clone(&inner)),
			commit: GiteeCommit(Arc::clone(&inner)),
			issue: GiteeIssue(Arc::clone(&inner)),
			release: GiteeRelease(Arc::clone(&inner)),
			inner,
		}
	}
}

impl GiteeClient {
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

impl Provider for GiteeClient {
	type Err = Error;
	type User = GiteeUser;
	type Org = GiteeOrg;
	type Repo = GiteeRepo;
	type Commit = GiteeCommit;
	type Issue = GiteeIssue;
	type Release = GiteeRelease;

	fn user(&self) -> &GiteeUser {
		&self.user
	}

	fn org(&self) -> &GiteeOrg {
		&self.org
	}

	fn repo(&self) -> &GiteeRepo {
		&self.repo
	}

	fn commit(&self) -> &GiteeCommit {
		&self.commit
	}

	fn issue(&self) -> &GiteeIssue {
		&self.issue
	}

	fn release(&self) -> &GiteeRelease {
		&self.release
	}
}
