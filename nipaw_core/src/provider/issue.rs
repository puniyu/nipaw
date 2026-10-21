use std::error::Error;

use crate::{
	issue::{IssueCreateOptions, IssueInfo, IssueListOptions, IssueUpdateOptions},
	repo::RepoPath,
};
use async_trait::async_trait;

#[async_trait]
pub trait Issue {
	type Err: Error;

	/// 创建一个issue
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径
	/// - `title` - issue标题
	/// - `body` - issue内容
	/// - `option` - 创建issue选项, 详见 [CreateIssueOptions]
	///
	async fn create(
		&self,
		repo_path: RepoPath,
		title: &str,
		body: Option<&str>,
		option: Option<IssueCreateOptions>,
	) -> Result<IssueInfo, Self::Err>;

	/// 获取issue信息
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径
	/// - `issue_number` - issue编号
	///
	async fn info(&self, repo_path: RepoPath, issue_number: &str) -> Result<IssueInfo, Self::Err>;

	/// 获取仓库所有issue信息
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径
	async fn list(
		&self,
		repo_path: RepoPath,
		options: Option<IssueListOptions>,
	) -> Result<Vec<IssueInfo>, Self::Err>;

	/// 更新issue信息
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径
	/// - `issue_number` - issue编号
	/// - `options` - 更新issue选项, 详见 [UpdateIssueOptions]
	///
	async fn update(
		&self,
		repo_path: RepoPath,
		issue_number: &str,
		options: Option<IssueUpdateOptions>,
	) -> Result<IssueInfo, Self::Err>;
}
