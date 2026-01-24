use crate::GitHubClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::option::release::UpdateOption;
use nipaw_core::types::release::ReleaseInfo;
use nipaw_core::types::repo::RepoPath;
use nipaw_core::{Error, Release, Result};
use std::sync::Arc;

pub struct GitHubRelease(pub(crate) Arc<GitHubClientInner>);

impl GitHubRelease {
	pub(crate) async fn get_release_id(
		&self,
		repo_path: &RepoPath,
		tag_name: &str,
	) -> Result<u64> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url =
			format!("{}/repos/{}/{}/releases/tags/{}", api_url, repo_path.owner, repo_path.repo, tag_name);
		let client = self.0.client.read().await;
		let request = client.get(url).bearer_auth(token.as_ref().unwrap());
		let res = request.send().await?.json::<JsonValue>().await?;
		let id = res.0.get("id").and_then(|x| x.as_u64()).ok_or(Error::NotFound{ resource: "id".to_string()})?;
		Ok(id)
	}
}

#[async_trait]
impl Release for GitHubRelease {
	async fn create(
		&self,
		repo_path: RepoPath,
		tag_name: &str,
		name: Option<&str>,
		body: Option<&str>,
		target_commitish: Option<&str>,
	) -> Result<ReleaseInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/{}/releases", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.client.read().await;
		let json_body = serde_json::json!({
			"tag_name": tag_name,
			"name": name.unwrap_or(tag_name),
			"body": body.unwrap_or(tag_name),
			"target_commitish":  target_commitish.unwrap_or("HEAD")
		});

		let request = client.post(url).bearer_auth(token.as_ref().unwrap()).json(&json_body);
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn info(&self, repo_path: RepoPath, tag_name: Option<&str>) -> Result<ReleaseInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = if let Some(tag_name) = tag_name {
			format!(
				"{}/repos/{}/{}/releases/tags/{}",
				api_url,
				repo_path.owner,
				repo_path.repo,
				tag_name
			)
		} else {
			format!("{}/repos/{}/{}/releases/latest", api_url, repo_path.owner, repo_path.repo)
		};
		let client = self.0.client.read().await;
		let request = client.get(url).bearer_auth(token.as_ref().unwrap());
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn list(&self, repo_path: RepoPath) -> Result<Vec<ReleaseInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/{}/releases", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.client.read().await;
		let request = client.get(url).bearer_auth(token.as_ref().unwrap());
		let res = request.send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|x| x.into()).collect())
	}

	async fn update(
		&self,
		repo_path: RepoPath,
		tag_name: &str,
		option: UpdateOption,
	) -> Result<ReleaseInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let release_id = self.get_release_id(&repo_path, tag_name).await?;
		let url =
			format!("{}/repos/{}/{}/releases/{}", api_url, repo_path.owner, repo_path.repo, release_id);
		let client = self.0.client.read().await;
		let request = client.patch(url).bearer_auth(token.as_ref().unwrap());
		let json_body = serde_json::json!({
			"name": option.name.unwrap_or(tag_name.to_string()),
			"body": option.body.unwrap_or(tag_name.to_string())
		});
		let res = request.json(&json_body).send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}
}
