use crate::{Error, GitHubState, Html, Json, Result};
use async_trait::async_trait;
use nipaw_core::User;
use nipaw_core::repo::{RepoInfo, RepoListOptions};
use nipaw_core::user::{Contribution, ContributionDay, UserInfo};
use scraper::Selector;
use std::collections::HashMap;
use std::sync::Arc;

type State = Arc<GitHubState>;

#[derive(Debug, Clone)]
pub struct GitHubUser(State);

impl GitHubUser {
	pub(crate) fn new(state: impl Into<State>) -> Self {
		Self(state.into())
	}
}

#[async_trait]
impl User for GitHubUser {
	type Err = crate::Error;

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
			request = request.bearer_auth(token);
		}
		let res = request.send().await?.json::<Json>().await?;
		Ok(res.into())
	}

	async fn avatar_url(&self, user_name: Option<&str>) -> Result<String> {
		let config = self.0.config();
		let (token, web_url) = (&config.token, &config.web_url);
		if token.is_none() && user_name.is_none() {
			return Err(Error::TokenEmpty);
		}
		if let Some(user_name) = user_name {
			let url = format!("{}/{}", web_url, user_name);
			let client = self.0.http();
			let request = client.get(url).header("Accept", "image/*");
			let resp = request.send().await?;
			let html: Html = Html::from(resp.text().await?);
			let document = scraper::Html::parse_document(html.as_inner());

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

	async fn contribution(&self, user_name: Option<&str>) -> Result<Contribution> {
		let config = self.0.config();
		let (token, web_url) = (&config.token, &config.web_url);
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
			web_url, user_name, user_name
		);
		let client = self.0.http();
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
			format!("{}/user/repo", api_url)
		};
		let client = self.0.http();
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

		let cell_selector = Selector::parse("td.Contribution-day").unwrap();
		let total_selector = Selector::parse("h2#js-contribution-activity-description").unwrap();
		let tooltip_selector = Selector::parse("tool-tip").unwrap();

		let total = document
			.select(&total_selector)
			.next()
			.map(|element| {
				element
					.text()
					.collect::<String>()
					.split_whitespace()
					.next()
					.unwrap_or("0")
					.replace(",", "")
					.parse::<u32>()
					.unwrap_or(0)
			})
			.unwrap_or(0);

		let mut calendar: Contribution = document
			.select(&cell_selector)
			.filter_map(|element| {
				let date_str = element.value().attr("data-date")?;
				let id = element.value().attr("id")?;

				let date = date_str.parse().ok()?;
				let mut count = 0;
				if let Some(tooltip) =
					document.select(&tooltip_selector).find(|e| e.value().attr("for") == Some(id))
				{
					let tooltip_text = tooltip.text().collect::<String>();
					if !tooltip_text.contains("No contributions") {
						let contributions_str = tooltip_text.split_whitespace().next().unwrap();
						count = contributions_str.parse().unwrap_or(1);
					}
				}

				Some(ContributionDay { date, count })
			})
			.collect();

		calendar.total = total;
		calendar
	}
}
