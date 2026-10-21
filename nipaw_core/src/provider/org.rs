use std::error::Error;

use crate::{
	org::{OrgInfo, OrgListOptions},
	repo::RepoInfo,
};
use async_trait::async_trait;

#[async_trait]
pub trait Org {
	type Err: Error;
	/// 获取组织信息
	///
	/// # 参数
	///
	/// * `org_name` - 组织名
	///
	async fn info(&self, org_name: &str) -> Result<OrgInfo, Self::Err>;

	/// 获取组织仓库信息列表
	/// # 参数
	///
	/// * `org_name` - 组织名
	/// * `options` - 获取仓库列表选项, 详见 [OrgListOptions]
	async fn repo_list(
		&self,
		org_name: &str,
		options: Option<OrgListOptions>,
	) -> Result<Vec<RepoInfo>, Self::Err>;

	/// 获取组织头像URL
	///
	/// # 参数
	///
	/// * `org_name` - 组织名
	///
	async fn avatar_url(&self, org_name: &str) -> Result<String, Self::Err>;
}
