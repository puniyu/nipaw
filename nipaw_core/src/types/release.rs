use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseInfo {
	/// 标签名
	pub tag_name: String,
	/// 目标提交分支或 SHA
	pub target_commitish: String,
	/// 是否为预发布版本
	pub prerelease: bool,
	/// 发布名称
	pub name: String,
	/// 发布说明内容
	pub body: Option<String>,
	/// 发布作者信息
	pub author: AuthorInfo,
	/// 创建时间
	pub created_at: DateTime<Utc>,
	/// 附件资源列表
	pub assets: Vec<AssetsInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorInfo {
	/// 用户登录名
	pub login: String,
	/// 用户头像地址
	pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetsInfo {
	/// 资源文件名
	pub name: String,
	/// 资源下载地址
	pub url: String,
}
