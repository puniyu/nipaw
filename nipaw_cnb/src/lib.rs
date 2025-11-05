mod client;
mod common;
mod middleware;

pub use nipaw_core::Client;

use crate::{
	client::{HTTP_CLIENT, PROXY_URL},
	common::JsonValue,
};
use async_trait::async_trait;
use chrono::{Datelike, Local};
use nipaw_core::option::CreateIssueOptions;
use nipaw_core::types::issue::IssueInfo;
use nipaw_core::{
	CollaboratorPermission, Result,
	error::Error,
	option::{CommitListOptions, OrgRepoListOptions, ReposListOptions},
	types::{
		collaborator::CollaboratorResult,
		commit::CommitInfo,
		org::OrgInfo,
		repo::RepoInfo,
		user::{ContributionResult, UserInfo},
	},
};
use reqwest::{Url, header};
use serde_json::Value;
use std::collections::HashMap;

const BASE_URL: &str = "https://cnb.cool";
const API_URL: &str = "https://api.cnb.cool";

#[derive(Debug, Default)]
pub struct CnbClient {
	pub token: Option<String>,
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
		self.token = Some(token.to_string());
		Ok(())
	}

	fn set_proxy(&mut self, proxy: &str) -> Result<()> {
		PROXY_URL.set(proxy.to_string()).unwrap();
		Ok(())
	}

	async fn get_user_info(&self) -> Result<UserInfo> {
		if self.token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/user", API_URL);
		let request = HTTP_CLIENT
			.get(url)
			.header("Authorization", format!("Bearer {}", self.token.as_ref().unwrap()));
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
		let url = format!("{}/users/{}", API_URL, user_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.header("Authorization", format!("Bearer {}", token));
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
		Ok(format!("{}/users/{}/avatar/l", BASE_URL, user_name))
	}

	async fn get_user_contribution(&self, user_name: &str) -> Result<ContributionResult> {
		let mut url = Url::parse(&format!("{}/users/{}/calendar", BASE_URL, user_name))?;
		let year = Local::now().year();
		url.query_pairs_mut().append_pair("year", &year.to_string());
		let resp =
			HTTP_CLIENT.get(url).header("Accept", " application/vnd.cnb.web+json").send().await?;
		let contribution_result: JsonValue = resp.json().await?;
		Ok(contribution_result.into())
	}

	async fn get_org_info(&self, org_name: &str) -> Result<OrgInfo> {
		let url = format!("{}/{}", API_URL, org_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
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
		let url = format!("{}/{}/-/repos", API_URL, org_name);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}
		let mut params: HashMap<&str, String> = HashMap::new();
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
		let url = format!("{}/{}/-/logos/l", BASE_URL, org_name);
		Ok(url.to_string())
	}

	async fn get_repo_info(&self, repo_path: (&str, &str)) -> Result<RepoInfo> {
		let url = format!("{}/repos/{}/{}", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut repo_info: JsonValue = resp.json().await?;
		let default_branch = get_repo_default_branch(&repo_info, self.token.clone()).await?;
		repo_info
			.0
			.as_object_mut()
			.unwrap()
			.insert("default_branch".to_string(), Value::String(default_branch));
		Ok(repo_info.into())
	}

	async fn get_user_repos(&self, option: Option<ReposListOptions>) -> Result<Vec<RepoInfo>> {
		let url = format!("{}/user/repos", API_URL);
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}

		params.insert("sort", "pushed".to_owned());

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
		let url = format!("{}/users/{}/repos", API_URL, user_name);
		let mut request = HTTP_CLIENT.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}

		params.insert("role", "owner".to_owned());

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
		let url = format!(
			"{}/{}/{}/-/git/commits/{}",
			API_URL,
			repo_path.0,
			repo_path.1,
			sha.unwrap_or("HEAD")
		);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
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
		let url = format!("{}/{}/{}/-/commits", API_URL, repo_path.0, repo_path.1);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = &self.token {
			request = request.bearer_auth(token);
		}
		let mut params: HashMap<&str, String> = HashMap::new();
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
		if self.token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/{}/{}/-/members/{}", API_URL, repo_path.0, repo_path.1, user_name);
		let request = HTTP_CLIENT.post(url).bearer_auth(self.token.as_ref().unwrap());
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
		if self.token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/{}/{}/-/issues", API_URL, repo_path.0, repo_path.1);
		let request = HTTP_CLIENT.post(url).bearer_auth(self.token.as_ref().unwrap());
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

		let mut res = request.json(&req_body).send().await?.json::<JsonValue>().await?;
		let user_name = res
			.0
			.get("author")
			.and_then(|author| author.get("username"))
			.and_then(|username| username.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();
		let user_info = get_user_info(&user_name).await?;
		if let Some(obj) = res.0.as_object_mut() {
			obj.insert("user".to_string(), serde_json::to_value(user_info).unwrap());
		}
		Ok(res.into())
	}
}

async fn get_repo_default_branch(repo_info: &JsonValue, token: Option<String>) -> Result<String> {
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
			BASE_URL, owner, repo
		);
		let request = HTTP_CLIENT.get(url).header("Accept", "application/vnd.cnb.web+json");
		let res = request.send().await?.json::<Vec<Value>>().await?;
		let default_branch = res
			.into_iter()
			.find(|branch| branch.get("is_head").and_then(|v| v.as_bool()).unwrap_or(false))
			.and_then(|branch| branch.get("ref").and_then(|v| v.as_str()).map(|s| s.to_string()))
			.map(|ref_str| ref_str.trim_start_matches("refs/heads/").to_string());
		Ok(default_branch.unwrap())
	} else {
		let url = format!("{}/repos/{}/{}/-/git/head", API_URL, owner, repo);
		let mut request = HTTP_CLIENT.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let repo_info: JsonValue = resp.json().await?;
		Ok(repo_info.0.get("name").and_then(|v| v.as_str()).unwrap().to_string())
	}
}

async fn get_user_info(user_name: &str) -> Result<UserInfo> {
	let url = format!("{}/user/{}", BASE_URL, user_name);
	let request = HTTP_CLIENT.get(url).header("Accept", "application/vnd.cnb.web+json");
	let resp = request.send().await?.json::<JsonValue>().await?;
	Ok(resp.into())
}
