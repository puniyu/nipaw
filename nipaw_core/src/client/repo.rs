use crate::types::repo::{CollaboratorPermission, CollaboratorResult, RepoInfo, RepoPath};
use async_trait::async_trait;

#[async_trait]
pub trait Repo: Send + Sync {
	/// 获取仓库信息
	///
	/// # 参数
	///
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	///
	async fn info(&self, repo_path: RepoPath<'_>) -> crate::Result<RepoInfo>;

	/// 添加仓库协作者， 如果仓库属于某个组织下的则为外部协作者
	///
	/// # 参数
	///
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `user_name` - 协作者用户名
	/// * `permission` - 协作者权限, 默认为 `Pull`, 可选值为 `Admin`, `Push`, `Pull`
	///
	async fn add_repo_collaborator(
		&self,
		repo_path: RepoPath<'_>,
		user_name: &str,
		permission: Option<CollaboratorPermission>,
	) -> crate::Result<CollaboratorResult>;
}
