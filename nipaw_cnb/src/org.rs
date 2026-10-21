use crate::{CnbClientInner, Error, Json, Result};
use async_trait::async_trait;
use nipaw_core::{
	Org,
	org::{OrgInfo, OrgListOptions},
	repo::RepoInfo,
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CnbOrg(pub(crate) Arc<CnbClientInner>);

#[async_trait]
impl Org for CnbOrg {
	type Err = Error;

	async fn info(&self, org_name: &str) -> Result<OrgInfo> {
		let config = self.0.config();
		let (token, api_url, base_url) = (&config.token, &config.api_url, &config.base_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/{}", api_url, org_name);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut org_info: Json = resp.json().await?;

		if let Some(login) = org_info.as_inner().get("login").and_then(|v| v.as_str()) {
			let avatar_url = format!("{}/{}/-/logos/l", base_url, login);
			org_info
				.as_inner_mut()
				.as_object_mut()
				.unwrap()
				.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(org_info.into())
	}

	async fn repo_list(
		&self,
		org_name: &str,
		options: Option<OrgListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/{}/-/repos", api_url, org_name);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(option) = options {
			let per_page = option.per_page.unwrap_or(30).max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.send().await?;
		let repo_infos: Vec<Json> = resp.json().await?;
		Ok(repo_infos.into_iter().map(Into::into).collect())
	}

	async fn avatar_url(&self, org_name: &str) -> Result<String> {
		let config = self.0.config();
		let base_url = config.base_url.as_str();
		let url = format!("{}/{}/-/logos/l", base_url, org_name);
		Ok(url)
	}
}

impl From<Json<'_>> for OrgInfo {
	fn from(json_value: Json<'_>) -> Self {
		let org_info = json_value.as_inner();
		Self {
			login: org_info.get("login").and_then(|v| v.as_str()).unwrap().to_string(),
			name: org_info
				.get("name")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			email: org_info
				.get("email")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			avatar_url: org_info.get("avatar_url").and_then(|v| v.as_str()).unwrap().to_string(),
			description: org_info
				.get("description")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			follow_count: org_info.get("followers").and_then(|v| v.as_u64()).unwrap_or(0),
		}
	}
}
