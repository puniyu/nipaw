use crate::{Error, GitCodeClientInner, Json, Result};
use async_trait::async_trait;
use nipaw_core::{
	Repo,
	repo::{
		RepoInfo, RepoPath, Visibility,
		collaborator::{CollaboratorInfo, Permission as CollaboratorPermission},
	},
};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GitCodeRepo(pub(crate) Arc<GitCodeClientInner>);

#[async_trait]
impl Repo for GitCodeRepo {
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
		let res = request.send().await?.json::<Json>().await?;
		Ok(res.into())
	}

	async fn add_repo_collaborator(
		&self,
		repo_path: RepoPath,
		user_name: &str,
		permission: Option<CollaboratorPermission>,
	) -> Result<CollaboratorInfo> {
		let config = self.0.config();
		let (token, api_url, web_api_url, base_url) =
			(&config.token, &config.api_url, &config.web_api_url, &config.base_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!(
			"{}/repos/{}/{}/collaborators/{}",
			api_url, repo_path.owner, repo_path.repo, user_name
		);
		let client = self.0.http();
		let request = client.put(url).bearer_auth(token.as_ref().unwrap());
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

		let resp = request.body(body.to_string()).send().await?;
		let mut collaborator = resp.json::<Json>().await?;
		if let Some(obj) = collaborator.as_inner_mut().as_object_mut() {
			let avatar_url =
				get_user_avatar_url(client.clone(), web_api_url, base_url, user_name).await?;
			obj.insert("avatar_url".to_string(), Value::String(avatar_url));
		}
		Ok(collaborator.into())
	}
}

async fn get_user_avatar_url(
	client: Arc<reqwest_middleware::ClientWithMiddleware>,
	web_api_url: &str,
	base_url: &str,
	user_name: &str,
) -> Result<String> {
	let url = format!("{}/uc/api/v1/user/setting/profile?username={}", web_api_url, user_name);
	let res = client.get(url).header("Referer", base_url).send().await?.json::<Value>().await?;
	let avatar_url = res.get("avatar").and_then(|v| v.as_str()).unwrap().to_string();
	Ok(avatar_url)
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
