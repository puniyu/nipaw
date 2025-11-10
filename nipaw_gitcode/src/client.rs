use crate::middleware::{HeaderMiddleware, ResponseMiddleware};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::sync::{LazyLock, OnceLock};

pub(crate) static HTTP_CLIENT: LazyLock<ClientWithMiddleware> = LazyLock::new(|| {
	let client = Client::new();
	ClientBuilder::new(client).with(HeaderMiddleware).with(ResponseMiddleware).build()
});

pub(crate) static PROXY_URL: OnceLock<String> = OnceLock::new();
