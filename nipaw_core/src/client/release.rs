use crate::Result;
use crate::option::release::UpdateOption;
use crate::types::release::ReleaseInfo;
use crate::types::repo::RepoPath;
use async_trait::async_trait;

#[async_trait]
pub trait Release: Send + Sync {
	/// 创建一个Release
	///
	/// 当为传入参数为[None]则使用tag_name
	///
	/// ## 参数
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `tag_name` - 标签名称
	/// * `name` - Release名称
	/// * `body` - Release内容
	/// * `target_commitish` - 目标提交SHA或分支
	///
	async fn create(
		&self,
		repo_path: RepoPath<'_>,
		tag_name: &str,
		name: Option<&str>,
		body: Option<&str>,
		target_commitish: Option<&str>,
	) -> Result<ReleaseInfo>;

	/// 获取Release信息
	///
	/// 当不传入参数时，获取最新Release信息
	///
	/// ## 参数
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `tag_name` - 标签名称
	///
	async fn info(&self, repo_path: RepoPath<'_>, tag_name: Option<&str>) -> Result<ReleaseInfo>;

	/// 获取Release列表
	///
	/// ## 参数
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	///
	async fn list(&self, repo_path: RepoPath<'_>) -> Result<Vec<ReleaseInfo>>;

	/// 更新Release
	///
	/// ## 参数
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `tag_name` - 标签名称
	/// * `option` - 更新参数，参考[UpdateOption]
	///
	async fn update(
		&self,
		repo_path: RepoPath<'_>,
		tag_name: &str,
		option: UpdateOption,
	) -> Result<ReleaseInfo>;
}
