use std::error::Error;

use crate::{
	repo::{RepoInfo, RepoListOptions},
	user::{Contribution, UserInfo},
};
use async_trait::async_trait;

#[async_trait]
pub trait User {
	type Err: Error;
	/// 获取用户信息
	///
	/// 当参数未为[None],则获取当前用户信息
	///
	/// # 参数
	///
	/// * `user_name` - 用户名
	///
	async fn info(&self, user_name: Option<&str>) -> Result<UserInfo, Self::Err>;

	/// 获取用户头像URL
	///
	/// 当参数未为[None],则获取当前用户头像URL
	///
	/// # 参数
	///
	/// * `user_name` - 用户名
	///
	async fn avatar_url(&self, user_name: Option<&str>) -> Result<String, Self::Err>;

	/// 获取指定用户贡献数据
	///
	/// 当参数未为[None],则获取当前用户贡献信息
	///
	/// ## 参数
	///
	/// * `user_name` - 用户名
	///
	async fn contribution(&self, user_name: Option<&str>) -> Result<Contribution, Self::Err>;

	/// 获取用户仓库信息列表
	///
	/// 当参数`user_name`未为[None],则获取当前用户的仓库列表
	///
	/// # 参数
	///
	/// * `user_name` - 用户名
	/// * `option` - 获取仓库列表选项, 详见 [ReposListOptions]
	///
	async fn repo_list(
		&self,
		user_name: Option<&str>,
		option: Option<RepoListOptions>,
	) -> Result<Vec<RepoInfo>, Self::Err>;
}
