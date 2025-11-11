mod client;
mod common;
mod middleware;

pub use nipaw_core::Client;

use crate::{
	client::{HTTP_CLIENT, PROXY_URL},
	common::{Html, JsonValue},
};
use async_trait::async_trait;
use nipaw_core::{
	Result,
	error::Error,
	option::{
		CommitListOptions, CreateIssueOptions, IssueListOptions, OrgRepoListOptions,
		ReposListOptions, UpdateIssueOptions,
	},
	types::{
		collaborator::{CollaboratorPermission, CollaboratorResult},
		commit::CommitInfo,
		issue::{IssueInfo, StateType},
		org::OrgInfo,
		repo::RepoInfo,
		user::{ContributionResult, UserInfo},
	},
};
use reqwest::header;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct GitHubConfig {
	pub(crate) token: Option<String>,
	pub(crate) api_url: String,
	pub(crate) base_url: String,
}

impl Default for GitHubConfig {
	fn default() -> Self {
		Self {
			token: None,
			base_url: "https://github.com".to_string(),
			api_url: "https://api.github.com".to_string(),
		}
	}
}

impl GitHubConfig {
	/// 设置访问令牌
	pub fn set_token(&mut self, token: &str) {
		self.token = Some(token.to_string());
	}
	/// 设置 GitHub API 的 URL
	pub fn set_api_url(&mut self, api_url: String) {
		self.api_url = api_url;
	}

	/// 设置 GitHub 基础 URL
	pub fn set_base_url(&mut self, base_url: String) {
		self.base_url = base_url;
	}
}

#[derive(Debug, Default)]
pub struct GitHubClient {
	pub(crate) config: GitHubConfig,
}

impl GitHubClient {
	pub fn new() -> Self {
		Self::default()
	}

	/// 设置反向代理
	pub fn set_reverse_proxy(&mut self, url: &str) {
		let clean_url = url.trim_end_matches('/');
		let api_url = format!("{}/{}", clean_url, self.config.api_url);
		let base_url = format!("{}/{}", clean_url, self.config.base_url);
		self.config.set_api_url(api_url);
		self.config.set_base_url(base_url);
	}
}

#[async_trait]
impl Client for GitHubClient {
	fn set_token(&mut self, token: &str) -> Result<()> {
		if token.is_empty() {
			return Err(Error::TokenEmpty);
		}
		self.config.set_token(token);
		Ok(())
	}

	fn set_proxy(&mut self, proxy: &str) -> Result<()> {
		PROXY_URL.set(proxy.to_string()).unwrap();
		Ok(())
	}

	async fn get_user_info(&self) -> Result<UserInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/user", api_url);
		let request = HTTP_CLIENT.get(url).bearer_auth(token.as_ref().unwrap());
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/users/{}", api_url, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn get_user_avatar_url(&self, user_name: &str) -> Result<String> {
		let base_url = self.config.base_url.as_str();
		let url = format!("{}/{}", base_url, user_name);
		let request = HTTP_CLIENT.get(url).header("Accept", "image/*");
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
		Ok(avatar_url)
	}

	async fn get_user_contribution(&self, user_name: &str) -> Result<ContributionResult> {
		let base_url = &self.config.base_url;
		let url = format!(
			"{}/{}?action=show&controller=profiles&tab=contributions&user_id={}",
			base_url, user_name, user_name
		);

		let request = HTTP_CLIENT
			.get(url)
			.header("X-Requested-With", "XMLHttpRequest")
			.header("Accept", "text/html");
		let resp = request.send().await?;
		let html: Html = resp.text().await?.into();
		Ok(html.into())
	}

	async fn get_org_info(&self, org_name: &str) -> Result<OrgInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/orgs/{}", api_url, org_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn get_org_repos(
		&self,
		org_name: &str,
		option: Option<OrgRepoListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/orgs/{}/repos", api_url, org_name);
		let mut request = HTTP_CLIENT.get(url);
		let mut params = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().min(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.send().await?;
		let repo_infos: Vec<JsonValue> = resp.json().await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	async fn get_org_avatar_url(&self, org_name: &str) -> Result<String> {
		let api_url = self.config.api_url.as_str();
		let url = format!("{}/orgs/{}", api_url, org_name);
		let request = HTTP_CLIENT.get(url);
		let resp = request.send().await?;
		let org_html = resp.text().await?;

		let document = scraper::Html::parse_document(&org_html);
		let selector = scraper::Selector::parse("meta[name='hovercard-subject-tag']").unwrap();
		let element = document.select(&selector).next().unwrap();
		let org_id = element.value().attr("content").unwrap();
		let avatar_url = format!("https://avatars.githubusercontent.com/u/{}?v=4", org_id);
		Ok(avatar_url)
	}

	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/repos/{}/{}", api_url, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let repo_info: JsonValue = resp.json().await?;
		Ok(repo_info.into())
	}

	async fn get_user_repos(&self, option: Option<ReposListOptions>) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/user/repos", api_url);
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}

		params.insert("sort", "pushed".to_string());

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().min(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.query(&params).send().await?;
		let repo_infos: Vec<JsonValue> = resp.json().await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	async fn get_user_repos_with_name(
		&self,
		user_name: &str,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/users/{}/repos", api_url, user_name);
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		params.insert("sort", "pushed".to_string());

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().min(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.query(&params).send().await?;
		let repo_infos: Vec<JsonValue> = resp.json().await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	async fn get_commit_info(
		&self,
		repo_path: (&str, &str),
		sha: Option<&str>,
	) -> Result<CommitInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!(
			"{}/repos/{}/{}/commits/{}",
			api_url,
			repo_path.0,
			repo_path.1,
			sha.unwrap_or("HEAD")
		);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let mut res = request.send().await?.json::<JsonValue>().await?;
		let author_avatar_url = res
			.0
			.get("author")
			.and_then(|v| v.get("avatar_url"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string();
		let committer_avatar_url = res
			.0
			.get("committer")
			.and_then(|v| v.get("avatar_url"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string();
		if let Some(author_obj) = res
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("author"))
			.and_then(|author| author.as_object_mut())
		{
			author_obj.insert("avatar_url".to_string(), Value::String(author_avatar_url));
		}

		if let Some(committer_obj) = res
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("committer"))
			.and_then(|committer| committer.as_object_mut())
		{
			committer_obj.insert("avatar_url".to_string(), Value::String(committer_avatar_url));
		}
		Ok(res.into())
	}

	async fn get_commit_infos(
		&self,
		repo_path: (&str, &str),
		option: Option<CommitListOptions>,
	) -> Result<Vec<CommitInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/repos/{}/{}/commits", api_url, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().min(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
			if let Some(sha) = option.sha {
				params.insert("sha", sha.to_string());
			}
			if let Some(author) = option.author {
				params.insert("author", author.to_string());
			}
			if let Some(since) = option.since {
				params.insert("since", since.to_rfc3339());
			}
			if let Some(until) = option.until {
				params.insert("until", until.to_rfc3339());
			}
		}
		let resp = request.query(&params).send().await?;
		let commit_infos: Vec<JsonValue> = resp.json().await?;
		Ok(commit_infos.into_iter().map(|v| v.into()).collect())
	}

	async fn add_repo_collaborator(
		&self,
		repo_path: (&str, &str),
		user_name: &str,
		permission: Option<CollaboratorPermission>,
	) -> Result<CollaboratorResult> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!(
			"{}/repos/{}/{}/collaborators/{}",
			api_url, repo_path.0, repo_path.1, user_name
		);
		let mut request = HTTP_CLIENT.put(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let permission = match permission {
			Some(permission) => match permission {
				CollaboratorPermission::Admin => "admin".to_string(),
				CollaboratorPermission::Push => "push".to_string(),
				CollaboratorPermission::Pull => "pull".to_string(),
			},
			None => "pull".to_string(),
		};

		let body = serde_json::json!({
			"permission": permission,
		});
		let resp = request
			.header(header::CONTENT_TYPE, "application/json")
			.body(body.to_string())
			.send()
			.await?;
		let collaborator_result: JsonValue = resp.json().await?;
		Ok(collaborator_result.into())
	}

	async fn create_issue(
		&self,
		repo_path: (&str, &str),
		title: &str,
		body: Option<&str>,
		option: Option<CreateIssueOptions>,
	) -> Result<IssueInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/{}/issues", api_url, repo_path.0, repo_path.1);
		let request = HTTP_CLIENT.put(url).query(&[("access_token", token.as_ref().unwrap())]);
		let mut req_body: HashMap<&str, String> = HashMap::new();
		req_body.insert("title", title.to_string());
		if let Some(body) = body {
			req_body.insert("body", body.to_string());
		}
		if let Some(option) = option {
			if !option.labels.is_empty() {
				req_body.insert("labels", option.labels.join(","));
			}
			if !option.assignees.is_empty() {
				req_body.insert("assignees", option.assignees.join(","));
			}
		};
		let res = request.json(&req_body).send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn get_issue_info(
		&self,
		repo_path: (&str, &str),
		issue_number: &str,
	) -> Result<IssueInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url =
			format!("{}/repos/{}/{}/issues/{}", api_url, repo_path.0, repo_path.1, issue_number);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		};
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn get_issue_list(
		&self,
		repo_path: (&str, &str),
		options: Option<IssueListOptions>,
	) -> Result<Vec<IssueInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/repos/{}/{}/issues", api_url, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		};
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(option) = options {
			let per_page = option.per_page.unwrap_or_default().max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
			if !option.labels.is_empty() {
				params.insert("labels", option.labels.join(","));
			}
			if let Some(state) = option.state {
				let state_type = match state {
					StateType::Opened => "open",
					StateType::Closed => "closed",
				};
				params.insert("state", state_type.to_string());
			}
			if let Some(assignee) = option.assignee {
				params.insert("assignee", assignee);
			}
			if let Some(creator) = option.creator {
				params.insert("creator", creator);
			}
		};
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|v| v.into()).collect())
	}

	async fn update_issue(
		&self,
		repo_path: (&str, &str),
		issue_number: &str,
		options: Option<UpdateIssueOptions>,
	) -> Result<IssueInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url =
			format!("{}/repos/{}/{}/issues/{}", api_url, repo_path.0, repo_path.1, issue_number);
		let request = HTTP_CLIENT.patch(url).bearer_auth(token.as_ref().unwrap());
		let mut req_body: HashMap<&str, String> = HashMap::new();
		if let Some(option) = options {
			if let Some(title) = option.title {
				req_body.insert("title", title);
			}
			if let Some(body) = option.body {
				req_body.insert("body", body);
			}
			if let Some(state) = option.state {
				let state_type = match state {
					StateType::Opened => "open",
					StateType::Closed => "closed",
				};
				req_body.insert("state", state_type.to_string());
			}
		};
		let res = request.json(&req_body).send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}
}
