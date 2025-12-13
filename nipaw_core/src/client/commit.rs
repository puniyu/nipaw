use crate::option::commit::ListOptions;
use crate::types::commit::CommitInfo;
use crate::types::repo::RepoPath;
use async_trait::async_trait;

#[async_trait]
pub trait Commit: Send + Sync {
	/// 获取仓库提交信息
	///
	/// # 参数
	///
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `sha` - 提交ID, 默认为最新提交
	///
	async fn info(&self, repo_path: RepoPath<'_>, sha: Option<&str>) -> crate::Result<CommitInfo>;

	/// 获取仓库所有提交信息
	///
	/// # 参数
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `option` - 获取提交列表选项, 详见 [CommitListOptions]
	async fn list(
		&self,
		repo_path: RepoPath<'_>,
		option: Option<ListOptions>,
	) -> crate::Result<Vec<CommitInfo>>;
}
