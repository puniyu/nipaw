use crate::GitCodeClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::option::release::UpdateOption;
use nipaw_core::types::release::ReleaseInfo;
use nipaw_core::types::repo::RepoPath;
use nipaw_core::{Error, Release, Result};
use std::sync::Arc;

pub struct GitCodeRelease(pub(crate) Arc<GitCodeClientInner>);

#[async_trait]
impl Release for GitCodeRelease {
	async fn create(
		&self,
		repo_path: RepoPath<'_>,
		tag_name: &str,
		name: Option<&str>,
		body: Option<&str>,
		target_commitish: Option<&str>,
	) -> Result<ReleaseInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let client = self.0.client.read().await;
		let url = format!("{}/repos/{}/{}/releases", api_url, repo_path.0, repo_path.1);
		let request = client.post(url).bearer_auth(token.as_ref().unwrap());
		let json_body = serde_json::json!({
			"tag_name": tag_name,
			"name": if name.is_none() { tag_name } else { name.unwrap() },
			"body": if body.is_none() { tag_name } else { body.unwrap() },
			"target_commitish": if target_commitish.is_none() { "HEAD" } else { target_commitish.unwrap() }
		});

		let res = request.json(&json_body).send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn info(&self, repo_path: RepoPath<'_>, tag_name: Option<&str>) -> Result<ReleaseInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = if tag_name.is_none() {
			format!("{}/repos/{}/{}/releases/latest", api_url, repo_path.0, repo_path.1)
		} else {
			format!(
				"{}/repos/{}/{}/releases/tags/{}",
				api_url,
				repo_path.0,
				repo_path.1,
				tag_name.unwrap()
			)
		};
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn list(&self, repo_path: RepoPath<'_>) -> Result<Vec<ReleaseInfo>> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/{}/releases", api_url, repo_path.0, repo_path.1);
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}
		let res = request.send().await?.json::<Vec<JsonValue>>().await?;
		Ok(res.into_iter().map(|x| x.into()).collect())
	}

	async fn update(
		&self,
		repo_path: RepoPath<'_>,
		tag_name: &str,
		option: UpdateOption,
	) -> Result<ReleaseInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url =
			format!("{}/repos/{}/{}/releases/{}", api_url, repo_path.0, repo_path.1, tag_name);
		let client = self.0.client.read().await;
		let mut request = client.patch(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
		}

		let json_body = serde_json::json!({
			"name": if option.name.is_none() { tag_name } else { option.name.as_ref().unwrap() },
			"body": if option.body.is_none() { tag_name } else { option.body.as_ref().unwrap() }
		});
		let res = request.json(&json_body).send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}
}
