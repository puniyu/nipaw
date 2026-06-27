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

impl_client!(Cnb, nipaw_cnb::CnbClient);
impl_user!(Cnb);
impl_org!(Cnb);
impl_repo!(Cnb);
impl_commit!(Cnb);
impl_release!(Cnb);
impl_issue!(Cnb);
