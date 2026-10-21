mod commit;
#[doc(inline)]
pub use commit::GitHubCommit;
mod config;
#[doc(inline)]
pub use config::Config;
mod error;
#[doc(inline)]
pub use error::{Error, Result};
mod issue;
#[doc(inline)]
pub use issue::GitHubIssue;
mod middleware;
mod org;
#[doc(inline)]
pub use org::GitHubOrg;
mod release;
#[doc(inline)]
pub use release::GitHubRelease;
mod repo;
#[doc(inline)]
pub use repo::GitHubRepo;
mod user;
#[doc(inline)]
pub use user::GitHubUser;
mod json;
pub(crate) use json::Json;
mod time;
pub(crate) use time::{format_datetime, parse_datetime};
mod html;
pub(crate) use html::Html;

use nipaw_core::Provider;
use parking_lot::RwLock;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct GitHubState {
	pub(crate) config: RwLock<Arc<Config>>,
	pub(crate) http: RwLock<Arc<ClientWithMiddleware>>,
}

impl GitHubState {
	pub fn config(&self) -> Arc<Config> {
		Arc::clone(&self.config.read())
	}

	pub fn http(&self) -> Arc<ClientWithMiddleware> {
		Arc::clone(&self.http.read())
	}
}

#[derive(Debug, Clone)]
pub struct GitHubClient {
	pub(crate) state: Arc<GitHubState>,
	user: GitHubUser,
	org: GitHubOrg,
	repo: GitHubRepo,
	commit: GitHubCommit,
	issue: GitHubIssue,
	release: GitHubRelease,
}

impl Default for GitHubClient {
	fn default() -> Self {
		let state = Arc::new(GitHubState {
			config: RwLock::new(Arc::new(Config::default())),
			http: RwLock::new(Arc::new(
				ClientBuilder::new(reqwest::Client::new())
					.with(middleware::HeaderMiddleware)
					.with(middleware::ResponseMiddleware)
					.build(),
			)),
		});
		Self {
			user: GitHubUser::new(state.clone()),
			org: GitHubOrg::new(state.clone()),
			repo: GitHubRepo::new(state.clone()),
			commit: GitHubCommit::new(state.clone()),
			issue: GitHubIssue::new(state.clone()),
			release: GitHubRelease::new(state.clone()),
			state,
		}
	}
}

impl GitHubClient {
	pub fn new() -> Self {
		Self::default()
	}

	/// 设置访问令牌
	pub fn set_token(&self, token: &str) -> Result<()> {
		if token.is_empty() {
			return Err(Error::TokenEmpty);
		}
		let mut guard = self.state.config.write();
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
		*self.state.http.write() = client;
		Ok(())
	}

	/// 设置反向代理
	pub fn set_reverse_proxy(&self, url: &str) {
		let clean_url = url.trim_end_matches('/');
		let mut guard = self.state.config.write();
		let config = Arc::make_mut(&mut *guard);
		let base_url = format!("{}/{}", clean_url, config.base_url);
		let web_url = format!("{}/{}", clean_url, config.web_url);
		let api_url = format!("{}/{}", clean_url, config.api_url);
		config.set_base_url(base_url);
		config.set_web_url(web_url);
		config.set_api_url(api_url);
	}
}

impl Provider for GitHubClient {
	type Err = Error;
	type User = GitHubUser;
	type Org = GitHubOrg;
	type Repo = GitHubRepo;
	type Commit = GitHubCommit;
	type Issue = GitHubIssue;
	type Release = GitHubRelease;

	fn user(&self) -> &GitHubUser {
		&self.user
	}

	fn org(&self) -> &GitHubOrg {
		&self.org
	}

	fn repo(&self) -> &GitHubRepo {
		&self.repo
	}

	fn commit(&self) -> &GitHubCommit {
		&self.commit
	}

	fn issue(&self) -> &GitHubIssue {
		&self.issue
	}

	fn release(&self) -> &GitHubRelease {
		&self.release
	}
}
