use crate::GitCodeClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::option::commit::ListOptions;
use nipaw_core::types::commit::{CommitInfo, FileInfo};
use nipaw_core::types::repo::RepoPath;
use nipaw_core::{Commit, Error, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GitCodeCommit(pub(crate) Arc<GitCodeClientInner>);

impl GitCodeCommit {
	pub(crate) async fn get_file_info(
		&self,
		repo_path: RepoPath<'_>,
		base: &str,
		head: &str,
	) -> Result<Vec<FileInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let token = token.as_ref().ok_or(Error::TokenEmpty)?;
		let client = self.0.client.read().await;
		let url = format!(
			"{}/repos/{}/{}/compare/{}...{}",
			api_url, repo_path.0, repo_path.1, base, head
		);
		let resp = client.get(&url).query(&[("straight", true)]).bearer_auth(token).send().await?;
		let json = resp.json::<JsonValue>().await?;
		println!("res: {:#?}", json);
		let files = json
			.0
			.get("files")
			.and_then(|f| f.as_array())
			.map(|arr| arr.iter().map(|v| JsonValue(v.clone()).into()).collect())
			.unwrap_or_default();
		Ok(files)
	}
	pub(crate) async fn get_user_avatar_url(&self, user_name: &str) -> Result<String> {
		let (web_api_url, base_url) = (&self.0.config.web_api_url, &self.0.config.base_url);
		let client = self.0.client.read().await;
		let url = format!("{}/uc/api/v1/user/setting/profile?username={}", web_api_url, user_name);
		let res = client.get(url).header("Referer", base_url).send().await?.json::<Value>().await?;
		let default_avatar_url = "https://cdn-static.gitcode.com/doc/avatar-5.png";
		let avatar_url = res.get("avatar").and_then(|v| v.as_str()).unwrap_or(default_avatar_url);
		Ok(avatar_url.to_string())
	}
}
#[async_trait]
impl Commit for GitCodeCommit {
	async fn info(&self, repo_path: RepoPath<'_>, sha: Option<&str>) -> Result<CommitInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
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
			let avatar_url = self.get_user_avatar_url(&author_name).await?;
			author.insert("avatar_url".to_string(), Value::String(avatar_url));
		}

		if let Some(committer) = commit_info
			.0
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("committer"))
			.and_then(|committer| committer.as_object_mut())
		{
			let avatar_url = self.get_user_avatar_url(&committer_name).await?;
			committer.insert("avatar_url".to_string(), Value::String(avatar_url));
		};

		let commit_sha =
			commit_info.0.get("sha").and_then(|v| v.as_str()).unwrap_or(sha.unwrap_or("HEAD"));

		let parent_sha = commit_info
			.0
			.get("parents")
			.and_then(|p| p.as_array())
			.and_then(|arr| arr.first())
			.and_then(|p| p.get("sha"))
			.and_then(|v| v.as_str())
			.unwrap();

		let files = self.get_file_info(repo_path, parent_sha, commit_sha).await?;

		commit_info
			.0
			.as_object_mut()
			.unwrap()
			.insert("files".to_string(), serde_json::to_value(&files).unwrap());
		Ok(commit_info.into())
	}

	async fn list(
		&self,
		repo_path: RepoPath<'_>,
		option: Option<ListOptions>,
	) -> Result<Vec<CommitInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
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
		let res = request.query(&params).send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|v| v.into()).collect())
	}
}
