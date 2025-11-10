use chrono::{DateTime, Utc};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[napi(object)]
pub struct UserInfo {
	/// 登录用户名
	pub login: String,
	/// 用户昵称
	pub name: Option<String>,
	/// 用户邮箱
	pub email: Option<String>,
	/// 用户头像URL
	pub avatar_url: String,
	/// 关注者数量
	pub followers: u32,
	/// 关注的用户数量
	pub following: u32,
	/// 公开仓库数量
	pub public_repo_count: u32,
}

impl From<nipaw_core::types::user::UserInfo> for UserInfo {
	fn from(user_info: nipaw_core::types::user::UserInfo) -> Self {
		Self {
			login: user_info.login,
			name: user_info.name,
			avatar_url: user_info.avatar_url,
			email: user_info.email,
			followers: user_info.followers as u32,
			following: user_info.following as u32,
			public_repo_count: user_info.public_repo_count as u32,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct ContributionData {
	/// 日期
	pub date: DateTime<Utc>,

	/// 当日的贡献次数
	pub count: u32,
}

impl From<nipaw_core::types::user::ContributionData> for ContributionData {
	fn from(value: nipaw_core::types::user::ContributionData) -> Self {
		ContributionData { date: value.date, count: value.count }
	}
}

/// 整体贡献统计结果，包含总数与按周组织的贡献数据
#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct ContributionResult {
	/// 总贡献次数
	pub total: u32,

	/// 二维数组结构的贡献数据
	/// 第一维通常表示周数，第二维表示每周的贡献数据
	pub contributions: Vec<Vec<ContributionData>>,
}

impl From<nipaw_core::types::user::ContributionResult> for ContributionResult {
	fn from(value: nipaw_core::types::user::ContributionResult) -> Self {
		ContributionResult {
			total: value.total,
			contributions: value
				.contributions
				.into_iter()
				.map(|week| week.into_iter().map(Into::into).collect())
				.collect(),
		}
	}
}
