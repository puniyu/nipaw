#[derive(Debug, Clone)]
pub struct Config {
	pub(crate) base_url: String,
	pub(crate) web_url: String,
	pub(crate) api_url: String,
	pub(crate) token: Option<String>,
}

const DEFAULT_BASE_URL: &str = "https://github.com";
const DEFAULT_WEB_URL: &str = DEFAULT_BASE_URL;
const DEFAULT_API_URL: &str = "https://api.github.com";

impl Default for Config {
	fn default() -> Self {
		Self {
			base_url: DEFAULT_BASE_URL.to_string(),
			web_url: DEFAULT_WEB_URL.to_string(),
			api_url: DEFAULT_API_URL.to_string(),
			token: None,
		}
	}
}

impl Config {
	pub fn set_base_url(&mut self, base_url: String) {
		self.base_url = base_url;
	}

	pub fn set_web_url(&mut self, web_url: String) {
		self.web_url = web_url;
	}

	pub fn set_api_url(&mut self, api_url: String) {
		self.api_url = api_url;
	}

	pub fn set_token(&mut self, token: &str) {
		self.token = Some(token.to_string());
	}
}
