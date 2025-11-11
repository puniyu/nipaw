use crate::types::issue::StateType;
use chrono::{DateTime, Utc};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[napi(object)]
pub struct ReposListOptions {
	/// 每页数量，默认 30，最大 100
	pub per_page: Option<u32>,
	/// 页码，默认 1
	pub page: Option<u32>,
}

impl From<ReposListOptions> for nipaw_core::option::ReposListOptions {
	fn from(value: ReposListOptions) -> Self {
		nipaw_core::option::ReposListOptions { per_page: value.per_page, page: value.page }
	}
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[napi(object)]
pub struct CommitListOptions {
	/// 每页数量，默认 30，最大 100
	pub per_page: Option<u32>,
	/// 页码，默认 1
	pub page: Option<u32>,
	/// 从此提交开始获取，默认最新提交
	pub sha: Option<String>,
	/// 筛选提交作者
	pub author: Option<String>,
	/// 筛选从此时间开始
	pub since: Option<DateTime<Utc>>,
	/// 筛选到此时间结束
	pub until: Option<DateTime<Utc>>,
}
impl From<CommitListOptions> for nipaw_core::option::CommitListOptions {
	fn from(value: CommitListOptions) -> Self {
		nipaw_core::option::CommitListOptions {
			per_page: value.per_page,
			page: value.page,
			sha: value.sha,
			author: value.author,
			since: value.since,
			until: value.until,
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[napi(object)]
pub struct OrgRepoListOptions {
	/// 每页数量，默认 30，最大 100
	pub per_page: Option<u32>,
	/// 页码，默认 1
	pub page: Option<u32>,
}

impl From<OrgRepoListOptions> for nipaw_core::option::OrgRepoListOptions {
	fn from(value: OrgRepoListOptions) -> Self {
		nipaw_core::option::OrgRepoListOptions { per_page: value.per_page, page: value.page }
	}
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[napi(object)]
pub struct CreateIssueOptions {
	/// 标签
	pub labels: Option<Vec<String>>,
	/// 分配的用户名
	pub assignees: Option<Vec<String>>,
}

impl From<CreateIssueOptions> for nipaw_core::option::CreateIssueOptions {
	fn from(value: CreateIssueOptions) -> Self {
		nipaw_core::option::CreateIssueOptions { labels: value.labels, assignees: value.assignees }
	}
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[napi(object)]
pub struct IssueListOptions {
	/// 每页数量，默认 30，最大 100
	pub per_page: Option<u32>,
	/// 页码，默认 1
	pub page: Option<u32>,
	/// 标签
	pub labels: Option<Vec<String>>,
	/// 创建者
	pub creator: Option<String>,
	/// 分配的用户名
	pub assignee: Option<String>,
	/// 状态
	pub state: Option<StateType>,
}
impl From<IssueListOptions> for nipaw_core::option::IssueListOptions {
	fn from(value: IssueListOptions) -> Self {
		nipaw_core::option::IssueListOptions {
			per_page: value.per_page,
			page: value.page,
			labels: value.labels,
			creator: value.creator,
			assignee: value.assignee,
			state: value.state.map(|s| s.into()),
		}
	}
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[napi(object)]
pub struct UpdateIssueOptions {
	/// 标题
	pub title: Option<String>,
	/// 内容
	pub body: Option<String>,
	/// 状态
	pub state: Option<StateType>,
}

impl From<UpdateIssueOptions> for nipaw_core::option::UpdateIssueOptions {
	fn from(value: UpdateIssueOptions) -> Self {
		nipaw_core::option::UpdateIssueOptions {
			title: value.title,
			body: value.body,
			state: value.state.map(|s| s.into()),
		}
	}
}
