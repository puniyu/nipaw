use crate::GitCodeClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::option::repo::ListOptions;
use nipaw_core::types::{
	repo::RepoInfo,
	user::{ContributionResult, UserInfo},
};
use nipaw_core::{Error, Result, User};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GitCodeUser(pub(crate) Arc<GitCodeClientInner>);

#[async_trait]
impl User for GitCodeUser {
	async fn info(&self, user_name: Option<&str>) -> Result<UserInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
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
		if let Some(user) = user_info.0.as_object_mut() {
			let user_name = if let Some(name) = user_name {
				name.to_string()
			} else {
				user.get("username").and_then(|v| v.as_str()).unwrap().to_string()
			};
			let repo_count =
				crate::get_user_repo_count(client.clone(), &self.0.config, &user_name).await?;
			user.insert("repo_count".to_string(), Value::Number(repo_count.into()));
		}
		Ok(user_info.into())
	}

	async fn avatar_url(&self, user_name: Option<&str>) -> Result<String> {
		let (token, web_api_url, base_url) =
			(&self.0.config.token, &self.0.config.web_api_url, &self.0.config.base_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		if let Some(user_name) = user_name {
			let url =
				format!("{}/uc/api/v1/user/setting/profile?username={}", web_api_url, user_name);
			let client = self.0.client.read().await;
			let res =
				client.get(url).header("Referer", base_url).send().await?.json::<Value>().await?;
			let avatar_url = res.get("avatar").and_then(|v| v.as_str()).unwrap().to_string();
			return Ok(avatar_url);
		}
		let info = self.info(None).await?;
		Ok(info.avatar_url)
	}

	async fn contribution(&self, user_name: Option<&str>) -> Result<ContributionResult> {
		let (token, web_api_url, base_url) =
			(&self.0.config.token, &self.0.config.web_api_url, &self.0.config.base_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let user_name = if let Some(user_name) = user_name {
			user_name.to_string()
		} else {
			self.info(None).await?.login
		};
		let url = format!(
			"{}/uc/api/v1/events/{}/contributions?username={}",
			web_api_url, user_name, user_name
		);
		let client = self.0.client.read().await;
		let request = client.get(url);
		let res = request.header("Referer", base_url).send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn repo_list(
		&self,
		user_name: Option<&str>,
		option: Option<ListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
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

		params.insert("sort", "pushed".to_string());
		if user_name.is_none() {
			params.insert("type", "owner".to_string());
		}

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or(30).max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|v| v.into()).collect())
	}
}
