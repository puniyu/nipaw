use crate::CnbClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use chrono::{Datelike, Local};
use nipaw_core::option::repo::ListOptions;
use nipaw_core::types::{
	repo::RepoInfo,
	user::{ContributionResult, UserInfo},
};
use nipaw_core::{Error, Result, User};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct CnbUser(pub(crate) Arc<CnbClientInner>);

#[async_trait]
impl User for CnbUser {
	async fn info(&self, user_name: Option<&str>) -> Result<UserInfo> {
		let (token, api_url, base_url) =
			(&self.0.config.token, &self.0.config.api_url, &self.0.config.base_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		let client = self.0.client.read().await;
		let url = if let Some(name) = user_name {
			format!("{}/users/{}", api_url, name)
		} else {
			format!("{}/user", api_url)
		};
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut user_info: JsonValue = resp.json().await?;

		if let Some(username) = user_info.0.get("username").and_then(|v| v.as_str()) {
			let avatar_url = format!("{}/users/{}/avatar/l", base_url, username);
			user_info
				.0
				.as_object_mut()
				.unwrap()
				.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(user_info.into())
	}

	async fn avatar_url(&self, user_name: Option<&str>) -> Result<String> {
		let (token, base_url) = (&self.0.config.token, &self.0.config.base_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		if let Some(user_name) = user_name {
			return Ok(format!("{}/users/{}/avatar/l", base_url, user_name));
		}
		let info = self.info(None).await?;
		Ok(info.avatar_url)
	}

	async fn contribution(&self, user_name: Option<&str>) -> Result<ContributionResult> {
		let (token, base_url) = (&self.0.config.token, &self.0.config.base_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		let user_name = if let Some(user_name) = user_name {
			user_name.to_string()
		} else {
			self.info(None).await?.login
		};
		let year = Local::now().year();
		let url = format!("{}/users/{}/calendar?year={}", base_url, user_name, year);
		let client = self.0.client.read().await;
		let resp = client.get(url).header("Accept", " application/vnd.cnb.web+json").send().await?;
		let contribution_result: JsonValue = resp.json().await?;
		Ok(contribution_result.into())
	}

	async fn repo_list(
		&self,
		user_name: Option<&str>,
		option: Option<ListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = if let Some(name) = user_name {
			format!("{}/users/{}/repos", api_url, name)
		} else {
			format!("{}/user/repos", api_url)
		};
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}

		params.insert("desc", true.to_string());
		params.insert("order_by", "last_updated_at".to_owned());
		if user_name.is_some() {
			params.insert("role", "owner".to_owned());
		}

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or(30).max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.query(&params).send().await?;
		let repo_infos: Vec<JsonValue> = resp.json().await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}
}
