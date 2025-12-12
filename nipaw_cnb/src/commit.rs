use crate::CnbClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::option::commit::ListOptions;
use nipaw_core::types::commit::CommitInfo;
use nipaw_core::{Commit, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct CnbCommit(pub(crate) Arc<CnbClientInner>);

#[async_trait]
impl Commit for CnbCommit {
	async fn info(&self, repo_path: (&str, &str), sha: Option<&str>) -> Result<CommitInfo> {
		let (token, api_url, base_url) =
			(&self.0.config.token, &self.0.config.api_url, &self.0.config.base_url);
		let url = format!(
			"{}/{}/{}/-/git/commits/{}",
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
			let avatar_url = format!("{}/users/{}/avatar/l", base_url, author_name);
			author.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		if let Some(committer) = commit_info
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("committer"))
			.and_then(|committer| committer.as_object_mut())
		{
			let avatar_url = format!("{}/users/{}/avatar/l", base_url, committer_name);
			committer.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(commit_info.into())
	}

	async fn list(
		&self,
		repo_path: (&str, &str),
		option: Option<ListOptions>,
	) -> Result<Vec<CommitInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!("{}/{}/{}/-/commits", api_url, repo_path.0, repo_path.1);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let mut params: HashMap<&str, String> = HashMap::new();
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
