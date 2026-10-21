pub mod collaborator;

use super::{default_page, default_per_page};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use jiff::civil::DateTime;
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
	pub created_at: DateTime,
	/// 仓库更新时间
	pub updated_at: DateTime,
	/// 仓库推送时间
	pub pushed_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString, IntoStaticStr)]
pub enum Visibility {
	/// 公开
	#[serde(rename = "public")]
	Public,
	#[serde(rename = "private")]
	/// 私有
	Private,
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
		Self { owner: repo_path.0.to_string(), repo: repo_path.1.to_string() }
	}
}

impl From<(String, String)> for RepoPath {
	fn from(repo_path: (String, String)) -> Self {
		Self { owner: repo_path.0, repo: repo_path.1 }
	}
}

impl Display for RepoPath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}/{}", self.owner, self.repo)
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RepoListOptions {
	/// 每页数量，默认 30，最大 100
	#[serde(default = "default_per_page")]
	pub per_page: Option<u32>,
	/// 页码，默认 1
	#[serde(default = "default_page")]
	pub page: Option<u32>,
}

impl Default for RepoListOptions {
	#[inline]
	fn default() -> Self {
		Self { per_page: default_per_page(), page: default_page() }
	}
}
