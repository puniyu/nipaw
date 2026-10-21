use crate::{CnbClientInner, Error, Json, Result};
use async_trait::async_trait;
use chrono::{Datelike, Local};
use nipaw_core::{
	User,
	repo::{RepoInfo, RepoListOptions},
	user::{Contribution, ContributionDay, UserInfo},
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CnbUser(pub(crate) Arc<CnbClientInner>);

#[async_trait]
impl User for CnbUser {
	type Err = Error;

	async fn info(&self, user_name: Option<&str>) -> Result<UserInfo> {
		let config = self.0.config();
		let (token, api_url, base_url) = (&config.token, &config.api_url, &config.base_url);
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

		if let Some(username) = user_info.as_inner().get("username").and_then(|v| v.as_str()) {
			let avatar_url = format!("{}/users/{}/avatar/l", base_url, username);
			user_info
				.as_inner_mut()
				.as_object_mut()
				.unwrap()
				.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(user_info.into())
	}

	async fn avatar_url(&self, user_name: Option<&str>) -> Result<String> {
		let config = self.0.config();
		let (token, base_url) = (&config.token, &config.base_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		if let Some(user_name) = user_name {
			return Ok(format!("{}/users/{}/avatar/l", base_url, user_name));
		}
		let info = self.info(None).await?;
		Ok(info.avatar_url)
	}

	async fn contribution(&self, user_name: Option<&str>) -> Result<Contribution> {
		let config = self.0.config();
		let (token, base_url) = (&config.token, &config.base_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let user_name = if let Some(user_name) = user_name {
			user_name.to_string()
		} else {
			self.info(None).await?.login
		};
		let year = Local::now().year();
		let url = format!("{}/users/{}/calendar?year={}", base_url, user_name, year);
		let client = self.0.http();
		let resp = client.get(url).header("Accept", " application/vnd.cnb.web+json").send().await?;
		let contribution_result: Json = resp.json().await?;
		Ok(contribution_result.into())
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
		let repo_infos: Vec<Json> = resp.json().await?;
		Ok(repo_infos.into_iter().map(Into::into).collect())
	}
}

impl From<Json<'_>> for UserInfo {
	fn from(json_value: Json<'_>) -> Self {
		let user_info = json_value.as_inner();
		let login = user_info.get("username").and_then(|v| v.as_str()).unwrap();
		let base_url = "https://cnb.cool";
		Self {
			login: login.to_string(),
			name: user_info
				.get("nickname")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			avatar_url: format!("{}/{}", base_url, login),
			email: user_info
				.get("email")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			followers: user_info.get("follower_count").and_then(|v| v.as_u64()).unwrap(),
			following: user_info.get("follow_count").and_then(|v| v.as_u64()).unwrap(),
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
			.map(|(date, data)| ContributionDay {
				date: jiff::civil::Date::strptime("%Y%m%d", date).unwrap(),
				count: data.get("score").and_then(|v| v.as_u64()).unwrap_or(0) as u32,
			})
			.collect()
	}
}
