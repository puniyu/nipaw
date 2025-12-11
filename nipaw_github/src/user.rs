use crate::common::{Html, JsonValue};
use crate::GitHubClientInner;
use async_trait::async_trait;
use nipaw_core::option::ReposListOptions;
use nipaw_core::types::{
	repo::RepoInfo,
	user::{ContributionResult, UserInfo},
};
use nipaw_core::{Error, User, Result};
use std::collections::HashMap;
use std::sync::Arc;

pub struct GitHubUser(pub(crate) Arc<GitHubClientInner>);

#[async_trait]
impl User for GitHubUser {
	async fn info(&self, user_name: Option<&str>) -> Result<UserInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
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
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn avatar_url(&self, user_name: Option<&str>) -> Result<String> {
		let (token, base_url) = (&self.0.config.token, &self.0.config.base_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		if let Some(user_name) = user_name {
			let url = format!("{}/{}", base_url, user_name);
			let client = self.0.client.read().await;
			let request = client.get(url).header("Accept", "image/*");
			let resp = request.send().await?;
			let html: Html = Html::from(resp.text().await?);
			let document = scraper::Html::parse_document(&html.0);

			let selector =
				scraper::Selector::parse("meta[name='octolytics-dimension-user_id']").unwrap();
			let user_id = document
				.select(&selector)
				.next()
				.and_then(|element| element.value().attr("content"))
				.map(|id| id.to_string())
				.unwrap();
			let avatar_url = format!("https://avatars.githubusercontent.com/u/{}?v=4", user_id);
			return Ok(avatar_url);
		}
		let info = self.info(None).await?;
		Ok(info.avatar_url)
	}

	async fn contribution(
		&self,
		user_name: Option<&str>,
	) -> Result<ContributionResult> {
		let (token, base_url) = (&self.0.config.token, &self.0.config.base_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		let user_name = if let Some(user_name) = user_name {
			user_name.to_string()
		} else {
			self.info(None).await?.login
		};
		let url = format!(
			"{}/{}?action=show&controller=profiles&tab=contributions&user_id={}",
			base_url, user_name, user_name
		);
		let client = self.0.client.read().await;
		let request = client
			.get(url)
			.header("X-Requested-With", "XMLHttpRequest")
			.header("Accept", "text/html");
		let resp = request.send().await?;
		let html: Html = resp.text().await?.into();
		Ok(html.into())
	}

	async fn repo_list(
		&self,
		user_name: Option<&str>,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = if let Some(name) = user_name {
			format!("{}/users/{}/repos", api_url, name)
		} else {
			format!("{}/user/repo", api_url)
		};
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}

		params.insert("sort", "pushed".to_string());
		params.insert("type", "owner".to_string());

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
