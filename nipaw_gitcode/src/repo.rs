use crate::GitCodeClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::types::repo::{CollaboratorPermission, CollaboratorResult, RepoInfo, RepoPath};
use nipaw_core::{Error, Repo, Result};
use serde_json::Value;
use std::sync::Arc;

pub struct GitCodeRepo(pub(crate) Arc<GitCodeClientInner>);

#[async_trait]
impl Repo for GitCodeRepo {
	async fn info(&self, repo_path: RepoPath<'_>) -> Result<RepoInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/{}", api_url, repo_path.0, repo_path.1);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn add_repo_collaborator(
		&self,
		repo_path: RepoPath<'_>,
		user_name: &str,
		permission: Option<CollaboratorPermission>,
	) -> Result<CollaboratorResult> {
		let (token, api_url, web_api_url, base_url) = (
			&self.0.config.token,
			&self.0.config.api_url,
			&self.0.config.web_api_url,
			&self.0.config.base_url,
		);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!(
			"{}/repos/{}/{}/collaborators/{}",
			api_url, repo_path.0, repo_path.1, user_name
		);
		let client = self.0.client.read().await;
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
		let mut collaborator = resp.json::<JsonValue>().await?;
		if let Some(obj) = collaborator.0.as_object_mut() {
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
