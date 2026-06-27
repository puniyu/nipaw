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

impl_client!(Gitee, nipaw_gitee::GiteeClient);
impl_user!(Gitee);
impl_org!(Gitee);
impl_repo!(Gitee);
impl_commit!(Gitee);
impl_release!(Gitee);
impl_issue!(Gitee);
