use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoInfo {
	/// 仓库所有者
	pub owner: String,
	/// 仓库名称
	pub name: String,
	/// 仓库全名
	pub full_name: String,
	/// 仓库描述
	pub description: Option<String>,
	/// 仓库可见性
	pub visibility: Visibility,
	/// 是否是fork仓库
	pub fork: bool,
	/// 仓库fork数量
	pub fork_count: u64,
	/// 仓库语言
	pub language: Option<String>,
	/// 仓库星标数量
	pub star_count: u64,
	/// 仓库默认分支
	pub default_branch: String,
	/// 仓库创建时间
	pub created_at: DateTime<Utc>,
	/// 仓库更新时间
	pub updated_at: DateTime<Utc>,
	/// 仓库推送时间
	pub pushed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, IntoStaticStr)]
/// 只有`public`和`private`
pub enum Visibility {
	/// 公开
	#[serde(rename = "public")]
	Public,
	#[serde(rename = "private")]
	/// 私有
	Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaboratorResult {
	/// 协作者用户名
	pub login: String,
	/// 协作者头像URL
	pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaboratorPermission {
	/// 管理权限
	Admin,
	/// 推送权限
	Push,
	/// 拉取权限
	Pull,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RepoPath {
	/// 仓库所有者
	pub owner: String,
	/// 仓库名称
	pub repo: String,
}

impl RepoPath {
	pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
		Self { owner: owner.into(), repo: repo.into() }
	}
}

impl<'r> From<(&'r str, &'r str)> for RepoPath {
	fn from(repo_path: (&'r str, &'r str)) -> Self {
		Self {
			owner: repo_path.0.to_string(),
			repo: repo_path.1.to_string(),
		}
	}
}

impl From<(String, String)> for RepoPath {
	fn from(repo_path: (String, String)) -> Self {
		Self {
			owner: repo_path.0,
			repo: repo_path.1,
		}
	}
}

impl Display for RepoPath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}/{}", self.owner, self.repo)
	}
}
