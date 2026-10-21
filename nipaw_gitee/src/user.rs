use crate::{Error, GiteeClientInner, Html, Json, Result};
use async_trait::async_trait;
use nipaw_core::{
	User,
	repo::{RepoInfo, RepoListOptions},
	user::{Contribution, ContributionDay, UserInfo},
};
use scraper::Selector;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GiteeUser(pub(crate) Arc<GiteeClientInner>);

#[async_trait]
impl User for GiteeUser {
	type Err = Error;

	async fn info(&self, user_name: Option<&str>) -> Result<UserInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() && user_name.is_none() {
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
			request = request.query(&[("access_token", token.as_str())]);
		}
		let res = request.send().await?.json::<Json>().await?;
		Ok(res.into())
	}

	async fn avatar_url(&self, user_name: Option<&str>) -> Result<String> {
		let config = self.0.config();
		let (token, base_url) = (&config.token, &config.base_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		if let Some(user_name) = user_name {
			let url = format!("{}/users/{}/detail", base_url, user_name);
			let client = self.0.http();
			let request = client.get(url).header("Referer", base_url);
			let resp = request.send().await?;
			let user_info: Json = resp.json().await?;
			let avatar_url = user_info
				.as_inner()
				.get("data")
				.and_then(|data| data.get("avatar_url"))
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string();
			return Ok(avatar_url);
		}
		let info = self.info(None).await?;
		Ok(info.avatar_url)
	}

	async fn contribution(&self, user_name: Option<&str>) -> Result<Contribution> {
		let config = self.0.config();
		let (token, base_url) = (&config.token, &config.base_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		let user_name = if let Some(user_name) = user_name {
			user_name.to_string()
		} else {
			self.info(None).await?.login
		};
		let url = format!("{}/{}", base_url, user_name);
		let client = self.0.http();
		let request = client
			.get(url)
			.header("X-Requested-With", "XMLHttpRequest")
			.header("Accept", "application/javascript");
		let resp = request.send().await?;
		let html: Html = resp.text().await?.into();
		Ok(html.into())
	}

	async fn repo_list(
		&self,
		user_name: Option<&str>,
		option: Option<RepoListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = if let Some(name) = user_name {
			format!("{}/users/{}/repos", api_url, name)
		} else {
			format!("{}/user/repos", api_url)
		};
		let client = self.0.http();
		let request = client.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			params.insert("access_token", token.to_owned());
		}

		params.insert("sort", "pushed".to_string());
		params.insert("type", "owner".to_string());

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or(30).max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or(1);
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
			public_repo_count: user_info.get("public_repos").and_then(|v| v.as_u64()).unwrap(),
		}
	}
}

impl From<Html> for Contribution {
	fn from(value: Html) -> Self {
		let html = value.as_inner();
		let document = scraper::Html::parse_document(html);

		let selector = Selector::parse("div.right-side div.box").unwrap();

		document
			.select(&selector)
			.filter_map(|element| {
				let data_content = element.value().attr("data-content")?;
				let date_str = element.value().attr("date")?;

				let count = data_content
					.split('个')
					.next()?
					.rsplit(':')
					.next()?
					.parse::<u32>()
					.unwrap_or(0);
				let date = jiff::civil::Date::strptime("%Y%m%d", date_str).ok()?;

				Some(ContributionDay { date, count })
			})
			.collect()
	}
}
