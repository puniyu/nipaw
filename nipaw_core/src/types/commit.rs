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
	/// 文件差异状态
	pub files: Vec<FileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitListInfo {
	/// 提交的SHA
	pub sha: String,
	/// 提交的数据
	pub commit: CommitData,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
	/// 文件名
	pub file_name: String,
	/// 文件状态
	pub status: FileStatus,
	/// 新增行数
	pub additions: u64,
	/// 删除行数
	pub deletions: u64,
	/// 修改行数
	pub changes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileStatus {
	/// 新增文件
	Added,
	/// 修改文件
	Modified,
	/// 删除文件
	Deleted,
	/// 重命名文件
	Renamed,
	/// 复制文件
	Copied,
	/// 文件已变更
	Changed,
	/// 文件未变更
	UnChanged,
}
