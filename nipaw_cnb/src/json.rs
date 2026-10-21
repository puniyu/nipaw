use std::borrow::Cow;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub(crate) struct Json<'s>(Cow<'s, Value>);

impl<'s> Json<'s> {
	pub fn new(value: impl Into<Self>) -> Self {
		value.into()
	}

	pub fn as_inner(&self) -> &Value {
		self.0.as_ref()
	}

	pub fn as_inner_mut(&mut self) -> &mut Value {
		self.0.to_mut()
	}
}

impl From<Value> for Json<'_> {
	fn from(value: Value) -> Self {
		Self(Cow::Owned(value))
	}
}

impl<'s> From<&'s Value> for Json<'s> {
	fn from(value: &'s Value) -> Self {
		Self(Cow::Borrowed(value))
	}
}
