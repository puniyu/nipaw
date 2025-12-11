use crate::common::JsonValue;
use crate::{get_user_info, CnbClientInner};
use async_trait::async_trait;
use futures::future::join_all;
use nipaw_core::option::{CreateIssueOptions, IssueListOptions, UpdateIssueOptions};
use nipaw_core::types::issue::{IssueInfo, StateType};
use nipaw_core::{Error, Issue, Result};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CnbIssue(pub(crate) Arc<CnbClientInner>);

#[async_trait]
impl Issue for CnbIssue {
	async fn create(
		&self,
		repo_path: (&str, &str),
		title: &str,
		body: Option<&str>,
		option: Option<CreateIssueOptions>,
	) -> Result<IssueInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let client = self.0.client.read().await;
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
		let user_info = get_user_info(client.clone(), &self.0.config, &user_name).await?;
		res.0.as_object_mut().unwrap().insert("user".to_string(), serde_json::to_value(user_info)?);
		Ok(res.into())
	}

	async fn info(&self, repo_path: (&str, &str), issue_number: &str) -> Result<IssueInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/{}/{}/-/issues/{}", api_url, repo_path.0, repo_path.1, issue_number);
		let client = self.0.client.read().await;
		let request = client.get(url).bearer_auth(token.as_ref().unwrap());
		let mut res = request.send().await?.json::<JsonValue>().await?;
		let user_name = res
			.0
			.get("author")
			.and_then(|author| author.get("username"))
			.and_then(|username| username.as_str())
			.map(|s| s.to_string())
			.unwrap_or_default();
		let user_info = get_user_info(client.clone(), &self.0.config, &user_name).await?;
		res.0.as_object_mut().unwrap().insert("user".to_string(), serde_json::to_value(user_info)?);
		Ok(res.into())
	}

	async fn list(
		&self,
		repo_path: (&str, &str),
		options: Option<IssueListOptions>,
	) -> Result<Vec<IssueInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}

		let url = format!("{}/{}/{}/-/issues", api_url, repo_path.0, repo_path.1);
		let client = self.0.client.read().await;
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
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;

		let config = self.0.config.clone();
		let issues = join_all(res.into_iter().map(|mut issue_json| {
			let client = client.clone();
			let config = config.clone();
			async move {
				let user_name = issue_json
					.0
					.get("author")
					.and_then(|author| author.get("username"))
					.and_then(|username| username.as_str())
					.unwrap_or_default()
					.to_string();
				let user_info = get_user_info(client, &config, &user_name).await.unwrap();
				issue_json
					.0
					.as_object_mut()
					.unwrap()
					.insert("user".to_string(), serde_json::to_value(user_info).unwrap());

				issue_json
			}
		}))
		.await
		.into_iter()
		.map(|v| v.into())
		.collect::<Vec<IssueInfo>>();
		Ok(issues)
	}

	async fn update(
		&self,
		repo_path: (&str, &str),
		issue_number: &str,
		options: Option<UpdateIssueOptions>,
	) -> Result<IssueInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/{}/{}/-/issues/{}", api_url, repo_path.0, repo_path.1, issue_number);
		let client = self.0.client.read().await;
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
		let user_info = get_user_info(client.clone(), &self.0.config, &user_name).await?;
		res.0.as_object_mut().unwrap().insert("user".to_string(), serde_json::to_value(user_info)?);
		Ok(res.into())
	}
}
