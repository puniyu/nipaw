use chrono::{DateTime, Utc};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[napi(object)]
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

#[derive(Debug, Serialize, Deserialize)]
#[napi(object)]
pub struct CommitListInfo {
	/// 提交的SHA
	pub sha: String,
	/// 提交的数据
	pub commit: CommitData,
}

#[derive(Debug, Deserialize, Serialize)]
#[napi(object)]
pub struct CommitData {
	/// 作者信息
	pub author: CommitUserInfo,
	/// 提交者信息
	pub committer: CommitUserInfo,
	/// 提交信息
	pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[napi(object)]
pub struct CommitUserInfo {
	/// 用户名
	pub name: String,
	/// 邮箱
	pub email: Option<String>,
	/// 头像URL
	pub avatar_url: String,
	/// 提交时间
	pub date: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
#[napi(object)]
pub struct StatsInfo {
	/// 总提交行数
	pub total: u32,
	/// 新增的行数
	pub additions: u32,
	/// 删除的行数
	pub deletions: u32,
}

impl From<nipaw_core::types::commit::CommitInfo> for CommitInfo {
	fn from(value: nipaw_core::types::commit::CommitInfo) -> Self {
		Self {
			sha: value.sha,
			commit: value.commit.into(),
			stats: value.stats.into(),
			files: value.files.into_iter().map(|f| f.into()).collect(),
		}
	}
}

impl From<nipaw_core::types::commit::CommitListInfo> for CommitListInfo {
	fn from(value: nipaw_core::types::commit::CommitListInfo) -> Self {
		Self { sha: value.sha, commit: value.commit.into() }
	}
}

impl From<nipaw_core::types::commit::CommitData> for CommitData {
	fn from(value: nipaw_core::types::commit::CommitData) -> Self {
		Self {
			author: value.author.into(),
			committer: value.committer.into(),
			message: value.message,
		}
	}
}

impl From<nipaw_core::types::commit::UserInfo> for CommitUserInfo {
	fn from(value: nipaw_core::types::commit::UserInfo) -> Self {
		Self {
			name: value.name,
			email: value.email,
			avatar_url: value.avatar_url,
			date: value.date,
		}
	}
}

impl From<nipaw_core::types::commit::StatsInfo> for StatsInfo {
	fn from(value: nipaw_core::types::commit::StatsInfo) -> Self {
		Self {
			total: value.total as u32,
			additions: value.additions as u32,
			deletions: value.deletions as u32,
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
#[napi(object)]
pub struct FileInfo {
	/// 文件名
	pub file_name: String,
	/// 文件状态
	pub status: FileStatus,
	/// 新增行数
	pub additions: u32,
	/// 删除行数
	pub deletions: u32,
	/// 修改行数
	pub changes: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[napi(string_enum)]
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

impl From<nipaw_core::types::commit::FileInfo> for FileInfo {
	fn from(value: nipaw_core::types::commit::FileInfo) -> Self {
		Self {
			file_name: value.file_name,
			status: value.status.into(),
			additions: value.additions as u32,
			deletions: value.deletions as u32,
			changes: value.changes as u32,
		}
	}
}

impl From<nipaw_core::types::commit::FileStatus> for FileStatus {
	fn from(value: nipaw_core::types::commit::FileStatus) -> Self {
		match value {
			nipaw_core::types::commit::FileStatus::Added => Self::Added,
			nipaw_core::types::commit::FileStatus::Modified => Self::Modified,
			nipaw_core::types::commit::FileStatus::Deleted => Self::Deleted,
			nipaw_core::types::commit::FileStatus::Renamed => Self::Renamed,
			nipaw_core::types::commit::FileStatus::Copied => Self::Copied,
			nipaw_core::types::commit::FileStatus::Changed => Self::Changed,
			nipaw_core::types::commit::FileStatus::UnChanged => Self::UnChanged,
		}
	}
}
