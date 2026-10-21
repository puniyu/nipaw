use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaboratorInfo {
	/// 协作者用户名
	pub login: String,
	/// 协作者头像URL
	pub avatar_url: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
	/// 管理权限
	Admin,
	/// 推送权限
	Push,
	/// 拉取权限
	Pull,
}