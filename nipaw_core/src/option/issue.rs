use super::{default_page, default_per_page};
use crate::types::issue::StateType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CreateOptions {
	/// 标签
	pub labels: Option<Vec<String>>,
	/// 分配的用户名
	pub assignees: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateOptions {
	/// 标题
	pub title: Option<String>,
	/// 内容
	pub body: Option<String>,
	/// 状态
	pub state: Option<StateType>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ListOptions {
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
