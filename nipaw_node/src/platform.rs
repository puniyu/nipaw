use crate::{
	common::RT_RUNTIME,
	error,
	option::{
		CommitListOptions, CreateIssueOptions, IssueListOptions, RepoListOptions,
		UpdateIssueOptions,
	},
	types::{
		commit::CommitInfo,
		issue::IssueInfo,
		org::OrgInfo,
		repo::{CollaboratorPermission, CollaboratorResult, RepoInfo},
		user::{ContributionResult, UserInfo},
	},
};
use napi::tokio::sync::{RwLock, RwLockWriteGuard};
use napi_derive::napi;
use nipaw_core::Client;
use paste::paste;
use std::sync::LazyLock;

type Result<T> = std::result::Result<T, error::Error>;

macro_rules! impl_client {
	($client_type:ident, $client:ty) => {
		paste! {
			static [<$client_type:upper _CLIENT>]: LazyLock<RwLock<$client>> =
				LazyLock::new(|| RwLock::new(<$client>::default()));

			async fn [<create_client_ $client_type:lower>]() -> RwLockWriteGuard<'static, $client> {
				[<$client_type:upper _CLIENT>].write().await
			}

			// User 子模块
			#[derive(Debug, Default)]
			#[napi(constructor)]
			pub struct [<$client_type User>];

			#[napi]
			impl [<$client_type User>] {
				/// 获取用户信息
				///
				/// ## 参数
				/// - `user_name` 用户名称, 为空时获取当前登录用户信息
				#[napi]
				pub async fn info(&self, user_name: Option<String>) -> Result<UserInfo> {
					let client = [<create_client_ $client_type:lower>]().await;
					let user_info = client.user().info(user_name.as_deref()).await?;
					Ok(user_info.into())
				}

				/// 获取用户头像地址
				///
				/// ## 参数
				/// - `user_name` 用户名称, 为空时获取当前登录用户头像
				#[napi]
				pub async fn avatar_url(&self, user_name: Option<String>) -> Result<String> {
					let client = [<create_client_ $client_type:lower>]().await;
					let avatar_url = client.user().avatar_url(user_name.as_deref()).await?;
					Ok(avatar_url)
				}

				/// 获取用户贡献信息
				///
				/// ## 参数
				/// - `user_name` 用户名称, 为空时获取当前登录用户贡献信息
				#[napi]
				pub async fn contribution(&self, user_name: Option<String>) -> Result<ContributionResult> {
					let client = [<create_client_ $client_type:lower>]().await;
					let contribution = client.user().contribution(user_name.as_deref()).await?;
					Ok(contribution.into())
				}

				/// 获取用户仓库列表
				///
				/// ## 参数
				/// - `user_name` 用户名称, 为空时获取当前登录用户仓库列表
				/// - `option` 仓库列表选项
				#[napi]
				pub async fn repo_list(
					&self,
					user_name: Option<String>,
					option: Option<RepoListOptions>,
				) -> Result<Vec<RepoInfo>> {
					let client = [<create_client_ $client_type:lower>]().await;
					let repo_infos = client.user().repo_list(user_name.as_deref(), option.map(|o| o.into())).await?;
					Ok(repo_infos.into_iter().map(|v| v.into()).collect())
				}
			}

			// Org 子模块
			#[derive(Debug, Default)]
			#[napi(constructor)]
			pub struct [<$client_type Org>];

			#[napi]
			impl [<$client_type Org>] {
				/// 获取组织信息
				///
				/// ## 参数
				/// - `org_name` 组织名称
				#[napi]
				pub async fn info(&self, org_name: String) -> Result<OrgInfo> {
					let client = [<create_client_ $client_type:lower>]().await;
					let org_info = client.org().info(org_name.as_str()).await?;
					Ok(org_info.into())
				}

				/// 获取组织头像地址
				///
				/// ## 参数
				/// - `org_name` 组织名称
				#[napi]
				pub async fn avatar_url(&self, org_name: String) -> Result<String> {
					let client = [<create_client_ $client_type:lower>]().await;
					let avatar_url = client.org().avatar_url(org_name.as_str()).await?;
					Ok(avatar_url)
				}

				/// 获取组织仓库列表
				///
				/// ## 参数
				/// - `org_name` 组织名称
				/// - `option` 仓库列表选项
				#[napi]
				pub async fn repo_list(
					&self,
					org_name: String,
					option: Option<RepoListOptions>,
				) -> Result<Vec<RepoInfo>> {
					let client = [<create_client_ $client_type:lower>]().await;
					let repo_infos = client.org().repo_list(org_name.as_str(), option.map(|o| o.into())).await?;
					Ok(repo_infos.into_iter().map(|v| v.into()).collect())
				}
			}

			// Repo 子模块
			#[derive(Debug, Default)]
			#[napi(constructor)]
			pub struct [<$client_type Repo>];

			#[napi]
			impl [<$client_type Repo>] {
				/// 获取仓库信息
				///
				/// ## 参数
				/// - `owner` 仓库所有者
				/// - `repo` 仓库名称
				#[napi]
				pub async fn info(&self, owner: String, repo: String) -> Result<RepoInfo> {
					let client = [<create_client_ $client_type:lower>]().await;
					let repo_info = client.repo().info((owner.as_str(), repo.as_str())).await?;
					Ok(repo_info.into())
				}

				/// 添加仓库协作者
				///
				/// ## 参数
				/// - `owner` 仓库所有者
				/// - `repo` 仓库名称
				/// - `user_name` 协作者用户名
				/// - `permission` 协作者权限, 默认为 `Pull`, 可选值为 `Admin`, `Push`, `Pull`
				#[napi]
				pub async fn add_collaborator(
					&self,
					owner: String,
					repo: String,
					user_name: String,
					permission: Option<CollaboratorPermission>,
				) -> Result<CollaboratorResult> {
					let client = [<create_client_ $client_type:lower>]().await;
					let collaborator_result = client
						.repo().add_repo_collaborator(
							(owner.as_str(), repo.as_str()),
							user_name.as_str(),
							permission.map(|p| p.into()),
						)
						.await?;
					Ok(collaborator_result.into())
				}
			}

			// Commit 子模块
			#[derive(Debug, Default)]
			#[napi(constructor)]
			pub struct [<$client_type Commit>];

			#[napi]
			impl [<$client_type Commit>] {
				/// 获取仓库提交信息
				///
				/// ## 参数
				/// - `owner` 仓库所有者
				/// - `repo` 仓库名称
				/// - `sha` 提交SHA, 如果不设置则会获取默认分支的最新提交
				#[napi]
				pub async fn info(
					&self,
					owner: String,
					repo: String,
					sha: Option<String>,
				) -> Result<CommitInfo> {
					let client = [<create_client_ $client_type:lower>]().await;
					let commit_info = client.commit().info((owner.as_str(), repo.as_str()), sha.as_deref()).await?;
					Ok(commit_info.into())
				}

				/// 获取仓库提交列表
				///
				/// ## 参数
				/// - `owner` 仓库所有者
				/// - `repo` 仓库名称
				/// - `option` 提交列表选项
				#[napi]
				pub async fn list(
					&self,
					owner: String,
					repo: String,
					option: Option<CommitListOptions>,
				) -> Result<Vec<CommitInfo>> {
					let client = [<create_client_ $client_type:lower>]().await;
					let commit_infos = client.commit().list((owner.as_str(), repo.as_str()), option.map(|o| o.into())).await?;
					Ok(commit_infos.into_iter().map(|v| v.into()).collect())
				}
			}

			// Issue 子模块
			#[derive(Debug, Default)]
			#[napi(constructor)]
			pub struct [<$client_type Issue>];

			#[napi]
			impl [<$client_type Issue>] {
				/// 创建一个issue
				///
				/// ## 参数
				/// - `owner` 仓库所有者
				/// - `repo` 仓库名称
				/// - `title` issue标题
				/// - `body` issue内容
				/// - `option` 创建issue选项
				#[napi]
				pub async fn create(
					&self,
					owner: String,
					repo: String,
					title: String,
					body: Option<String>,
					option: Option<CreateIssueOptions>,
				) -> Result<IssueInfo> {
					let client = [<create_client_ $client_type:lower>]().await;
					let repo_path = (owner.as_str(), repo.as_str());
					let issue_info = client.issue().create(repo_path, title.as_str(), body.as_deref(), option.map(|o| o.into())).await?;
					Ok(issue_info.into())
				}

				/// 获取issue信息
				///
				/// ## 参数
				/// - `owner` 仓库所有者
				/// - `repo` 仓库名称
				/// - `issue_number` issue编号
				#[napi]
				pub async fn info(&self, owner: String, repo: String, issue_number: String) -> Result<IssueInfo> {
					let client = [<create_client_ $client_type:lower>]().await;
					let repo_path = (owner.as_str(), repo.as_str());
					let issue_info = client.issue().info(repo_path, issue_number.as_str()).await?;
					Ok(issue_info.into())
				}

				/// 获取仓库所有issue信息
				///
				/// ## 参数
				/// - `owner` 仓库所有者
				/// - `repo` 仓库名称
				/// - `option` issue列表选项
				#[napi]
				pub async fn list(&self, owner: String, repo: String, option: Option<IssueListOptions>) -> Result<Vec<IssueInfo>> {
					let client = [<create_client_ $client_type:lower>]().await;
					let repo_path = (owner.as_str(), repo.as_str());
					let issue_infos = client.issue().list(repo_path, option.map(|o| o.into())).await?;
					Ok(issue_infos.into_iter().map(|v| v.into()).collect())
				}

				/// 更新issue信息
				///
				/// ## 参数
				/// - `owner` 仓库所有者
				/// - `repo` 仓库名称
				/// - `issue_number` issue编号
				/// - `options` 更新issue选项
				#[napi]
				pub async fn update(&self, owner: String, repo: String, issue_number: String, options: Option<UpdateIssueOptions>) -> Result<IssueInfo> {
					let client = [<create_client_ $client_type:lower>]().await;
					let repo_path = (owner.as_str(), repo.as_str());
					let issue_info = client.issue().update(repo_path, issue_number.as_str(), options.map(|o| o.into())).await?;
					Ok(issue_info.into())
				}
			}

			// Client 主模块
			#[derive(Debug, Default)]
			#[napi(constructor)]
			pub struct [<$client_type Client>];

			#[napi]
			impl [<$client_type Client>] {
				/// 设置访问令牌
				///
				/// ## 参数
				/// - `token` 访问令牌
				#[napi]
				pub fn set_token(&self, token: String) -> Result<()> {
					let rt = RT_RUNTIME.lock().unwrap();
					rt.block_on(async {
						let mut client = [<$client_type:upper _CLIENT>].write().await;
						client.set_token(token.as_str())?;
						Ok(())
					})
				}

				/// 设置代理
				///
				/// ## 参数
				/// - `proxy` 代理地址
				///
				/// 支持http,https,socks5协议
				#[napi]
				pub fn set_proxy(&self, proxy: String) -> Result<()> {
					let rt = RT_RUNTIME.lock().unwrap();
					rt.block_on(async {
						let mut client = [<$client_type:upper _CLIENT>].write().await;
						client.set_proxy(proxy.as_str())?;
						Ok(())
					})
				}

				/// 获取用户操作模块
				#[napi]
				pub fn user(&self) -> [<$client_type User>] {
					[<$client_type User>]
				}

				/// 获取组织操作模块
				#[napi]
				pub fn org(&self) -> [<$client_type Org>] {
					[<$client_type Org>]
				}

				/// 获取仓库操作模块
				#[napi]
				pub fn repo(&self) -> [<$client_type Repo>] {
					[<$client_type Repo>]
				}

				/// 获取提交操作模块
				#[napi]
				pub fn commit(&self) -> [<$client_type Commit>] {
					[<$client_type Commit>]
				}

				/// 获取Issue操作模块
				#[napi]
				pub fn issue(&self) -> [<$client_type Issue>] {
					[<$client_type Issue>]
				}
			}
		}
	};
}

impl_client!(Cnb, nipaw_cnb::CnbClient);
impl_client!(Gitee, nipaw_gitee::GiteeClient);
impl_client!(Github, nipaw_github::GitHubClient);
impl_client!(GitCode, nipaw_gitcode::GitCodeClient);

#[napi]
impl GithubClient {
	#[napi]
	/// 设置反向代理
	///
	/// ## 参数
	/// - `url` - 反向代理地址, 反代地址需要同时支持`github.com`和`api.github.com`
	pub async fn set_reverse_proxy(&self, url: String) {
		let mut client = create_client_github().await;
		client.set_reverse_proxy(url.as_str())
	}
}
