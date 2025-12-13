use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UpdateOption {
	/// 发行名称
	pub name: Option<String>,
	/// 发行正文
	pub body: Option<String>,
}
