mod common;
mod middleware;

pub use nipaw_core::Client;

use crate::common::JsonValue;
use crate::middleware::{HeaderMiddleware, ResponseMiddleware};
use async_trait::async_trait;
use chrono::{Datelike, Local};
use futures::future::join_all;
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
use reqwest::{Proxy, header};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub(crate) struct CnbConfig {
	pub(crate) token: Option<String>,
	pub(crate) api_url: String,
	pub(crate) base_url: String,
}

impl Default for CnbConfig {
	fn default() -> Self {
		Self {
			token: None,
			base_url: "https://cnb.cool".to_string(),
			api_url: "https://api.cnb.cool".to_string(),
		}
	}
}

impl CnbConfig {
	/// 设置访问令牌
	pub fn set_token(&mut self, token: &str) {
		self.token = Some(token.to_string());
	}
}

#[derive(Debug)]
pub struct CnbClient {
	pub(crate) config: CnbConfig,
	pub(crate) client: RwLock<Arc<ClientWithMiddleware>>,
}

impl Default for CnbClient {
	fn default() -> Self {
		let client = reqwest::Client::new();
		Self {
			config: CnbConfig::default(),
			client: RwLock::new(Arc::new(
				ClientBuilder::new(client).with(HeaderMiddleware).with(ResponseMiddleware).build(),
			)),
		}
	}
}

impl CnbClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for CnbClient {
	fn set_token(&mut self, token: &str) -> Result<()> {
		if token.is_empty() {
			return Err(Error::TokenEmpty);
		}
		self.config.set_token(token);
		Ok(())
	}

	fn set_proxy(&mut self, proxy: &str) -> Result<()> {
		let client = reqwest::Client::builder().proxy(Proxy::all(proxy)?).build()?;
		*self.client.try_write().unwrap() = Arc::new(
			ClientBuilder::new(client).with(HeaderMiddleware).with(ResponseMiddleware).build(),
		);
		Ok(())
	}

	async fn get_user_info(&self) -> Result<UserInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/user", api_url);
		let client = self.client.read().await;
		let request = client.get(url).bearer_auth(token.as_ref().unwrap());
		let resp = request.send().await?;
		let mut user_info: JsonValue = resp.json().await?;

		if let Some(username) = user_info.0.get("username").and_then(|v| v.as_str()) {
			let avatar_url = self.get_user_avatar_url(username).await?;
			user_info
				.0
				.as_object_mut()
				.unwrap()
				.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(user_info.into())
	}

	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/users/{}", api_url, user_name);
		let client = self.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut user_info: JsonValue = resp.json().await?;

		if let Some(username) = user_info.0.get("username").and_then(|v| v.as_str()) {
			let avatar_url = self.get_user_avatar_url(username).await?;
			user_info
				.0
				.as_object_mut()
				.unwrap()
				.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(user_info.into())
	}

	async fn get_user_avatar_url(&self, user_name: &str) -> Result<String> {
		let base_url = &self.config.base_url;
		Ok(format!("{}/users/{}/avatar/l", base_url, user_name))
	}

	async fn get_user_contribution(&self, user_name: &str) -> Result<ContributionResult> {
		let base_url = &self.config.base_url;
		let year = Local::now().year();
		let url = format!("{}/users/{}/calendar?year={}", base_url, user_name, year);
		let client = self.client.read().await;
		let resp = client.get(url).header("Accept", " application/vnd.cnb.web+json").send().await?;
		let contribution_result: JsonValue = resp.json().await?;
		Ok(contribution_result.into())
	}

	async fn get_org_info(&self, org_name: &str) -> Result<OrgInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/{}", api_url, org_name);
		let client = self.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut org_info: JsonValue = resp.json().await?;

		if let Some(username) = org_info.0.get("login").and_then(|v| v.as_str()) {
			let avatar_url = self.get_user_avatar_url(username).await?;
			org_info
				.0
				.as_object_mut()
				.unwrap()
				.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(org_info.into())
	}

	async fn get_org_repos(
		&self,
		org_name: &str,
		option: Option<OrgRepoListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/{}/-/repos", api_url, org_name);
		let client = self.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let resp = request.send().await?;
		let repo_infos: Vec<JsonValue> = resp.json().await?;
		Ok(repo_infos.into_iter().map(|v| v.into()).collect())
	}

	async fn get_org_avatar_url(&self, org_name: &str) -> Result<String> {
		let base_url = &self.config.base_url;
		let url = format!("{}/{}/-/logos/l", base_url, org_name);
		Ok(url.to_string())
	}

	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/repos/{}/{}", api_url, repo_path.0, repo_path.1);
		let client = self.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut repo_info: JsonValue = resp.json().await?;
		let default_branch =
			get_repo_default_branch(client.clone(), &self.config, &repo_info, token.clone())
				.await?;
		repo_info
			.0
			.as_object_mut()
			.unwrap()
			.insert("default_branch".to_string(), Value::String(default_branch));
		Ok(repo_info.into())
	}

	async fn get_user_repos(&self, option: Option<ReposListOptions>) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/user/repos", api_url);
		let client = self.client.read().await;
		let mut request = client.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}

		params.insert("sort", "pushed".to_owned());

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|v| v.into()).collect())
	}

	async fn get_user_repos_with_name(
		&self,
		user_name: &str,
		option: Option<ReposListOptions>,
	) -> Result<Vec<RepoInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/users/{}/repos", api_url, user_name);
		let client = self.client.read().await;
		let mut request = client.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}

		params.insert("role", "owner".to_owned());

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().max(100);
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
			"{}/{}/{}/-/git/commits/{}",
			api_url,
			repo_path.0,
			repo_path.1,
			sha.unwrap_or("HEAD")
		);
		let client = self.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut commit_info: JsonValue = resp.json().await?;
		let author_name = commit_info
			.0
			.get("commit")
			.and_then(|commit| commit.as_object())
			.and_then(|commit_obj| commit_obj.get("author"))
			.and_then(|author| author.as_object())
			.and_then(|author_obj| author_obj.get("name"))
			.and_then(|name| name.as_str())
			.unwrap()
			.to_string();

		let committer_name = commit_info
			.0
			.get("commit")
			.and_then(|commit| commit.as_object())
			.and_then(|commit_obj| commit_obj.get("committer"))
			.and_then(|committer| committer.as_object())
			.and_then(|committer_obj| committer_obj.get("name"))
			.and_then(|name| name.as_str())
			.unwrap()
			.to_string();

		if let Some(author) = commit_info
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("author"))
			.and_then(|author| author.as_object_mut())
		{
			let avatar_url = self.get_user_avatar_url(author_name.as_str()).await?;
			author.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		if let Some(committer) = commit_info
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("committer"))
			.and_then(|committer| committer.as_object_mut())
		{
			let avatar_url = self.get_user_avatar_url(committer_name.as_str()).await?;
			committer.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(commit_info.into())
	}

	async fn get_commit_infos(
		&self,
		repo_path: (&str, &str),
		option: Option<CommitListOptions>,
	) -> Result<Vec<CommitInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/{}/{}/-/commits", api_url, repo_path.0, repo_path.1);
		let client = self.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().max(100);
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
		let url = format!("{}/{}/{}/-/members/{}", api_url, repo_path.0, repo_path.1, user_name);
		let client = self.client.read().await;
		let request = client.post(url).bearer_auth(token.as_ref().unwrap());
		let permission = match permission {
			Some(permission) => match permission {
				CollaboratorPermission::Admin => "Master",
				CollaboratorPermission::Push => "Developer",
				CollaboratorPermission::Pull => "Reporter",
			},
			None => "Guest",
		};

		let body = serde_json::json!({
			"access_level": permission.to_string(),
			"is_outside_collaborator": true,
		});
		let resp = request
			.header(header::CONTENT_TYPE, "application/json")
			.body(body.to_string())
			.send()
			.await?;
		let status_code = resp.status().as_u16();
		if status_code == 200 {
			let collaborator = CollaboratorResult {
				login: user_name.to_string(),
				avatar_url: self.get_user_avatar_url(user_name).await?,
			};
			Ok(collaborator)
		} else {
			Err(Error::NotFound)
		}
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
		let client = self.client.read().await;
		let url = format!("{}/{}/{}/-/issues", api_url, repo_path.0, repo_path.1);
		let request = client.post(url).bearer_auth(token.as_ref().unwrap());
		let mut req_body: HashMap<&str, String> = HashMap::new();
		req_body.insert("title", title.to_string());
		if let Some(body) = body {
			req_body.insert("body", body.to_string());
		}
		if let Some(option) = option {
			if let Some(labels) = option.labels {
				req_body.insert("labels", labels.join(","));
			}
			if let Some(assignees) = option.assignees {
				req_body.insert("assignees", assignees.join(","));
			}
		};

		let mut res = request.json(&req_body).send().await?.json::<JsonValue>().await?;
		let user_name = res
			.0
			.get("author")
			.and_then(|author| author.get("username"))
			.and_then(|username| username.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();
		let user_info = get_user_info(client.clone(), &self.config, &user_name).await?;
		res.0.as_object_mut().unwrap().insert("user".to_string(), serde_json::to_value(user_info)?);
		Ok(res.into())
	}

	async fn get_issue_info(
		&self,
		repo_path: (&str, &str),
		issue_number: &str,
	) -> Result<IssueInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/{}/{}/-/issues/{}", api_url, repo_path.0, repo_path.1, issue_number);
		let client = self.client.read().await;
		let request = client.post(url).bearer_auth(token.as_ref().unwrap());
		let mut res = request.send().await?.json::<JsonValue>().await?;
		let user_name = res
			.0
			.get("author")
			.and_then(|author| author.get("username"))
			.and_then(|username| username.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();
		let user_info = get_user_info(client.clone(), &self.config, &user_name).await?;
		res.0.as_object_mut().unwrap().insert("user".to_string(), serde_json::to_value(user_info)?);
		Ok(res.into())
	}

	async fn get_issue_list(
		&self,
		repo_path: (&str, &str),
		options: Option<IssueListOptions>,
	) -> Result<Vec<IssueInfo>> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}

		let url = format!("{}/{}/{}/-/issues", api_url, repo_path.0, repo_path.1);
		let client = self.client.read().await;
		let request = client.get(url).bearer_auth(token.as_ref().unwrap());
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(option) = options {
			let per_page = option.per_page.unwrap_or_default().max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
			if let Some(labels) = option.labels {
				params.insert("labels", labels.join(","));
			}
			if let Some(state) = option.state {
				let state_type = match state {
					StateType::Opened => "open",
					StateType::Closed => "closed",
				};
				params.insert("state", state_type.to_string());
			}
			if let Some(assignees) = option.assignee {
				params.insert("assignees", assignees);
			}
			if let Some(author) = option.creator {
				params.insert("authors", author);
			}
		};
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;

		let issues = join_all(res.into_iter().map(|mut issue_json| async {
			let user_name = issue_json
				.0
				.get("author")
				.and_then(|author| author.get("username"))
				.and_then(|username| username.as_str())
				.unwrap_or_default()
				.to_string();
			let user_info = get_user_info(client.clone(), &self.config, &user_name).await.unwrap();
			issue_json
				.0
				.as_object_mut()
				.unwrap()
				.insert("user".to_string(), serde_json::to_value(user_info).unwrap());

			issue_json
		}))
		.await
		.into_iter()
		.map(|v| v.into())
		.collect::<Vec<IssueInfo>>();
		Ok(issues)
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
		let url = format!("{}/{}/{}/-/issues/{}", api_url, repo_path.0, repo_path.1, issue_number);
		let client = self.client.read().await;
		let request = client.put(url).bearer_auth(token.as_ref().unwrap());
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
		let mut res = request.json(&req_body).send().await?.json::<JsonValue>().await?;
		let user_name = res
			.0
			.get("author")
			.and_then(|author| author.get("username"))
			.and_then(|username| username.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();
		let user_info = get_user_info(client.clone(), &self.config, &user_name).await?;
		res.0.as_object_mut().unwrap().insert("user".to_string(), serde_json::to_value(user_info)?);
		Ok(res.into())
	}
}

async fn get_repo_default_branch(
	client: Arc<ClientWithMiddleware>,
	config: &CnbConfig,
	repo_info: &JsonValue,
	token: Option<String>,
) -> Result<String> {
	let is_public = repo_info
		.0
		.get("visibility_level")
		.and_then(|v| v.as_str())
		.map(|s| s.to_lowercase() == "public")
		.unwrap_or(false);

	let (owner, repo) = (
		repo_info
			.0
			.get("owner")
			.and_then(|v| v.get("login"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string(),
		repo_info.0.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
	);
	if is_public {
		let url = format!(
			"{}/{}/{}/-/git/refs?page=1&page_size=5000&prefix=branch",
			config.base_url, owner, repo
		);

		let request = client.get(url).header("Accept", "application/vnd.cnb.web+json");
		let res = request.send().await?.json::<Vec<Value>>().await?;
		let default_branch = res
			.into_iter()
			.find(|branch| branch.get("is_head").and_then(|v| v.as_bool()).unwrap_or(false))
			.and_then(|branch| branch.get("ref").and_then(|v| v.as_str()).map(|s| s.to_string()))
			.map(|ref_str| ref_str.trim_start_matches("refs/heads/").to_string());
		Ok(default_branch.unwrap())
	} else {
		let url = format!("{}/repos/{}/{}/-/git/head", config.api_url, owner, repo);
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let repo_info: JsonValue = resp.json().await?;
		Ok(repo_info.0.get("name").and_then(|v| v.as_str()).unwrap().to_string())
	}
}

async fn get_user_info(
	client: Arc<ClientWithMiddleware>,
	config: &CnbConfig,
	user_name: &str,
) -> Result<UserInfo> {
	let url = format!("{}/users/{}", config.base_url, user_name);
	let request = client.get(url);
	let resp = request.send().await?.json::<JsonValue>().await?;
	Ok(resp.into())
}
