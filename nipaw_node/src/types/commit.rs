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
	/// 修改的文件数
	pub change_files: u32,
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
			change_files: value.change_files,
		}
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
