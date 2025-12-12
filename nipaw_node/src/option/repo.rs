use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
#[napi(object)]
pub struct RepoListOptions {
	/// 每页数量，默认 30，最大 100
	pub per_page: Option<u32>,
	/// 页码，默认 1
	pub page: Option<u32>,
}

impl From<RepoListOptions> for nipaw_core::option::repo::ListOptions {
	fn from(value: RepoListOptions) -> Self {
		nipaw_core::option::repo::ListOptions { per_page: value.per_page, page: value.page }
	}
}
