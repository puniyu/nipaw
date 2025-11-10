use crate::{
	Result,
	option::{
		CommitListOptions, CreateIssueOptions, IssueListOptions, OrgRepoListOptions,
		ReposListOptions,
	},
	types::{
		collaborator::{CollaboratorPermission, CollaboratorResult},
		commit::CommitInfo,
		issue::IssueInfo,
		org::OrgInfo,
		repo::RepoInfo,
		user::{ContributionResult, UserInfo},
	},
};
use async_trait::async_trait;

#[async_trait]
pub trait Client: Send + Sync {
	/// 设置访问令牌
	///
	/// # 参数
	///
	/// * `token` - token
	///
	/// # 示例
	///
	/// ```ignore
	/// client.set_token("token").unwrap();
	/// ```
	fn set_token(&mut self, token: &str) -> Result<()>;

	/// 设置代理
	///
	/// # 参数
	///
	/// * `proxy` - 代理字符串
	///
	/// # 示例
	///
	/// ```ignore
	/// client.set_proxy("http://127.0.0.1:7890").unwrap();
	/// ```
	fn set_proxy(&mut self, proxy: &str) -> Result<()>;

	/// 获取当前授权用户信息
	async fn get_user_info(&self) -> Result<UserInfo>;

	/// 根据用户名获取用户信息
	///
	/// # 参数
	///
	/// * `user_name` - 用户名
	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo>;

	/// 获取用户头像URL
	///
	/// # 参数
	///
	/// * `user_name` - 用户名
	async fn get_user_avatar_url(&self, user_name: &str) -> Result<String>;

	/// 获取指定用户贡献数据
	async fn get_user_contribution(&self, user_name: &str) -> Result<ContributionResult>;

	/// 获取组织信息
	///
	/// # 参数
	///
	/// * `org_name` - 组织名
	///
	async fn get_org_info(&self, org_name: &str) -> Result<OrgInfo>;

	/// 获取组织仓库信息列表
	/// # 参数
	///
	/// * `org_name` - 组织名
	/// * `options` - 获取仓库列表选项, 详见 [OrgRepoListOptions]
	async fn get_org_repos(
		&self,
		org_name: &str,
		options: Option<OrgRepoListOptions>,
	) -> Result<Vec<RepoInfo>>;

	/// 获取组织头像URL
	///
	/// # 参数
	///
	/// * `org_name` - 组织名
	///
	async fn get_org_avatar_url(&self, org_name: &str) -> Result<String>;
	/// 获取仓库信息
	///
	/// # 参数
	///
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	///
	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo>;

	/// 获取用户仓库信息列表
	///
	/// # 参数
	///
	/// * `option` - 获取仓库列表选项, 详见 [ReposListOptions]
	///
	async fn get_user_repos(&self, option: Option<ReposListOptions>) -> Result<Vec<RepoInfo>>;

	/// 根据用户名获取用户仓库信息列表
	///
	/// # 参数
	///
	/// * `user_name` - 用户名
	/// * `option` - 获取仓库列表选项, 详见 [ReposListOptions]
	async fn get_user_repos_with_name(
		&self,
		user_name: &str,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>>;

	/// 获取仓库提交信息
	///
	/// # 参数
	///
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `sha` - 提交ID, 默认为最新提交
	///
	async fn get_commit_info(
		&self,
		repo_path: (&str, &str),
		sha: Option<&str>,
	) -> Result<CommitInfo>;

	/// 获取仓库所有提交信息
	///
	/// # 参数
	/// * `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// * `option` - 获取提交列表选项, 详见 [CommitListOptions]
	async fn get_commit_infos(
		&self,
		repo_path: (&str, &str),
		option: Option<CommitListOptions>,
	) -> Result<Vec<CommitInfo>>;

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
		repo_path: (&str, &str),
		user_name: &str,
		permission: Option<CollaboratorPermission>,
	) -> Result<CollaboratorResult>;

	/// 创建一个issue
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// - `title` - issue标题
	/// - `body` - issue内容
	/// - `option` - 创建issue选项, 详见 [CreateIssueOptions]
	///
	async fn create_issue(
		&self,
		repo_path: (&str, &str),
		title: &str,
		body: Option<&str>,
		option: Option<CreateIssueOptions>,
	) -> Result<IssueInfo>;

	/// 获取issue信息
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径，格式为 `(owner, repo)`
	/// - `issue_number` - issue编号
	///
	async fn get_issue_info(
		&self,
		repo_path: (&str, &str),
		issue_number: String,
	) -> Result<IssueInfo>;

	/// 获取仓库所有issue信息
	///
	/// ## 参数
	/// - `repo_path` - 仓库路径，格式为 `(owner, repo)`
	async fn get_issue_list(
		&self,
		repo_path: (&str, &str),
		options: Option<IssueListOptions>,
	) -> Result<Vec<IssueInfo>>;
}
