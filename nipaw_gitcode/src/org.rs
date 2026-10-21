use crate::{Error, GitCodeClientInner, Json, Result};
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
pub struct GitCodeOrg(pub(crate) Arc<GitCodeClientInner>);

#[async_trait]
impl Org for GitCodeOrg {
	type Err = Error;

	async fn info(&self, org_name: &str) -> Result<OrgInfo> {
		let config = self.0.config();
		let (token, web_api_url) = (&config.token, &config.web_api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/orgs/{}", web_api_url, org_name);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let res = request.send().await?.json::<Json>().await?;
		Ok(res.into())
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
		let url = format!("{}/orgs/{}/repos", api_url, org_name);
		let client = self.0.http();
		let mut request = client.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		if let Some(option) = options {
			let per_page = option.per_page.unwrap_or(30).max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let res = request.query(&params).send().await?.json::<Vec<Json>>().await?;
		Ok(res.into_iter().map(Into::into).collect())
	}

	async fn avatar_url(&self, org_name: &str) -> Result<String> {
		let config = self.0.config();
		let (token, base_url, web_api_url) = (&config.token, &config.base_url, &config.web_api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let client = self.0.http();
		let url = format!("{}/api/v2/groups/{}", web_api_url, org_name);
		let res = client.get(url).header("Referer", base_url).send().await?.json::<Value>().await?;
		let avatar_url = res.get("avatar").and_then(|v| v.as_str()).unwrap().to_string();
		Ok(avatar_url)
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
			follow_count: org_info.get("followers").and_then(|v| v.as_u64()).unwrap(),
		}
	}
}
