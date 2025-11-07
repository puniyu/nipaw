use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
	/// 提交的SHA
	pub sha: String,
	/// 提交的数据
	pub commit: CommitData,
	/// 提交统计信息
	pub stats: StatsInfo,
	/// 修改的文件数
	pub change_files: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitData {
	/// 作者信息
	pub author: UserInfo,
	/// 提交者信息
	pub committer: UserInfo,
	/// 提交信息
	pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
	/// 用户名
	pub name: String,
	/// 邮箱
	pub email: Option<String>,
	/// 头像URL
	pub avatar_url: String,
	/// 提交时间
	pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsInfo {
	/// 总提交行数
	pub total: u64,
	/// 新增的行数
	pub additions: u64,
	/// 删除的行数
	pub deletions: u64,
}
