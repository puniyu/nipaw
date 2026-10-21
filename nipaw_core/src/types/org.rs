use serde::{Deserialize, Serialize};
use super::{default_page, default_per_page};

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


#[derive(Debug, Deserialize, Serialize)]
pub struct OrgListOptions {
	/// 每页数量，默认 30，最大 100
	#[serde(default = "default_per_page")]
	pub per_page: Option<u32>,
	/// 页码，默认 1
	#[serde(default = "default_page")]
	pub page: Option<u32>,
}

impl Default for OrgListOptions {
	#[inline]
	fn default() -> Self {
		Self { per_page: default_per_page(), page: default_page() }
	}
}
