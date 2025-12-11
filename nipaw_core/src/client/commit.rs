use crate::option::CommitListOptions;
use crate::types::commit::CommitInfo;
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
	async fn info(
		&self,
		repo_path: (&str, &str),
		sha: Option<&str>,
	) -> crate::Result<CommitInfo>;

	/// 获取仓库所有提交信息
	///
	/// # 参数
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `option` - 获取提交列表选项, 详见 [CommitListOptions]
	async fn list(
		&self,
		repo_path: (&str, &str),
		option: Option<CommitListOptions>,
	) -> crate::Result<Vec<CommitInfo>>;
}
