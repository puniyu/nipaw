use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[napi(object)]
pub struct CollaboratorResult {
	/// 协作者用户名
	pub login: String,
	/// 协作者头像URL
	pub avatar_url: String,
}

impl From<nipaw_core::types::collaborator::CollaboratorResult> for CollaboratorResult {
	fn from(result: nipaw_core::types::collaborator::CollaboratorResult) -> Self {
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

impl From<CollaboratorPermission> for nipaw_core::types::collaborator::CollaboratorPermission {
	fn from(permission: CollaboratorPermission) -> Self {
		match permission {
			CollaboratorPermission::Admin => nipaw_core::types::collaborator::CollaboratorPermission::Admin,
			CollaboratorPermission::Push => nipaw_core::types::collaborator::CollaboratorPermission::Push,
			CollaboratorPermission::Pull => nipaw_core::types::collaborator::CollaboratorPermission::Pull,
		}
	}
}
