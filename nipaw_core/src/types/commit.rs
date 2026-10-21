use jiff::civil::DateTime;
use serde::{Deserialize, Serialize};
use super::{default_page, default_per_page};

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
	pub author: GitUserInfo,
	/// 提交者信息
	pub committer: GitUserInfo,
	/// 提交信息
	pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitUserInfo {
	/// 用户名
	pub name: String,
	/// 邮箱
	pub email: Option<String>,
	/// 头像URL
	pub avatar_url: String,
	/// 提交时间
	pub date: DateTime,
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
	/// 未知
	UnKnown,
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
	pub since: Option<DateTime>,
	/// 筛选到此时间结束
	pub until: Option<DateTime>,
}
impl Default for CommitListOptions {
	#[inline]
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
