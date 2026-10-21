use crate::{Error, GitCodeClientInner, Json, Result};
use async_trait::async_trait;
use nipaw_core::{
	Issue,
	issue::{
		IssueCreateOptions, IssueInfo, IssueListOptions, IssueUpdateOptions, IssueUserInfo,
		LabelInfo, StateType,
	},
	repo::RepoPath,
};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GitCodeIssue(pub(crate) Arc<GitCodeClientInner>);

#[async_trait]
impl Issue for GitCodeIssue {
	type Err = Error;

	async fn create(
		&self,
		repo_path: RepoPath,
		title: &str,
		body: Option<&str>,
		option: Option<IssueCreateOptions>,
	) -> Result<IssueInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let client = self.0.http();
		let url = format!("{}/repos/{}/{}/issues", api_url, repo_path.owner, repo_path.repo);
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

		let res = request.json(&req_body).send().await?.json::<Json>().await?;
		Ok(res.into())
	}

	async fn info(&self, repo_path: RepoPath, issue_number: &str) -> Result<IssueInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!(
			"{}/repos/{}/{}/issues/{}",
			api_url, repo_path.owner, repo_path.repo, issue_number
		);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		};
		let res = request.send().await?.json::<Json>().await?;
		Ok(res.into())
	}

	async fn list(
		&self,
		repo_path: RepoPath,
		options: Option<IssueListOptions>,
	) -> Result<Vec<IssueInfo>> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/{}/issues", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		};
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(option) = options {
			let per_page = option.per_page.unwrap_or(30).max(100);
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
			if let Some(assignee) = option.assignee {
				params.insert("assignee", assignee);
			}
			if let Some(creator) = option.creator {
				params.insert("creator", creator);
			}
		};
		let res = request.query(&params).send().await?.json::<Vec<Json>>().await?;
		Ok(res.into_iter().map(Into::into).collect())
	}

	async fn update(
		&self,
		repo_path: RepoPath,
		issue_number: &str,
		options: Option<IssueUpdateOptions>,
	) -> Result<IssueInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/issues/{}", api_url, repo_path.owner, issue_number);
		let client = self.0.http();
		let request = client.patch(url).bearer_auth(token.as_ref().unwrap());
		let mut req_body: HashMap<&str, String> = HashMap::new();
		req_body.insert("repo", repo_path.repo.to_string());
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
		let res = request.json(&req_body).send().await?.json::<Json>().await?;
		Ok(res.into())
	}
}

impl From<Json<'_>> for IssueInfo {
	fn from(issue: Json<'_>) -> Self {
		let issue_info = issue.as_inner();
		let is_open =
			issue_info.get("state").and_then(|v| v.as_str()).map(|s| s == "open").unwrap_or(false);
		let user_info = issue_info.get("user").unwrap();
		let labels_info = issue_info.get("labels").unwrap();
		Self {
			number: issue_info.get("number").and_then(|v| v.as_str()).unwrap().to_string(),
			state: if is_open { StateType::Opened } else { StateType::Closed },
			title: issue_info.get("title").and_then(|v| v.as_str()).unwrap().to_string(),
			body: issue_info
				.get("body")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			labels: Json::new(labels_info).into(),
			user: Json::new(user_info).into(),
			created_at: issue_info
				.get("created_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
			updated_at: issue_info
				.get("updated_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
			closed_at: issue_info
				.get("closed_at")
				.and_then(|v| v.as_str())
				.and_then(|s| s.parse().ok()),
		}
	}
}

impl From<Json<'_>> for IssueUserInfo {
	fn from(user: Json<'_>) -> Self {
		let user_info = user.as_inner();
		Self {
			login: user_info.get("login").and_then(|v| v.as_str()).unwrap().to_string(),
			avatar_url: user_info.get("avatar_url").and_then(|v| v.as_str()).unwrap().to_string(),
			email: user_info
				.get("email")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
		}
	}
}

impl From<Json<'_>> for LabelInfo {
	fn from(label: Json<'_>) -> Self {
		let label_info = label.as_inner();
		Self {
			name: label_info.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
			color: label_info.get("color").and_then(|v| v.as_str()).unwrap().to_string(),
		}
	}
}

impl From<Json<'_>> for Vec<LabelInfo> {
	fn from(label: Json<'_>) -> Self {
		label
			.as_inner()
			.as_array()
			.map(|arr| arr.iter().map(|v| Json::new(v).into()).collect())
			.unwrap_or_default()
	}
}
