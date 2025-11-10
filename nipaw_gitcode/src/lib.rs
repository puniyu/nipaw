mod client;
mod common;
mod middleware;

pub use nipaw_core::Client;

use crate::{
	client::{HTTP_CLIENT, PROXY_URL},
	common::JsonValue,
};
use async_trait::async_trait;
use nipaw_core::{
	CollaboratorPermission, Result,
	error::Error,
	option::{
		CommitListOptions, CreateIssueOptions, IssueListOptions, OrgRepoListOptions,
		ReposListOptions,
	},
	types::{
		collaborator::CollaboratorResult,
		commit::CommitInfo,
		issue::{IssueInfo, StateType},
		org::OrgInfo,
		repo::RepoInfo,
		user::{ContributionResult, UserInfo},
	},
};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct GitCodeConfig {
	pub(crate) token: Option<String>,
	pub(crate) base_url: String,
	pub(crate) api_url: String,
	pub(crate) web_api_url: String,
}

impl Default for GitCodeConfig {
	fn default() -> Self {
		GitCodeConfig {
			token: None,
			base_url: "https://gitcode.com".to_string(),
			api_url: "https://api.gitcode.com/api/v5".to_string(),
			web_api_url: "https://web-api.gitcode.com".to_string(),
		}
	}
}

impl GitCodeConfig {
	/// 设置访问令牌
	pub fn set_token(&mut self, token: &str) {
		self.token = Some(token.to_string());
	}
}

#[derive(Debug, Default)]
pub struct GitCodeClient {
	pub(crate) config: GitCodeConfig,
}

impl GitCodeClient {
	pub fn new() -> Self {
		Self::default()
	}
}

#[async_trait]
impl Client for GitCodeClient {
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
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut user_info: JsonValue = resp.json().await?;
		if let Some(user) = user_info.0.as_object_mut() {
			let user_name = user.get("username").and_then(|v| v.as_str()).unwrap();
			let repo_count = get_user_repo_count(&self.config, user_name).await?;
			user.insert("repo_count".to_string(), Value::Number(repo_count.into()));
		}
		Ok(user_info.into())
	}

	async fn get_user_info_with_name(&self, user_name: &str) -> Result<UserInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/users/{}", api_url, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut user_info: JsonValue = resp.json().await?;
		if let Some(user) = user_info.0.as_object_mut() {
			let repo_count = get_user_repo_count(&self.config, user_name).await?;
			user.insert("repo_count".to_string(), Value::Number(repo_count.into()));
		}
		Ok(user_info.into())
	}

	async fn get_user_avatar_url(&self, user_name: &str) -> Result<String> {
		let (web_api_url, base_url) = (&self.config.web_api_url, &self.config.base_url);
		let url = format!("{}/uc/api/v1/user/setting/profile?username={}", web_api_url, user_name);
		let res =
			HTTP_CLIENT.get(url).header("Referer", base_url).send().await?.json::<Value>().await?;
		let avatar_url = res.get("avatar").and_then(|v| v.as_str()).unwrap().to_string();
		Ok(avatar_url)
	}

	async fn get_user_contribution(&self, user_name: &str) -> Result<ContributionResult> {
		let (web_api_url, base_url) = (&self.config.web_api_url, &self.config.base_url);
		let url = format!(
			"{}/uc/api/v1/events/{}/contributions?username={}",
			web_api_url, user_name, user_name
		);
		let request = HTTP_CLIENT.get(url);
		let res = request.header("Referer", base_url).send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn get_org_info(&self, org_name: &str) -> Result<OrgInfo> {
		let (token, web_api_url) = (&self.config.token, &self.config.web_api_url);
		let url = format!("{}/orgs/{}", web_api_url, org_name);
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
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|v| v.into()).collect())
	}

	async fn get_org_avatar_url(&self, org_name: &str) -> Result<String> {
		let (base_url, web_api_url) = (&self.config.base_url, &self.config.web_api_url);
		let url = format!("{}/api/v2/groups/{}", web_api_url, org_name);
		let res =
			HTTP_CLIENT.get(url).header("Referer", base_url).send().await?.json::<Value>().await?;
		let avatar_url = res.get("avatar").and_then(|v| v.as_str()).unwrap().to_string();
		Ok(avatar_url)
	}

	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo> {
		let (token, api_url) = (&self.config.token, &self.config.api_url);
		let url = format!("{}/repos/{}/{}", api_url, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
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
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();

		if let Some(token) = token {
			request = request.bearer_auth(token);
		}

		params.insert("sort", "pushed".to_string());

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or_default().max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or_default();
			params.insert("page", page.to_string());
		}
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|v| v.into()).collect())
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
		let resp = request.send().await?;
		let mut commit_info = resp.json::<JsonValue>().await?;

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
		let url = format!("{}/repos/{}/{}/commits", api_url, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}

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
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|v| v.into()).collect())
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
		let request = HTTP_CLIENT.put(url).bearer_auth(token.as_ref().unwrap());
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

		let resp = request.body(body.to_string()).send().await?;
		let mut collaborator = resp.json::<JsonValue>().await?;
		if let Some(obj) = collaborator.0.as_object_mut() {
			let avatar_url = self.get_user_avatar_url(user_name).await?;
			obj.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(collaborator.into())
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
		let request = HTTP_CLIENT.post(url).bearer_auth(token.as_ref().unwrap());
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
		issue_number: String,
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
}

async fn get_user_repo_count(config: &GitCodeConfig, user_name: &str) -> Result<u64> {
	let (base_url, web_api_url) = (&config.base_url, &config.web_api_url);
	let url = format!(
		"{}/uc/api/v1/events/{}/contributions?username={}",
		web_api_url, user_name, user_name
	);
	let request = HTTP_CLIENT.get(url).header("Referer", base_url);
	let resp = request.send().await?;
	let repo_info: JsonValue = resp.json().await?;
	let repo_count = repo_info.0.get("total").and_then(|total| total.as_u64()).unwrap_or(0);
	Ok(repo_count)
}
