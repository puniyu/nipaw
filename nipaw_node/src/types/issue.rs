use chrono::{DateTime, Utc};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct IssueInfo {
	/// 议题编号
	pub number: String,
	/// 议题状态
	pub state: StateType,
	/// 议题标题
	pub title: String,
	/// 议题内容
	pub body: Option<String>,
	/// 标签信息
	pub labels: Vec<LabelInfo>,
	/// 创建者信息
	pub user: IssueUserInfo,
	/// 议题创建时间
	pub created_at: DateTime<Utc>,
	/// 议题更新时间
	pub updated_at: DateTime<Utc>,
	/// 议题关闭时间
	pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(string_enum)]
pub enum StateType {
	/// 已开启
	Opened,
	/// 已关闭
	Closed,
}

impl From<nipaw_core::types::issue::StateType> for StateType {
	fn from(value: nipaw_core::types::issue::StateType) -> Self {
		match value {
			nipaw_core::types::issue::StateType::Opened => StateType::Opened,
			nipaw_core::types::issue::StateType::Closed => StateType::Closed,
		}
	}
}

impl From<StateType> for nipaw_core::types::issue::StateType {
	fn from(value: StateType) -> Self {
		match value {
			StateType::Opened => nipaw_core::types::issue::StateType::Opened,
			StateType::Closed => nipaw_core::types::issue::StateType::Closed,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct IssueUserInfo {
	/// 用户名
	pub name: String,
	/// 邮箱
	pub email: Option<String>,
	/// 头像URL
	pub avatar_url: String,
}

impl From<nipaw_core::types::issue::UserInfo> for IssueUserInfo {
	fn from(value: nipaw_core::types::issue::UserInfo) -> Self {
		Self { name: value.name, email: value.email, avatar_url: value.avatar_url }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct LabelInfo {
	/// 标签名称
	pub name: String,
	/// 颜色,16进制
	pub color: String,
}

impl From<nipaw_core::types::issue::LabelInfo> for LabelInfo {
	fn from(value: nipaw_core::types::issue::LabelInfo) -> Self {
		Self { name: value.name, color: value.color }
	}
}

impl From<nipaw_core::types::issue::IssueInfo> for IssueInfo {
	fn from(value: nipaw_core::types::issue::IssueInfo) -> Self {
		Self {
			number: value.number,
			state: value.state.into(),
			title: value.title,
			body: value.body,
			labels: value.labels.into_iter().map(Into::into).collect(),
			user: value.user.into(),
			created_at: Default::default(),
			updated_at: Default::default(),
			closed_at: None,
		}
	}
}
