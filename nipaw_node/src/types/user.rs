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

impl From<nipaw_core::user::UserInfo> for UserInfo {
	fn from(user_info: nipaw_core::user::UserInfo) -> Self {
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
pub struct ContributionDay {
	pub date: String,
	pub count: u32,
}

impl From<nipaw_core::user::ContributionDay> for ContributionDay {
	fn from(value: nipaw_core::user::ContributionDay) -> Self {
		Self { date: value.date.to_string(), count: value.count }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct ContributionWeek {
	pub days: Vec<ContributionDay>,
}

impl From<nipaw_core::user::ContributionWeek> for ContributionWeek {
	fn from(value: nipaw_core::user::ContributionWeek) -> Self {
		Self { days: value.days.into_iter().map(Into::into).collect() }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct ContributionCalendar {
	pub total: u32,
	pub weeks: Vec<ContributionWeek>,
}

impl From<nipaw_core::user::ContributionCalendar> for ContributionCalendar {
	fn from(value: nipaw_core::user::ContributionCalendar) -> Self {
		Self { total: value.total, weeks: value.weeks.into_iter().map(Into::into).collect() }
	}
}
