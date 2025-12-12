use crate::GiteeClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::option::repo::ListOptions;
use nipaw_core::types::{org::OrgInfo, repo::RepoInfo};
use nipaw_core::{Org, Result};
use std::collections::HashMap;
use std::sync::Arc;

pub struct GiteeOrg(pub(crate) Arc<GiteeClientInner>);

#[async_trait]
impl Org for GiteeOrg {
	async fn info(&self, org_name: &str) -> Result<OrgInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!("{}/orgs/{}", api_url, org_name);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let org_info: JsonValue = resp.json().await?;
		Ok(org_info.into())
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
			request = request.query(&[("access_token", token.as_str())]);
		}
		if let Some(option) = options {
			let per_page = option.per_page.unwrap_or(30).max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or(1);
			params.insert("page", page.to_string());
		}
		let resp = request.send().await?;
		let repo_list: Vec<JsonValue> = resp.json().await?;
		Ok(repo_list.into_iter().map(|v| v.into()).collect())
	}

	async fn avatar_url(&self, org_name: &str) -> Result<String> {
		let base_url = &self.0.config.base_url;
		let url = format!("{}/{}", base_url, org_name);
		let client = self.0.client.read().await;
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
