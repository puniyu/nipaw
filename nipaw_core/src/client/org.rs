use crate::option::repo::ListOptions;
use crate::types::org::OrgInfo;
use crate::types::repo::RepoInfo;
use async_trait::async_trait;

#[async_trait]
pub trait Org: Send + Sync {
	/// 获取组织信息
	///
	/// # 参数
	///
	/// * `org_name` - 组织名
	///
	async fn info(&self, org_name: &str) -> crate::Result<OrgInfo>;

	/// 获取组织仓库信息列表
	/// # 参数
	///
	/// * `org_name` - 组织名
	/// * `options` - 获取仓库列表选项, 详见 [OrgRepoListOptions]
	async fn repo_list(
		&self,
		org_name: &str,
		options: Option<ListOptions>,
	) -> crate::Result<Vec<RepoInfo>>;

	/// 获取组织头像URL
	///
	/// # 参数
	///
	/// * `org_name` - 组织名
	///
	async fn avatar_url(&self, org_name: &str) -> crate::Result<String>;
}
