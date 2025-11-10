use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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
	pub followers: u64,
	/// 关注的用户数量
	pub following: u64,
	/// 公开仓库数量
	pub public_repo_count: u64,
}

/// 单日贡献数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionData {
	/// 日期
	pub date: DateTime<Utc>,

	/// 当日的贡献次数
	pub count: u32,
}

/// 整体贡献统计结果，包含总数与按周组织的贡献数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionResult {
	/// 总贡献次数
	pub total: u32,

	/// 二维数组结构的贡献数据
	/// 第一维通常表示周数，第二维表示每周的贡献数据
	pub contributions: Vec<Vec<ContributionData>>,
}
