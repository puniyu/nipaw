use crate::{CnbClientInner, Error, Json, Result, get_repo_default_branch};
use async_trait::async_trait;
use nipaw_core::{
	Repo,
	repo::{
		RepoInfo, RepoPath, Visibility,
		collaborator::{CollaboratorInfo, Permission as CollaboratorPermission},
	},
};
use reqwest::header;
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CnbRepo(pub(crate) Arc<CnbClientInner>);

#[async_trait]
impl Repo for CnbRepo {
	type Err = Error;

	async fn info(&self, repo_path: RepoPath) -> Result<RepoInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/{}", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut repo_info: Json = resp.json().await?;
		let default_branch =
			get_repo_default_branch(client.clone(), &config, &repo_info, token.clone()).await?;
		repo_info
			.as_inner_mut()
			.as_object_mut()
			.unwrap()
			.insert("default_branch".to_string(), Value::String(default_branch));
		Ok(repo_info.into())
	}

	async fn add_repo_collaborator(
		&self,
		repo_path: RepoPath,
		user_name: &str,
		permission: Option<CollaboratorPermission>,
	) -> Result<CollaboratorInfo> {
		let config = self.0.config();
		let (token, api_url, base_url) = (&config.token, &config.api_url, &config.base_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url =
			format!("{}/{}/{}/-/members/{}", api_url, repo_path.owner, repo_path.repo, user_name);
		let client = self.0.http();
		let request = client.post(url).bearer_auth(token.as_ref().unwrap());
		let permission = match permission {
			Some(permission) => match permission {
				CollaboratorPermission::Admin => "Master",
				CollaboratorPermission::Push => "Developer",
				CollaboratorPermission::Pull => "Reporter",
			},
			None => "Guest",
		};

		let body = serde_json::json!({
			"access_level": permission.to_string(),
			"is_outside_collaborator": true,
		});
		request
			.header(header::CONTENT_TYPE, "application/json")
			.body(body.to_string())
			.send()
			.await?;
		let collaborator = CollaboratorInfo {
			login: user_name.to_string(),
			avatar_url: format!("{}/users/{}/avatar/l", base_url, user_name),
		};
		Ok(collaborator)
	}
}

impl From<Json<'_>> for RepoInfo {
	fn from(json_value: Json<'_>) -> Self {
		let repo_info = json_value.as_inner();
		let is_public = repo_info
			.get("visibility_level")
			.and_then(|v| v.as_str())
			.map(|s| s.to_lowercase() == "public")
			.unwrap_or(false);

		Self {
			owner: repo_info
				.get("owner")
				.and_then(|v| v.get("login"))
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string(),
			name: repo_info.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
			full_name: repo_info.get("full_name").and_then(|v| v.as_str()).unwrap().to_string(),
			description: repo_info
				.get("description")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			visibility: if is_public { Visibility::Public } else { Visibility::Private },
			fork: repo_info
				.get("forked_from_repo")
				.and_then(|v| v.get("path"))
				.and_then(|v| v.as_str())
				.is_some(),
			fork_count: repo_info.get("fork_count").and_then(|v| v.as_u64()).unwrap_or(0),
			language: repo_info
				.get("language")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			star_count: repo_info.get("star_count").and_then(|v| v.as_u64()).unwrap_or(0),
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
				.get("updated_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
		}
	}
}
