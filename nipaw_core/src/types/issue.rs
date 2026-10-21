use super::{default_page, default_per_page};
use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
	pub created_at: DateTime,
	/// 议题更新时间
	pub updated_at: DateTime,
	/// 议题关闭时间
	pub closed_at: Option<DateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, IntoStaticStr)]
pub enum StateType {
	#[strum(serialize = "opened")]
	/// 以打开
	Opened,
	#[strum(serialize = "closed")]
	/// 已关闭
	Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueUserInfo {
	/// 用户名
	pub login: String,
	/// 邮箱
	pub email: Option<String>,
	/// 头像URL
	pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelInfo {
	/// 标签名称
	pub name: String,
	/// 颜色,16进制
	pub color: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct IssueCreateOptions {
	/// 标签
	pub labels: Option<Vec<String>>,
	/// 分配的用户名
	pub assignees: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct IssueUpdateOptions {
	/// 标题
	pub title: Option<String>,
	/// 内容
	pub body: Option<String>,
	/// 状态
	pub state: Option<StateType>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct IssueListOptions {
	/// 每页数量，默认 30，最大 100
	#[serde(default = "default_per_page")]
	pub per_page: Option<u32>,
	/// 页码，默认 1
	#[serde(default = "default_page")]
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
