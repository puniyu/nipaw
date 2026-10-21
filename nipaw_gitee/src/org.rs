use crate::{Error, GiteeClientInner, Json, Result};
use async_trait::async_trait;
use nipaw_core::{
	Org,
	org::{OrgInfo, OrgListOptions},
	repo::RepoInfo,
};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GiteeOrg(pub(crate) Arc<GiteeClientInner>);

#[async_trait]
impl Org for GiteeOrg {
	type Err = Error;

	async fn info(&self, org_name: &str) -> Result<OrgInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		let url = format!("{}/orgs/{}", api_url, org_name);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let org_info: Json = resp.json().await?;
		Ok(org_info.into())
	}

	async fn repo_list(
		&self,
		org_name: &str,
		options: Option<OrgListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		let url = format!("{}/orgs/{}/repos", api_url, org_name);
		let client = self.0.http();
		let mut request = client.get(url);
		let mut params = HashMap::new();
		if let Some(token) = token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		if let Some(option) = options {
			let per_page = option.per_page.unwrap_or(30).max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or(1);
			params.insert("page", page.to_string());
		}
		let resp = request.send().await?;
		let repo_list: Vec<Json> = resp.json().await?;
		Ok(repo_list.into_iter().map(Into::into).collect())
	}

	async fn avatar_url(&self, org_name: &str) -> Result<String> {
		let config = self.0.config();
		let base_url = &config.base_url;
		let url = format!("{}/{}", base_url, org_name);
		let client = self.0.http();
		let request = client.get(url);
		let resp = request.send().await?;
		let org_html: String = resp.text().await?;

		let document = scraper::Html::parse_document(&org_html);
		let selector = scraper::Selector::parse("img.avatar.current-group-avatar").unwrap();

		let element = document.select(&selector).next().unwrap();
		let src = element.value().attr("src").unwrap();
		let avatar_url = src.split('!').next().unwrap_or(src).to_string();
		Ok(avatar_url)
	}
}

impl From<Json<'_>> for OrgInfo {
	fn from(org: Json<'_>) -> Self {
		let org_info = org.as_inner();
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
			follow_count: org_info.get("follow_count").and_then(|v| v.as_u64()).unwrap_or(0),
		}
	}
}
