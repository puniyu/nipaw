use crate::GiteeClientInner;
use crate::common::JsonValue;
use async_trait::async_trait;
use nipaw_core::option::release::UpdateOption;
use nipaw_core::types::release::ReleaseInfo;
use nipaw_core::types::repo::RepoPath;
use nipaw_core::{Error, Release, Result};
use std::sync::Arc;

pub struct GiteeRelease(pub(crate) Arc<GiteeClientInner>);

impl GiteeRelease {
	pub(crate) async fn get_release_id(&self, repo_path: &RepoPath, tag_name: &str) -> Result<u64> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!(
			"{}/repos/{}/{}/releases/tags/{}",
			api_url, repo_path.owner, repo_path.repo, tag_name
		);
		let client = self.0.client.read().await;
		let request = client.get(url).query(&[("token", token.as_ref().unwrap())]);
		let res = request.send().await?.json::<JsonValue>().await?;
		let id = res
			.0
			.get("id")
			.and_then(|x| x.as_u64())
			.ok_or(Error::NotFound { resource: "id".to_string() })?;
		Ok(id)
	}
}

#[async_trait]
impl Release for GiteeRelease {
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
		let mut form = serde_json::Map::new();
		form.insert("tag_name".to_string(), serde_json::Value::String(tag_name.to_string()));
		form.insert(
			"name".to_string(),
			serde_json::Value::String(
				name.map(|s| s.to_string()).unwrap_or_else(|| tag_name.to_string()),
			),
		);
		form.insert(
			"body".to_string(),
			serde_json::Value::String(
				body.map(|s| s.to_string()).unwrap_or_else(|| tag_name.to_string()),
			),
		);
		if let Some(commitish) = target_commitish {
			form.insert(
				"target_commitish".to_string(),
				serde_json::Value::String(commitish.to_string()),
			);
		}
		let request = client.post(url).query(&[("token", token.as_ref().unwrap())]).form(&form);
		let res = request.send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}

	async fn info(&self, repo_path: RepoPath, tag_name: Option<&str>) -> Result<ReleaseInfo> {
		let (token, api_url) = (&self.0.config.token, &self.0.config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = if let Some(tag) = tag_name {
			format!(
				"{}/repos/{}/{}/releases/tags/{}",
				api_url, repo_path.owner, repo_path.repo, tag
			)
		} else {
			format!("{}/repos/{}/{}/releases/latest", api_url, repo_path.owner, repo_path.repo)
		};
		let client = self.0.client.read().await;
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.query(&[("token", token)])
		}
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
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.query(&[("token", token)])
		}
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
		let url = format!(
			"{}/repos/{}/{}/releases/{}",
			api_url, repo_path.owner, repo_path.repo, release_id
		);
		let client = self.0.client.read().await;
		let request = client.patch(url).query(&[("token", token.as_ref().unwrap())]);
		let mut form = serde_json::Map::new();
		form.insert(
			"name".to_string(),
			serde_json::Value::String(
				option.name.as_ref().map(|s| s.to_string()).unwrap_or_else(|| tag_name.to_string()),
			),
		);
		form.insert(
			"body".to_string(),
			serde_json::Value::String(
				option.body.as_ref().map(|s| s.to_string()).unwrap_or_else(|| tag_name.to_string()),
			),
		);
		let res = request.form(&form).send().await?.json::<JsonValue>().await?;
		Ok(res.into())
	}
}
