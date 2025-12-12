use crate::common::JsonValue;
use crate::{CnbClientInner, get_repo_default_branch};
use async_trait::async_trait;
use nipaw_core::types::repo::{CollaboratorPermission, CollaboratorResult, RepoInfo};
use nipaw_core::{Error, Repo, Result};
use reqwest::header;
use serde_json::Value;
use std::sync::Arc;

pub struct CnbRepo(pub(crate) Arc<CnbClientInner>);

#[async_trait]
impl Repo for CnbRepo {
	async fn info(&self, repo_path: (&str, &str)) -> Result<RepoInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		let url = format!("{}/repos/{}/{}", api_url, repo_path.0, repo_path.1);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let resp = request.send().await?;
		let mut repo_info: JsonValue = resp.json().await?;
		let default_branch =
			get_repo_default_branch(client.clone(), &self.0.config, &repo_info, token.clone())
				.await?;
		repo_info
			.0
			.as_object_mut()
			.unwrap()
			.insert("default_branch".to_string(), Value::String(default_branch));
		Ok(repo_info.into())
	}

	async fn add_repo_collaborator(
		&self,
		repo_path: (&str, &str),
		user_name: &str,
		permission: Option<CollaboratorPermission>,
	) -> Result<CollaboratorResult> {
		let (token, api_url, base_url) =
			(&self.0.config.token, &self.0.config.api_url, &self.0.config.base_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/{}/{}/-/members/{}", api_url, repo_path.0, repo_path.1, user_name);
		let client = self.0.client.read().await;
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
		let resp = request
			.header(header::CONTENT_TYPE, "application/json")
			.body(body.to_string())
			.send()
			.await?;
		let status_code = resp.status().as_u16();
		if status_code == 200 {
			let collaborator = CollaboratorResult {
				login: user_name.to_string(),
				avatar_url: format!("{}/users/{}/avatar/l", base_url, user_name),
			};
			Ok(collaborator)
		} else {
			Err(Error::NotFound)
		}
	}
}
