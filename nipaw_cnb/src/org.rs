use crate::CnbClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::option::repo::ListOptions;
use nipaw_core::types::{org::OrgInfo, repo::RepoInfo};
use nipaw_core::{Org, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct CnbOrg(pub(crate) Arc<CnbClientInner>);

#[async_trait]
impl Org for CnbOrg {
	async fn info(&self, org_name: &str) -> Result<OrgInfo> {
		let (token, api_url, base_url) =
			(&self.0.config.token, &self.0.config.api_url, &self.0.config.base_url);
		let url = format!("{}/{}", api_url, org_name);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut org_info: JsonValue = resp.json().await?;

		if let Some(login) = org_info.0.get("login").and_then(|v| v.as_str()) {
			let avatar_url = format!("{}/{}/-/logos/l", base_url, login);
			org_info
				.0
				.as_object_mut()
				.unwrap()
				.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(org_info.into())
	}

	async fn repo_list(
		&self,
		org_name: &str,
		options: Option<ListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!("{}/{}/-/repos", api_url, org_name);
		let client = self.0.client.read().await;
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
		let repo_infos: Vec<JsonValue> = resp.json().await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	async fn avatar_url(&self, org_name: &str) -> Result<String> {
		let base_url = &self.0.config.base_url;
		let url = format!("{}/{}/-/logos/l", base_url, org_name);
		Ok(url)
	}
}
