use crate::GitHubClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::option::repo::ListOptions;
use nipaw_core::types::{org::OrgInfo, repo::RepoInfo};
use nipaw_core::{Org, Result};
use std::collections::HashMap;
use std::sync::Arc;

pub struct GitHubOrg(pub(crate) Arc<GitHubClientInner>);

#[async_trait]
impl Org for GitHubOrg {
	async fn info(&self, org_name: &str) -> Result<OrgInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!("{}/orgs/{}", api_url, org_name);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn repo_list(
		&self,
		org_name: &str,
		options: Option<ListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!("{}/orgs/{}/repos", api_url, org_name);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		let mut params = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		if let Some(option) = options {
			let per_page = option.per_page.unwrap_or(30).max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.send().await?;
		let repo_infos: Vec<JsonValue> = resp.json().await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	async fn avatar_url(&self, org_name: &str) -> Result<String> {
		let api_url = self.0.config.api_url.as_str();
		let url = format!("{}/orgs/{}", api_url, org_name);
		let client = self.0.client.read().await;
		let request = client.get(url);
		let resp = request.send().await?;
		let org_html: String = resp.text().await?;
		let document = scraper::Html::parse_document(&org_html);
		let selector = scraper::Selector::parse("meta[name='hovercard-subject-tag']").unwrap();
		let element = document.select(&selector).next().unwrap();
		let org_id = element.value().attr("content").unwrap();
		let avatar_url = format!("https://avatars.githubusercontent.com/u/{}?v=4", org_id);
		Ok(avatar_url)
	}
}
