mod commit;
mod issue;
mod org;
mod release;
mod repo;
mod user;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub(crate) struct JsonValue(pub(crate) Value);

impl IntoIterator for JsonValue {
	type Item = JsonValue;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		match self.0 {
			Value::Array(vec) => vec.into_iter().map(JsonValue).collect::<Vec<_>>().into_iter(),
			Value::Object(map) => map
				.into_iter()
				.map(|(k, v)| {
					let obj = serde_json::json!({k: v});
					JsonValue(obj)
				})
				.collect::<Vec<_>>()
				.into_iter(),
			_ => Vec::new().into_iter(),
		}
	}
}

pub(crate) struct Html(pub(crate) String);

impl From<String> for Html {
	fn from(value: String) -> Self {
		Html(value)
	}
}
