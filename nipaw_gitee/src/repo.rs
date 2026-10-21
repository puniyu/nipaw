use crate::{Error, GiteeClientInner, Json, Result};
use async_trait::async_trait;
use nipaw_core::{
	Repo,
	repo::{
		RepoInfo, RepoPath, Visibility,
		collaborator::{CollaboratorInfo, Permission as CollaboratorPermission},
	},
};
use reqwest::header;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GiteeRepo(pub(crate) Arc<GiteeClientInner>);

#[async_trait]
impl Repo for GiteeRepo {
	type Err = Error;

	async fn info(&self, repo_path: RepoPath) -> Result<RepoInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		let url = format!("{}/repos/{}/{}", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.query(&[("access_token", token.as_str())]);
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
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!(
			"{}/repos/{}/{}/collaborators/{}",
			api_url, repo_path.owner, repo_path.repo, user_name
		);
		let client = self.0.http();
		let request = client.put(url);

		let permission = match permission {
			Some(permission) => match permission {
				CollaboratorPermission::Admin => "admin".to_string(),
				CollaboratorPermission::Push => "push".to_string(),
				CollaboratorPermission::Pull => "pull".to_string(),
			},
			None => "pull".to_string(),
		};

		let body = serde_json::json!({
			"access_token": token.as_ref().unwrap(),
			"permission": permission,
		});

		let res = request
			.header(header::CONTENT_TYPE, "application/json")
			.body(body.to_string())
			.send()
			.await?
			.json::<Json>()
			.await?;
		Ok(res.into())
	}
}

impl From<Json<'_>> for RepoInfo {
	fn from(json_value: Json<'_>) -> Self {
		let repo_info = json_value.as_inner();
		let is_public = repo_info.get("public").and_then(|v| v.as_bool()).unwrap_or(false);
		let owner =
			repo_info.get("owner").and_then(|v| v.get("login")).and_then(|v| v.as_str()).unwrap();
		let name = repo_info.get("path").and_then(|v| v.as_str()).unwrap();
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
			created_at: repo_info
				.get("created_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
			updated_at: repo_info
				.get("updated_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
			pushed_at: repo_info
				.get("pushed_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
		}
	}
}

impl From<Json<'_>> for CollaboratorInfo {
	fn from(json_value: Json<'_>) -> Self {
		let collaborator = json_value.as_inner();
		Self {
			login: collaborator.get("login").and_then(|v| v.as_str()).unwrap().to_string(),
			avatar_url: collaborator
				.get("avatar_url")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string(),
		}
	}
}
