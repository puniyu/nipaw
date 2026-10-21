use crate::{CnbClientInner, Error, Json, Result, get_user_info};
use async_trait::async_trait;
use futures::future::join_all;
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
pub struct CnbIssue(pub(crate) Arc<CnbClientInner>);

#[async_trait]
impl Issue for CnbIssue {
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
		let url = format!("{}/{}/{}/-/issues", api_url, repo_path.owner, repo_path.repo);
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

		let mut res = request.json(&req_body).send().await?.json::<Json>().await?;
		let user_name = res
			.as_inner()
			.get("author")
			.and_then(|author| author.get("username"))
			.and_then(|username| username.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();
		let user_info = get_user_info(client.clone(), &config, &user_name).await?;
		res.as_inner_mut()
			.as_object_mut()
			.unwrap()
			.insert("user".to_string(), serde_json::to_value(user_info)?);
		Ok(res.into())
	}

	async fn info(&self, repo_path: RepoPath, issue_number: &str) -> Result<IssueInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url =
			format!("{}/{}/{}/-/issues/{}", api_url, repo_path.owner, repo_path.repo, issue_number);
		let client = self.0.http();
		let request = client.get(url).bearer_auth(token.as_ref().unwrap());
		let mut res = request.send().await?.json::<Json>().await?;
		let user_name = res
			.as_inner()
			.get("author")
			.and_then(|author| author.get("username"))
			.and_then(|username| username.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();
		let user_info = get_user_info(client.clone(), &config, &user_name).await?;
		res.as_inner_mut()
			.as_object_mut()
			.unwrap()
			.insert("user".to_string(), serde_json::to_value(user_info)?);
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

		let url = format!("{}/{}/{}/-/issues", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.http();
		let request = client.get(url).bearer_auth(token.as_ref().unwrap());
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
			if let Some(assignees) = option.assignee {
				params.insert("assignees", assignees);
			}
			if let Some(author) = option.creator {
				params.insert("authors", author);
			}
		};
		let res = request.query(&params).send().await?.json::<Vec<Json>>().await?;

		let config = self.0.config();
		let issues = join_all(res.into_iter().map(|mut issue_json| {
			let client = client.clone();
			let config = config.clone();
			async move {
				let user_name = issue_json
					.as_inner()
					.get("author")
					.and_then(|author| author.get("username"))
					.and_then(|username| username.as_str())
					.unwrap_or_default()
					.to_string();
				let user_info = get_user_info(client, &config, &user_name).await.unwrap();
				issue_json
					.as_inner_mut()
					.as_object_mut()
					.unwrap()
					.insert("user".to_string(), serde_json::to_value(user_info).unwrap());

				issue_json
			}
		}))
		.await
		.into_iter()
		.map(Into::into)
		.collect::<Vec<IssueInfo>>();
		Ok(issues)
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
		let url =
			format!("{}/{}/{}/-/issues/{}", api_url, repo_path.owner, repo_path.repo, issue_number);
		let client = self.0.http();
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
		let mut res = request.json(&req_body).send().await?.json::<Json>().await?;
		let user_name = res
			.as_inner()
			.get("author")
			.and_then(|author| author.get("username"))
			.and_then(|username| username.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();
		let user_info = get_user_info(client.clone(), &config, &user_name).await?;
		res.as_inner_mut()
			.as_object_mut()
			.unwrap()
			.insert("user".to_string(), serde_json::to_value(user_info)?);
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
		let login = user_info.get("login").and_then(|v| v.as_str()).unwrap().to_string();
		let base_url = crate::CnbConfig::default().base_url;
		Self {
			login: login.clone(),
			avatar_url: format!("{}/users/{}/avatar/l", base_url, login),
			email: Some(user_info.get("email").and_then(|v| v.as_str()).unwrap().to_string()),
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
