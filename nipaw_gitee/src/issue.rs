use crate::common::JsonValue;
use crate::GiteeClientInner;
use async_trait::async_trait;
use nipaw_core::option::{CreateIssueOptions, IssueListOptions, UpdateIssueOptions};
use nipaw_core::types::issue::{IssueInfo, StateType};
use nipaw_core::{Error, Issue, Result};
use std::collections::HashMap;
use std::sync::Arc;

pub struct GiteeIssue(pub(crate) Arc<GiteeClientInner>);

#[async_trait]
impl Issue for GiteeIssue {
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
		let url = format!("{}/repos/{}/{}/issues", api_url, repo_path.0, repo_path.1);
		let client = self.0.client.read().await;
		let request = client.put(url).query(&[("access_token", token.as_ref().unwrap())]);
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
		let res = request.json(&req_body).send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn info(&self, repo_path: (&str, &str), issue_number: &str) -> Result<IssueInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url =
			format!("{}/repos/{}/{}/issues/{}", api_url, repo_path.0, repo_path.1, issue_number);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.query(&[("access_token", token)]);
		};
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn list(
		&self,
		repo_path: (&str, &str),
		options: Option<IssueListOptions>,
	) -> Result<Vec<IssueInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!("{}/repos/{}/{}/issues", api_url, repo_path.0, repo_path.1);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.query(&[("access_token", token)]);
		};
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(option) = options {
			let per_page = option.per_page.unwrap_or(30).max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or(1);
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
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|v| v.into()).collect())
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
		let url = format!("{}/repos/{}/issues/{}", api_url, repo_path.0, issue_number);
		let client = self.0.client.read().await;
		let request = client.patch(url).query(&[("access_token", token.as_ref().unwrap())]);
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
