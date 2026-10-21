use crate::{GitHubState, Json, Result};
use async_trait::async_trait;
use http::header;
use nipaw_core::Repo;
use nipaw_core::repo::collaborator::{CollaboratorInfo, Permission as CollaboratorPermission};
use nipaw_core::repo::{RepoInfo, RepoPath, Visibility};
use std::sync::Arc;

type State = Arc<GitHubState>;

#[derive(Debug, Clone)]
pub struct GitHubRepo(State);

impl GitHubRepo {
	pub(crate) fn new(state: impl Into<State>) -> Self {
		Self(state.into())
	}
}

#[async_trait]
impl Repo for GitHubRepo {
	type Err = crate::Error;

	async fn info(&self, repo_path: RepoPath) -> Result<RepoInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		let url = format!("{}/repos/{}/{}", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let repo_info: Json = resp.json().await?;
		Ok(repo_info.into())
	}

	async fn add_repo_collaborator(
		&self,
		repo_path: RepoPath,
		user_name: &str,
		permission: Option<CollaboratorPermission>,
	) -> Result<CollaboratorInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		let url = format!(
			"{}/repos/{}/{}/collaborators/{}",
			api_url, repo_path.owner, repo_path.repo, user_name
		);
		let client = self.0.http();
		let mut request = client.put(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let permission = match permission {
			Some(permission) => match permission {
				CollaboratorPermission::Admin => "admin".to_string(),
				CollaboratorPermission::Push => "push".to_string(),
				CollaboratorPermission::Pull => "pull".to_string(),
			},
			None => "pull".to_string(),
		};

		let body = serde_json::json!({
			"permission": permission,
		});
		let resp = request
			.header(header::CONTENT_TYPE, "application/json")
			.body(body.to_string())
			.send()
			.await?;
		let collaborator_result: Json = resp.json().await?;
		Ok(collaborator_result.into())
	}
}

impl From<Json<'_>> for RepoInfo {
	fn from(json_value: Json<'_>) -> Self {
		let repo_info = json_value.as_inner();
		let is_public = repo_info
			.get("visibility")
			.and_then(|v| v.as_str())
			.map(|s| s.to_lowercase() == "public")
			.unwrap_or(false);
		let owner =
			repo_info.get("owner").and_then(|v| v.get("login")).and_then(|v| v.as_str()).unwrap();
		let name = repo_info.get("name").and_then(|v| v.as_str()).unwrap();
		Self {
			owner: owner.to_string(),
			name: name.to_string(),
			full_name: format!("{}/{}", owner, name),
			description: repo_info
				.get("description")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			visibility: if is_public { Visibility::Public } else { Visibility::Private },
			fork: repo_info.get("fork").and_then(|v| v.as_bool()).unwrap_or(false),
			fork_count: repo_info.get("forks_count").and_then(|v| v.as_u64()).unwrap_or(0),
			language: repo_info
				.get("language")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			star_count: repo_info.get("stargazers_count").and_then(|v| v.as_u64()).unwrap_or(0),
			default_branch: repo_info
				.get("default_branch")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string(),
			created_at: super::parse_datetime(
				repo_info.get("created_at").and_then(|v| v.as_str()).unwrap(),
			),
			updated_at: super::parse_datetime(
				repo_info.get("updated_at").and_then(|v| v.as_str()).unwrap(),
			),
			pushed_at: super::parse_datetime(
				repo_info.get("pushed_at").and_then(|v| v.as_str()).unwrap(),
			),
		}
	}
}

impl From<Json<'_>> for CollaboratorInfo {
	fn from(json_value: Json<'_>) -> Self {
		let collaborator = json_value.as_inner();
		Self {
			login: collaborator
				.get("inviter")
				.unwrap()
				.get("login")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string(),
			avatar_url: collaborator
				.get("inviter")
				.unwrap()
				.get("avatar_url")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string(),
		}
	}
}
