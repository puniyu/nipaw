use chrono::{DateTime, Utc};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[napi(object)]
pub struct CommitListOptions {
	/// 每页数量，默认 30，最大 100
	pub per_page: Option<u32>,
	/// 页码，默认 1
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
impl From<CommitListOptions> for nipaw_core::option::commit::ListOptions {
	fn from(value: CommitListOptions) -> Self {
		nipaw_core::option::commit::ListOptions {
			per_page: value.per_page,
			page: value.page,
			sha: value.sha,
			author: value.author,
			since: value.since,
			until: value.until,
		}
	}
}
