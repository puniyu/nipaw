use super::{default_page, default_per_page};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ListOptions {
	/// 每页数量，默认 30，最大 100
	#[serde(default = "default_per_page")]
	pub per_page: Option<u32>,
	/// 页码，默认 1
	#[serde(default = "default_page")]
	pub page: Option<u32>,
}

impl Default for ListOptions {
	fn default() -> Self {
		Self { per_page: default_per_page(), page: default_page() }
	}
}
