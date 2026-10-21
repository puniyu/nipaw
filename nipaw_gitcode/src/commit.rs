use crate::{Error, GitCodeClientInner, Json, Result, format_datetime};
use async_trait::async_trait;
use nipaw_core::{
	Commit,
	commit::{
		CommitData, CommitInfo, CommitListInfo, CommitListOptions, FileInfo, FileStatus,
		GitUserInfo, StatsInfo,
	},
	repo::RepoPath,
};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GitCodeCommit(pub(crate) Arc<GitCodeClientInner>);

impl GitCodeCommit {
	pub(crate) async fn get_file_info(
		&self,
		repo_path: RepoPath,
		base: &str,
		head: &str,
	) -> Result<Vec<FileInfo>> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		let token = token.as_ref().ok_or(Error::TokenEmpty)?;
		let client = self.0.http();
		let url = format!(
			"{}/repos/{}/{}/compare/{}...{}",
			api_url, repo_path.owner, repo_path.repo, base, head
		);
		let resp = client.get(&url).query(&[("straight", true)]).bearer_auth(token).send().await?;
		let json = resp.json::<Json>().await?;
		let files = json
			.as_inner()
			.get("files")
			.and_then(|f| f.as_array())
			.map(|arr| arr.iter().map(|v| Json::new(v).into()).collect())
			.unwrap_or_default();
		Ok(files)
	}
	pub(crate) async fn get_user_avatar_url(&self, user_name: &str) -> Result<String> {
		let config = self.0.config();
		let (web_api_url, base_url) = (&config.web_api_url, &config.base_url);
		let client = self.0.http();
		let url = format!("{}/uc/api/v1/user/setting/profile?username={}", web_api_url, user_name);
		let res = client.get(url).header("Referer", base_url).send().await?.json::<Value>().await?;
		let default_avatar_url = "https://cdn-static.gitcode.com/doc/avatar-5.png";
		let avatar_url = res.get("avatar").and_then(|v| v.as_str()).unwrap_or(default_avatar_url);
		Ok(avatar_url.to_string())
	}
}
#[async_trait]
impl Commit for GitCodeCommit {
	type Err = Error;

	async fn info(&self, repo_path: RepoPath, sha: Option<&str>) -> Result<CommitInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!(
			"{}/repos/{}/{}/commits/{}",
			api_url,
			repo_path.owner,
			repo_path.repo,
			sha.unwrap_or("HEAD")
		);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut commit_info = resp.json::<Json>().await?;

		let author_name = commit_info
			.as_inner()
			.get("commit")
			.and_then(|commit| commit.as_object())
			.and_then(|commit_obj| commit_obj.get("author"))
			.and_then(|author| author.as_object())
			.and_then(|author_obj| author_obj.get("name"))
			.and_then(|name| name.as_str())
			.unwrap()
			.to_string();

		let committer_name = commit_info
			.as_inner()
			.get("commit")
			.and_then(|commit| commit.as_object())
			.and_then(|commit_obj| commit_obj.get("committer"))
			.and_then(|committer| committer.as_object())
			.and_then(|committer_obj| committer_obj.get("name"))
			.and_then(|name| name.as_str())
			.unwrap()
			.to_string();

		if let Some(author) = commit_info
			.as_inner_mut()
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("author"))
			.and_then(|author| author.as_object_mut())
		{
			let avatar_url = self.get_user_avatar_url(&author_name).await?;
			author.insert("avatar_url".to_string(), Value::String(avatar_url));
		}

		if let Some(committer) = commit_info
			.as_inner_mut()
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("committer"))
			.and_then(|committer| committer.as_object_mut())
		{
			let avatar_url = self.get_user_avatar_url(&committer_name).await?;
			committer.insert("avatar_url".to_string(), Value::String(avatar_url));
		};

		let commit_sha = commit_info
			.as_inner()
			.get("sha")
			.and_then(|v| v.as_str())
			.unwrap_or(sha.unwrap_or("HEAD"));

		let parent_sha = commit_info
			.as_inner()
			.get("parents")
			.and_then(|p| p.as_array())
			.and_then(|arr| arr.first())
			.and_then(|p| p.get("sha"))
			.and_then(|v| v.as_str())
			.unwrap();

		let files = self.get_file_info(repo_path, parent_sha, commit_sha).await?;

		commit_info
			.as_inner_mut()
			.as_object_mut()
			.unwrap()
			.insert("files".to_string(), serde_json::to_value(&files)?);
		Ok(commit_info.into())
	}

	async fn list(
		&self,
		repo_path: RepoPath,
		option: Option<CommitListOptions>,
	) -> Result<Vec<CommitListInfo>> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/{}/commits", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.http();
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
				params.insert("since", format_datetime(since));
			}
			if let Some(until) = option.until {
				params.insert("until", format_datetime(until));
			}
		}
		let res = request.query(&params).send().await?.json::<Vec<Json>>().await?;
		Ok(res.into_iter().map(Into::into).collect())
	}
}

impl From<Json<'_>> for CommitInfo {
	fn from(value: Json<'_>) -> Self {
		let commit_info = value.as_inner();
		let commit_value = commit_info.get("commit").unwrap();
		let stats_value = commit_info.get("stats").unwrap();
		Self {
			sha: commit_info.get("sha").and_then(|v| v.as_str()).unwrap().to_string(),
			commit: Json::new(commit_value).into(),
			stats: Json::new(stats_value).into(),
			files: commit_info
				.get("files")
				.and_then(|s| s.as_array())
				.map(|arr| arr.iter().map(|v| Json::new(v).into()).collect())
				.unwrap(),
		}
	}
}

impl From<Json<'_>> for CommitListInfo {
	fn from(value: Json<'_>) -> Self {
		let commit_info = value.as_inner();
		let commit_value = commit_info.get("commit").unwrap();
		Self {
			sha: commit_info.get("sha").and_then(|v| v.as_str()).unwrap().to_string(),
			commit: Json::new(commit_value).into(),
		}
	}
}

impl From<Json<'_>> for CommitData {
	fn from(value: Json<'_>) -> Self {
		let commit_data = value.as_inner();
		let author_value = commit_data.get("author").unwrap();
		let committer_value = commit_data.get("committer").unwrap();
		Self {
			author: Json::new(author_value).into(),
			committer: Json::new(committer_value).into(),
			message: commit_data.get("message").and_then(|v| v.as_str()).unwrap().to_string(),
		}
	}
}

impl From<Json<'_>> for GitUserInfo {
	fn from(value: Json<'_>) -> Self {
		let user_info = value.as_inner();
		Self {
			name: user_info.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
			email: user_info
				.get("email")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			avatar_url: user_info.get("avatar_url").and_then(|v| v.as_str()).unwrap().to_string(),
			date: user_info
				.get("date")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
		}
	}
}

impl From<Json<'_>> for StatsInfo {
	fn from(value: Json<'_>) -> Self {
		let stats_info = value.as_inner();
		Self {
			total: stats_info.get("total").and_then(|v| v.as_u64()).unwrap_or(0),
			additions: stats_info.get("additions").and_then(|v| v.as_u64()).unwrap_or(0),
			deletions: stats_info.get("deletions").and_then(|v| v.as_u64()).unwrap_or(0),
		}
	}
}

impl From<Json<'_>> for FileInfo {
	fn from(value: Json<'_>) -> Self {
		let file_info = value.as_inner();
		let status_value = file_info.get("status").unwrap();
		let additions = file_info.get("additions").and_then(|v| v.as_u64()).unwrap_or(0);
		let deletions = file_info.get("deletions").and_then(|v| v.as_u64()).unwrap_or(0);
		Self {
			file_name: file_info
				.get("filename")
				.or_else(|| file_info.get("file_name"))
				.and_then(|v| v.as_str())
				.unwrap_or_default()
				.to_string(),
			status: Json::new(status_value).into(),
			additions,
			deletions,
			changes: file_info
				.get("changes")
				.and_then(|v| v.as_u64())
				.unwrap_or(additions + deletions),
		}
	}
}

impl From<Json<'_>> for FileStatus {
	fn from(value: Json<'_>) -> Self {
		match value.as_inner().as_str().map(|s| s.to_lowercase()).as_deref() {
			Some("added") => Self::Added,
			Some("modified") => Self::Modified,
			Some("removed") => Self::Deleted,
			Some("renamed") => Self::Renamed,
			Some("copied") => Self::Copied,
			Some("changed") => Self::Changed,
			_ => Self::UnChanged,
		}
	}
}
