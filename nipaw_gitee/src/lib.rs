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

#[derive(Debug)]
pub(crate) struct GiteeConfig {
	pub(crate) token: Option<String>,
	pub(crate) base_url: String,
	pub(crate) api_url: String,
}

impl Default for GiteeConfig {
	fn default() -> Self {
		Self {
			token: None,
			base_url: "https://gitee.com".to_string(),
			api_url: "https://gitee.com/api/v5".to_string(),
		}
	}
}

impl GiteeConfig {
	/// 设置访问令牌
	pub fn set_token(&mut self, token: &str) {
		self.token = Some(token.to_string());
	}
}

#[derive(Debug, Default)]
pub struct GiteeClient {
	pub(crate) config: GiteeConfig,
}

impl GiteeClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for GiteeClient {
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
		let request =
			HTTP_CLIENT.get(url).query(&[("access_token", token.as_ref().unwrap().as_str())]);

		let resp = request.send().await?;
		let user_info: JsonValue = resp.json().await?;
		Ok(user_info.into())
	}

	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/users/{}", api_url, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let user_info: JsonValue = resp.json().await?;
		Ok(user_info.into())
	}

	async fn get_user_avatar_url(&self, user_name: &str) -> Result<String> {
		let base_url = &self.config.base_url;
		let url = format!("{}/users/{}/detail", base_url, user_name);
		let request = HTTP_CLIENT.get(url).header("Referer", base_url);
		let resp = request.send().await?;
		let user_info: JsonValue = resp.json().await?;
		let avatar_url = user_info
			.0
			.get("data")
			.and_then(|data| data.get("avatar_url"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string();
		Ok(avatar_url)
	}

	async fn get_user_contribution(&self, user_name: &str) -> Result<ContributionResult> {
		let base_url = &self.config.base_url;
		let url = format!("{}/{}", base_url, user_name);
		let request = HTTP_CLIENT
			.get(url)
			.header("X-Requested-With", "XMLHttpRequest")
			.header("Accept", "application/javascript");
		let resp = request.send().await?;
		let html: Html = resp.text().await?.into();
		Ok(html.into())
	}

	async fn get_org_info(&self, org_name: &str) -> Result<OrgInfo> {
		let (token, base_url) = (&self.config.token, &self.config.base_url);
		let url = format!("{}/orgs/{}", base_url, org_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let org_info: JsonValue = resp.json().await?;
		Ok(org_info.into())
	}

	async fn get_org_repos(
		&self,
		org_name: &str,
		options: Option<OrgRepoListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/orgs/{}/repos", api_url, org_name);
		let mut request = HTTP_CLIENT.get(url);
		let mut params = HashMap::new();
		if let Some(token) = token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		if let Some(option) = options {
			let per_page = option.per_page.unwrap_or_default().min(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.send().await?;
		let repo_list: Vec<JsonValue> = resp.json().await?;
		Ok(repo_list.into_iter().map(|v| v.into()).collect())
	}

	async fn get_org_avatar_url(&self, org_name: &str) -> Result<String> {
		let base_url = &self.config.base_url;
		let url = format!("{}/{}", base_url, org_name);
		let request = HTTP_CLIENT.get(url);
		let resp = request.send().await?;
		let org_html: String = resp.text().await?;

		let document = scraper::Html::parse_document(&org_html);
		let selector = scraper::Selector::parse("img.avatar.current-group-avatar").unwrap();

		let element = document.select(&selector).next().unwrap();
		let src = element.value().attr("src").unwrap();
		let avatar_url = src.split('!').next().unwrap_or(src).to_string();
		Ok(avatar_url)
	}

	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/repos/{}/{}", api_url, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let repo_info: JsonValue = resp.json().await?;
		Ok(repo_info.into())
	}

	async fn get_user_repos(&self, option: Option<ReposListOptions>) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/user/repos", api_url);
		let request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			params.insert("access_token", token.to_owned());
		}

		params.insert("sort", "updated".to_string());

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
		let request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			params.insert("access_token", token.to_owned());
		};
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
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let mut commit_info: JsonValue = resp.json().await?;
		let author_avatar_url = commit_info
			.0
			.get("author")
			.and_then(|v| v.get("avatar_url"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string();
		let committer_avatar_url = commit_info
			.0
			.get("committer")
			.and_then(|v| v.get("avatar_url"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string();
		if let Some(author_obj) = commit_info
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("author"))
			.and_then(|author| author.as_object_mut())
		{
			author_obj.insert("avatar_url".to_string(), Value::String(author_avatar_url));
		}

		if let Some(committer_obj) = commit_info
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("committer"))
			.and_then(|committer| committer.as_object_mut())
		{
			committer_obj.insert("avatar_url".to_string(), Value::String(committer_avatar_url));
		}
		Ok(commit_info.into())
	}

	async fn get_commit_infos(
		&self,
		repo_path: (&str, &str),
		option: Option<CommitListOptions>,
	) -> Result<Vec<CommitInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/repos/{}/{}/commits", api_url, repo_path.0, repo_path.1);
		let request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			params.insert("access_token", token.to_owned());
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
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!(
			"{}/repos/{}/{}/collaborators/{}",
			api_url, repo_path.0, repo_path.1, user_name
		);
		let request = HTTP_CLIENT.put(url);

		let permission = match permission {
			Some(permission) => match permission {
				CollaboratorPermission::Admin => "admin".to_string(),
				CollaboratorPermission::Push => "push".to_string(),
				CollaboratorPermission::Pull => "pull".to_string(),
			},
			None => "pull".to_string(),
		};

		let body = serde_json::json!({
			"access_token": token.as_ref().unwrap(),
			"permission": permission,
		});

		let res = request
			.header(header::CONTENT_TYPE, "application/json")
			.body(body.to_string())
			.send()
			.await?
			.json::<JsonValue>()
			.await?;
		Ok(res.into())
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
			request = request.query(&[("access_token", token)]);
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
			request = request.query(&[("access_token", token)]);
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
		let url = format!("{}/repos/{}/issues/{}", api_url, repo_path.0, issue_number);
		let request = HTTP_CLIENT.patch(url).query(&[("access_token", token.as_ref().unwrap())]);
		let mut req_body: HashMap<&str, String> = HashMap::new();
		req_body.insert("repo", repo_path.1.to_string());
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
		let res = request.form(&req_body).send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}
}
