use crate::{Error, GitCodeClientInner, Json, Result};
use async_trait::async_trait;
use nipaw_core::{
	User,
	repo::{RepoInfo, RepoListOptions},
	user::{Contribution, ContributionDay, UserInfo},
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GitCodeUser(pub(crate) Arc<GitCodeClientInner>);

#[async_trait]
impl User for GitCodeUser {
	type Err = Error;

	async fn info(&self, user_name: Option<&str>) -> Result<UserInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let client = self.0.http();
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
		let mut user_info: Json = resp.json().await?;
		if let Some(user) = user_info.as_inner_mut().as_object_mut() {
			let user_name = if let Some(name) = user_name {
				name.to_string()
			} else {
				user.get("username").and_then(|v| v.as_str()).unwrap().to_string()
			};
			let repo_count =
				crate::get_user_repo_count(client.clone(), &config, &user_name).await?;
			user.insert("repo_count".to_string(), Value::Number(repo_count.into()));
		}
		Ok(user_info.into())
	}

	async fn avatar_url(&self, user_name: Option<&str>) -> Result<String> {
		let config = self.0.config();
		let (token, web_api_url, base_url) = (&config.token, &config.web_api_url, &config.base_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		if let Some(user_name) = user_name {
			let url =
				format!("{}/uc/api/v1/user/setting/profile?username={}", web_api_url, user_name);
			let client = self.0.http();
			let res =
				client.get(url).header("Referer", base_url).send().await?.json::<Value>().await?;
			let avatar_url = res.get("avatar").and_then(|v| v.as_str()).unwrap().to_string();
			return Ok(avatar_url);
		}
		let info = self.info(None).await?;
		Ok(info.avatar_url)
	}

	async fn contribution(&self, user_name: Option<&str>) -> Result<Contribution> {
		let config = self.0.config();
		let (token, web_api_url, base_url) = (&config.token, &config.web_api_url, &config.base_url);
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
		let client = self.0.http();
		let request = client.get(url);
		let res = request.header("Referer", base_url).send().await?.json::<Json>().await?;
		Ok(res.into())
	}

	async fn repo_list(
		&self,
		user_name: Option<&str>,
		option: Option<RepoListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = if let Some(name) = user_name {
			format!("{}/users/{}/repos", api_url, name)
		} else {
			format!("{}/user/repos", api_url)
		};
		let client = self.0.http();
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
		let res = request.query(&params).send().await?.json::<Vec<Json>>().await?;
		Ok(res.into_iter().map(Into::into).collect())
	}
}

impl From<Json<'_>> for UserInfo {
	fn from(json_value: Json<'_>) -> Self {
		let user_info = json_value.as_inner();
		Self {
			login: user_info.get("login").and_then(|v| v.as_str()).unwrap().to_string(),
			name: user_info
				.get("name")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			avatar_url: user_info.get("avatar_url").and_then(|v| v.as_str()).unwrap().to_string(),
			email: user_info
				.get("email")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			followers: user_info.get("followers").and_then(|v| v.as_u64()).unwrap(),
			following: user_info.get("following").and_then(|v| v.as_u64()).unwrap(),
			public_repo_count: user_info.get("repo_count").and_then(|v| v.as_u64()).unwrap_or(0),
		}
	}
}

impl From<Json<'_>> for Contribution {
	fn from(value: Json<'_>) -> Self {
		value
			.as_inner()
			.as_object()
			.unwrap()
			.iter()
			.map(|(date, count)| ContributionDay {
				date: date.parse().unwrap(),
				count: count.as_u64().unwrap() as u32,
			})
			.collect()
	}
}
