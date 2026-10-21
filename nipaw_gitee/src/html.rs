pub(crate) struct Html(String);

impl Html {
	#[allow(dead_code)]
	pub fn new(html: &str) -> Self {
		Self(html.to_string())
	}

	pub fn as_inner(&self) -> &str {
		&self.0
	}
}

impl From<String> for Html {
	fn from(value: String) -> Self {
		Self(value)
	}
}
