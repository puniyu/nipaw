use chrono::{DateTime, Utc};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[napi(object)]
pub struct RepoInfo {
	/// 仓库所有者
	pub owner: String,
	/// 仓库名称
	pub name: String,
	/// 仓库全名
	pub full_name: String,
	/// 仓库描述
	pub description: Option<String>,
	/// 仓库可见性, public/private
	pub visibility: RepoVisibility,
	/// 是否是fork仓库
	pub fork: bool,
	/// 仓库fork数量
	pub fork_count: u32,
	/// 仓库语言
	pub language: Option<String>,
	/// 仓库星标数量
	pub star_count: u32,
	/// 仓库默认分支
	pub default_branch: String,
	/// 仓库创建时间
	pub created_at: DateTime<Utc>,
	/// 仓库更新时间
	pub updated_at: DateTime<Utc>,
	/// 仓库推送时间
	pub pushed_at: DateTime<Utc>,
}

impl From<nipaw_core::types::repo::RepoInfo> for RepoInfo {
	fn from(repo_info: nipaw_core::types::repo::RepoInfo) -> Self {
		Self {
			owner: repo_info.owner,
			name: repo_info.name,
			full_name: repo_info.full_name,
			description: repo_info.description,
			visibility: repo_info.visibility.into(),
			fork: repo_info.fork,
			fork_count: repo_info.fork_count as u32,
			language: repo_info.language,
			star_count: repo_info.star_count as u32,
			default_branch: repo_info.default_branch,
			created_at: repo_info.created_at,
			updated_at: repo_info.updated_at,
			pushed_at: repo_info.pushed_at,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(string_enum)]
pub enum RepoVisibility {
	/// 公开
	Public,
	/// 私有
	Private,
}

impl From<nipaw_core::types::repo::Visibility> for RepoVisibility {
	fn from(visibility: nipaw_core::types::repo::Visibility) -> Self {
		match visibility {
			nipaw_core::types::repo::Visibility::Public => RepoVisibility::Public,
			nipaw_core::types::repo::Visibility::Private => RepoVisibility::Private,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct CollaboratorResult {
	/// 协作者用户名
	pub login: String,
	/// 协作者头像URL
	pub avatar_url: String,
}

impl From<nipaw_core::types::repo::CollaboratorResult> for CollaboratorResult {
	fn from(result: nipaw_core::types::repo::CollaboratorResult) -> Self {
		Self { login: result.login, avatar_url: result.avatar_url }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(string_enum)]
pub enum CollaboratorPermission {
	/// 管理权限
	Admin,
	/// 推送权限
	Push,
	/// 拉取权限
	Pull,
}

impl From<CollaboratorPermission> for nipaw_core::types::repo::CollaboratorPermission {
	fn from(permission: CollaboratorPermission) -> Self {
		match permission {
			CollaboratorPermission::Admin => nipaw_core::types::repo::CollaboratorPermission::Admin,
			CollaboratorPermission::Push => nipaw_core::types::repo::CollaboratorPermission::Push,
			CollaboratorPermission::Pull => nipaw_core::types::repo::CollaboratorPermission::Pull,
		}
	}
}
