use std::collections::HashMap;
use std::sync::Arc;
use nipaw_core::{Commit, Result};
use async_trait::async_trait;
use serde_json::Value;
use nipaw_core::option::CommitListOptions;
use nipaw_core::types::commit::CommitInfo;
use crate::common::JsonValue;
use crate::GitHubClientInner;

pub struct GitHubCommit(pub(crate) Arc<GitHubClientInner>);

#[async_trait]
impl Commit for GitHubCommit {
	async fn info(&self, repo_path: (&str, &str), sha: Option<&str>) -> Result<CommitInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!(
			"{}/repos/{}/{}/commits/{}",
			api_url,
			repo_path.0,
			repo_path.1,
			sha.unwrap_or("HEAD")
		);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
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
	
	async fn list(&self, repo_path: (&str, &str), option: Option<CommitListOptions>) -> Result<Vec<CommitInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!("{}/repos/{}/{}/commits", api_url, repo_path.0, repo_path.1);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		
		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or(30).max(100);
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
}