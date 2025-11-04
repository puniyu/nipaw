use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ReposListOptions {
	/// 每页数量，默认 30，最大 100
	#[serde(default = "default_per_page")]
	pub per_page: Option<u32>,
	/// 页码，默认 1
	#[serde(default = "default_page")]
	pub page: Option<u32>,
}

fn default_per_page() -> Option<u32> {
	Some(30)
}

fn default_page() -> Option<u32> {
	Some(1)
}

impl Default for ReposListOptions {
	fn default() -> Self {
		Self { per_page: default_per_page(), page: default_page() }
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitListOptions {
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
impl Default for CommitListOptions {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct OrgRepoListOptions {
	/// 每页数量，默认 30，最大 100
	#[serde(default = "default_per_page")]
	pub per_page: Option<u32>,
	/// 页码，默认 1
	#[serde(default = "default_page")]
	pub page: Option<u32>,
}

impl Default for OrgRepoListOptions {
	fn default() -> Self {
		Self { per_page: default_per_page(), page: default_page() }
	}
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CreateIssueOptions {
	/// 标签
	pub labels: Vec<String>,
	/// 分配的用户名
	pub assignees: Vec<String>,
}
