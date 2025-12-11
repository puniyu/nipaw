use crate::common::JsonValue;
use crate::GitHubClientInner;
use async_trait::async_trait;
use http::header;
use nipaw_core::types::collaborator::{CollaboratorPermission, CollaboratorResult};
use nipaw_core::types::repo::RepoInfo;
use nipaw_core::{Repo, Result};
use std::sync::Arc;

pub struct GitHubRepo(pub(crate) Arc<GitHubClientInner>);

#[async_trait]
impl Repo for GitHubRepo {
	async fn info(&self, repo_path: (&str, &str)) -> Result<RepoInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!("{}/repos/{}/{}", api_url, repo_path.0, repo_path.1);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let repo_info: JsonValue = resp.json().await?;
		Ok(repo_info.into())
	}

	async fn add_repo_collaborator(
		&self,
		repo_path: (&str, &str),
		user_name: &str,
		permission: Option<CollaboratorPermission>,
	) -> Result<CollaboratorResult> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!(
			"{}/repos/{}/{}/collaborators/{}",
			api_url, repo_path.0, repo_path.1, user_name
		);
		let client = self.0.client.read().await;
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
		let collaborator_result: JsonValue = resp.json().await?;
		Ok(collaborator_result.into())
	}
}
