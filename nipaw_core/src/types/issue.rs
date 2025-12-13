use chrono::{DateTime, Utc};
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
	pub user: UserInfo,
	/// 议题创建时间
	pub created_at: DateTime<Utc>,
	/// 议题更新时间
	pub updated_at: DateTime<Utc>,
	/// 议题关闭时间
	pub closed_at: Option<DateTime<Utc>>,
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
pub struct UserInfo {
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
