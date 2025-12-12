use super::{default_page, default_per_page};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ListOptions {
	/// 每页数量，默认 30，最大 100
	#[serde(default = "default_per_page")]
	pub per_page: Option<u32>,
	/// 页码，默认 1
	#[serde(default = "default_page")]
	pub page: Option<u32>,
	/// 从此提交开始获取，默认最新提交
	pub sha: Option<String>,
	/// 筛选提交作者
	pub author: Option<String>,
	/// 筛选从此时间开始
	pub since: Option<DateTime<Utc>>,
	/// 筛选到此时间结束
	pub until: Option<DateTime<Utc>>,
}
impl Default for ListOptions {
	fn default() -> Self {
		Self {
			per_page: default_per_page(),
			page: default_page(),
			sha: None,
			author: None,
			since: None,
			until: None,
		}
	}
}
