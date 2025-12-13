use crate::option::issue::{CreateOptions, ListOptions, UpdateOptions};
use crate::types::issue::IssueInfo;
use crate::types::repo::RepoPath;
use async_trait::async_trait;

#[async_trait]
pub trait Issue: Send + Sync {
	/// 创建一个issue
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// - `title` - issue标题
	/// - `body` - issue内容
	/// - `option` - 创建issue选项, 详见 [CreateIssueOptions]
	///
	async fn create(
		&self,
		repo_path: RepoPath<'_>,
		title: &str,
		body: Option<&str>,
		option: Option<CreateOptions>,
	) -> crate::Result<IssueInfo>;

	/// 获取issue信息
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// - `issue_number` - issue编号
	///
	async fn info(&self, repo_path: RepoPath<'_>, issue_number: &str) -> crate::Result<IssueInfo>;

	/// 获取仓库所有issue信息
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径，格式为 `(owner, repo)`
	async fn list(
		&self,
		repo_path: RepoPath<'_>,
		options: Option<ListOptions>,
	) -> crate::Result<Vec<IssueInfo>>;

	/// 更新issue信息
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// - `issue_number` - issue编号
	/// - `options` - 更新issue选项, 详见 [UpdateIssueOptions]
	///
	async fn update(
		&self,
		repo_path: RepoPath<'_>,
		issue_number: &str,
		options: Option<UpdateOptions>,
	) -> crate::Result<IssueInfo>;
}
