use crate::GitCodeClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::option::repo::ListOptions;
use nipaw_core::types::{org::OrgInfo, repo::RepoInfo};
use nipaw_core::{Org, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GitCodeOrg(pub(crate) Arc<GitCodeClientInner>);

#[async_trait]
impl Org for GitCodeOrg {
	async fn info(&self, org_name: &str) -> Result<OrgInfo> {
		let (token, web_api_url) = (&self.0.config.token, &self.0.config.web_api_url);
		let url = format!("{}/orgs/{}", web_api_url, org_name);
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
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|v| v.into()).collect())
	}

	async fn avatar_url(&self, org_name: &str) -> Result<String> {
		let (base_url, web_api_url) = (&self.0.config.base_url, &self.0.config.web_api_url);
		let client = self.0.client.read().await;
		let url = format!("{}/api/v2/groups/{}", web_api_url, org_name);
		let res = client.get(url).header("Referer", base_url).send().await?.json::<Value>().await?;
		let avatar_url = res.get("avatar").and_then(|v| v.as_str()).unwrap().to_string();
		Ok(avatar_url)
	}
}
