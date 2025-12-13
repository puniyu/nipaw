use chrono::{DateTime, Utc};
use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
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
	pub author: ReleaseAuthorInfo,
	/// 创建时间
	pub created_at: DateTime<Utc>,
	/// 附件资源列表
	pub assets: Vec<AssetsInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct ReleaseAuthorInfo {
	/// 用户登录名
	pub login: String,
	/// 用户头像地址
	pub avatar_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct AssetsInfo {
	/// 资源文件名
	pub name: String,
	/// 资源下载地址
	pub url: String,
}

impl From<nipaw_core::types::release::ReleaseInfo> for ReleaseInfo {
	fn from(value: nipaw_core::types::release::ReleaseInfo) -> Self {
		Self {
			tag_name: value.tag_name,
			target_commitish: value.target_commitish,
			prerelease: value.prerelease,
			name: value.name,
			body: value.body,
			author: value.author.into(),
			created_at: value.created_at,
			assets: value.assets.into_iter().map(Into::into).collect(),
		}
	}
}

impl From<nipaw_core::types::release::AuthorInfo> for ReleaseAuthorInfo {
	fn from(value: nipaw_core::types::release::AuthorInfo) -> Self {
		Self { login: value.login, avatar_url: value.avatar_url }
	}
}

impl From<nipaw_core::types::release::AssetsInfo> for AssetsInfo {
	fn from(value: nipaw_core::types::release::AssetsInfo) -> Self {
		Self { name: value.name, url: value.url }
	}
}
