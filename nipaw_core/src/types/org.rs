use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgInfo {
	/// 登录名
	pub login: String,
	/// 组织名
	pub name: Option<String>,
	/// 组织邮箱
	pub email: Option<String>,
	/// 组织头像
	pub avatar_url: String,
	/// 组织描述
	pub description: Option<String>,
	/// 组织关注数
	pub follow_count: u64,
}
