use crate::{Error, GiteeClientInner, Json, Result, format_datetime};
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
pub struct GiteeCommit(pub(crate) Arc<GiteeClientInner>);

#[async_trait]
impl Commit for GiteeCommit {
	type Err = Error;

	async fn info(&self, repo_path: RepoPath, sha: Option<&str>) -> Result<CommitInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
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
			request = request.query(&[("access_token", token.as_str())]);
		}
		let resp = request.send().await?;
		let mut commit_info: Json = resp.json().await?;
		let author_avatar_url = commit_info
			.as_inner()
			.get("author")
			.and_then(|v| v.get("avatar_url"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string();
		let committer_avatar_url = commit_info
			.as_inner()
			.get("committer")
			.and_then(|v| v.get("avatar_url"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string();
		if let Some(author_obj) = commit_info
			.as_inner_mut()
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("author"))
			.and_then(|author| author.as_object_mut())
		{
			author_obj.insert("avatar_url".to_string(), Value::String(author_avatar_url));
		}

		if let Some(committer_obj) = commit_info
			.as_inner_mut()
			.get_mut("commit")
			.and_then(|commit| commit.as_object_mut())
			.and_then(|commit_obj| commit_obj.get_mut("committer"))
			.and_then(|committer| committer.as_object_mut())
		{
			committer_obj.insert("avatar_url".to_string(), Value::String(committer_avatar_url));
		}
		Ok(commit_info.into())
	}

	async fn list(
		&self,
		repo_path: RepoPath,
		option: Option<CommitListOptions>,
	) -> Result<Vec<CommitListInfo>> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		let url = format!("{}/repos/{}/{}/commits", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.http();
		let request = client.get(url);
		let mut params: HashMap<&str, String> = HashMap::new();
		if let Some(token) = token {
			params.insert("access_token", token.to_owned());
		}

		if let Some(option) = option {
			let per_page = option.per_page.unwrap_or(30).max(100);
			params.insert("per_page", per_page.to_string());
			let page = option.page.unwrap_or(1);
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
		let resp = request.query(&params).send().await?;
		let commit_infos: Vec<Json> = resp.json().await?;
		Ok(commit_infos.into_iter().map(Into::into).collect())
	}
}

impl From<Json<'_>> for CommitInfo {
	fn from(commit: Json<'_>) -> Self {
		let commit_info = commit.as_inner();
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
	fn from(commit: Json<'_>) -> Self {
		let commit_info = commit.as_inner();
		let commit_value = commit_info.get("commit").unwrap();
		Self {
			sha: commit_info.get("sha").and_then(|v| v.as_str()).unwrap().to_string(),
			commit: Json::new(commit_value).into(),
		}
	}
}

impl From<Json<'_>> for CommitData {
	fn from(commit: Json<'_>) -> Self {
		let commit_data = commit.as_inner();
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
	fn from(user: Json<'_>) -> Self {
		let user_info = user.as_inner();
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
	fn from(stats: Json<'_>) -> Self {
		let stats_info = stats.as_inner();
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
		Self {
			file_name: file_info
				.get("filename")
				.and_then(|v| v.as_str())
				.unwrap_or_default()
				.to_string(),
			status: Json::new(status_value).into(),
			additions: file_info.get("additions").and_then(|v| v.as_u64()).unwrap_or(0),
			deletions: file_info.get("deletions").and_then(|v| v.as_u64()).unwrap_or(0),
			changes: file_info.get("changes").and_then(|v| v.as_u64()).unwrap_or(0),
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
