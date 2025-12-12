use crate::types::issue::StateType;
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[napi(object)]
pub struct CreateIssueOptions {
	/// 标签
	pub labels: Option<Vec<String>>,
	/// 分配的用户名
	pub assignees: Option<Vec<String>>,
}

impl From<CreateIssueOptions> for nipaw_core::option::issue::CreateOptions {
	fn from(value: CreateIssueOptions) -> Self {
		nipaw_core::option::issue::CreateOptions {
			labels: value.labels,
			assignees: value.assignees,
		}
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
impl From<IssueListOptions> for nipaw_core::option::issue::ListOptions {
	fn from(value: IssueListOptions) -> Self {
		nipaw_core::option::issue::ListOptions {
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

impl From<UpdateIssueOptions> for nipaw_core::option::issue::UpdateOptions {
	fn from(value: UpdateIssueOptions) -> Self {
		nipaw_core::option::issue::UpdateOptions {
			title: value.title,
			body: value.body,
			state: value.state.map(|s| s.into()),
		}
	}
}
