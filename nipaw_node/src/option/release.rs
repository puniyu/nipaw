use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[napi(object)]
pub struct UpdateReleaseOptions {
	/// 发行名称
	pub name: Option<String>,
	/// 发行正文
	pub body: Option<String>,
}

impl From<UpdateReleaseOptions> for nipaw_core::option::release::UpdateOption {
	fn from(value: UpdateReleaseOptions) -> Self {
		nipaw_core::option::release::UpdateOption { name: value.name, body: value.body }
	}
}
