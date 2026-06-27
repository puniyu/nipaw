use super::{impl_client, impl_commit, impl_issue, impl_org, impl_release, impl_repo, impl_user};
use crate::{
	option::{
		CommitListOptions, CreateIssueOptions, IssueListOptions, RepoListOptions,
		UpdateIssueOptions, UpdateReleaseOptions,
	},
	types::{
		commit::{CommitInfo, CommitListInfo},
		issue::IssueInfo,
		org::OrgInfo,
		release::ReleaseInfo,
		repo::{CollaboratorPermission, CollaboratorResult, RepoInfo, RepoPath},
		user::{ContributionResult, UserInfo},
	},
};
use napi::tokio::sync::{RwLock, RwLockWriteGuard};
use napi_derive::napi;
use nipaw_core::{Commit, Issue, Org, Provider, Proxy, Release, Repo, Token, User};
use paste::paste;
use std::sync::LazyLock;

use crate::Result;

impl_client!(Github, nipaw_github::GitHubClient);
impl_user!(Github);
impl_org!(Github);
impl_repo!(Github);
impl_commit!(Github);
impl_release!(Github);
impl_issue!(Github);

#[napi]
impl GithubClient {
	#[napi]
	/// 设置反向代理
	///
	/// ## 参数
	/// - `url` - 反向代理地址, 反代地址需要同时支持`github.com`和`api.github.com`
	pub async fn set_reverse_proxy(&self, url: String) {
		let mut client = create_client_github().await;
		client.set_reverse_proxy(url.as_str())
	}
}
