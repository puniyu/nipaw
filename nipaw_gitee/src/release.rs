use crate::{Error, GiteeClientInner, Json, Result};
use async_trait::async_trait;
use nipaw_core::{
	Release,
	release::{AssetsInfo, AuthorInfo, ReleaseInfo, ReleaseUpdateOption},
	repo::RepoPath,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct GiteeRelease(pub(crate) Arc<GiteeClientInner>);

impl GiteeRelease {
	pub(crate) async fn get_release_id(&self, repo_path: &RepoPath, tag_name: &str) -> Result<u64> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!(
			"{}/repos/{}/{}/releases/tags/{}",
			api_url, repo_path.owner, repo_path.repo, tag_name
		);
		let client = self.0.http();
		let request = client.get(url).query(&[("token", token.as_ref().unwrap())]);
		let res = request.send().await?.json::<Json>().await?;
		let id = res
			.as_inner()
			.get("id")
			.and_then(|x| x.as_u64())
			.ok_or(Error::NotFound { resource: "id".to_string() })?;
		Ok(id)
	}
}

#[async_trait]
impl Release for GiteeRelease {
	type Err = Error;

	async fn create(
		&self,
		repo_path: RepoPath,
		tag_name: &str,
		name: Option<&str>,
		body: Option<&str>,
		target_commitish: Option<&str>,
	) -> Result<ReleaseInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/{}/releases", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.http();
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
		let res = request.send().await?.json::<Json>().await?;
		Ok(res.into())
	}

	async fn info(&self, repo_path: RepoPath, tag_name: Option<&str>) -> Result<ReleaseInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
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
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.query(&[("token", token)])
		}
		let res = request.send().await?.json::<Json>().await?;
		Ok(res.into())
	}

	async fn list(&self, repo_path: RepoPath) -> Result<Vec<ReleaseInfo>> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!("{}/repos/{}/{}/releases", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.query(&[("token", token)])
		}
		let res = request.send().await?.json::<Vec<Json>>().await?;
		Ok(res.into_iter().map(Into::into).collect())
	}

	async fn update(
		&self,
		repo_path: RepoPath,
		tag_name: &str,
		option: ReleaseUpdateOption,
	) -> Result<ReleaseInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let release_id = self.get_release_id(&repo_path, tag_name).await?;
		let url = format!(
			"{}/repos/{}/{}/releases/{}",
			api_url, repo_path.owner, repo_path.repo, release_id
		);
		let client = self.0.http();
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
		let res = request.form(&form).send().await?.json::<Json>().await?;
		Ok(res.into())
	}
}

impl From<Json<'_>> for ReleaseInfo {
	fn from(release: Json<'_>) -> Self {
		let release_info = release.as_inner();
		let author_info = release_info.get("author").unwrap();
		let assets_info = release_info.get("assets").unwrap();
		Self {
			tag_name: release_info
				.get("tag_name")
				.and_then(|tag_name| tag_name.as_str().map(|s| s.to_string()))
				.unwrap(),
			target_commitish: release_info
				.get("target_commitish")
				.and_then(|target_commitish| target_commitish.as_str().map(|s| s.to_string()))
				.unwrap(),
			prerelease: release_info
				.get("prerelease")
				.and_then(|prerelease| prerelease.as_bool())
				.unwrap(),
			name: release_info
				.get("name")
				.and_then(|name| name.as_str().map(|s| s.to_string()))
				.unwrap(),
			body: release_info
				.get("body")
				.and_then(|body| body.as_str().map(|s| s.to_string()))
				.filter(|s| !s.is_empty()),
			author: Json::new(author_info).into(),
			created_at: release_info
				.get("created_at")
				.and_then(|created_at| created_at.as_str().map(|s| s.to_string()))
				.unwrap()
				.parse()
				.unwrap(),
			assets: Json::new(assets_info).into(),
		}
	}
}

impl From<Json<'_>> for AuthorInfo {
	fn from(author: Json<'_>) -> Self {
		let author_info = author.as_inner();
		Self {
			login: author_info
				.get("login")
				.and_then(|name| name.as_str().map(|s| s.to_string()))
				.unwrap(),
			avatar_url: author_info
				.get("avatar_url")
				.and_then(|name| name.as_str().map(|s| s.to_string()))
				.unwrap(),
		}
	}
}

impl From<Json<'_>> for Vec<AssetsInfo> {
	fn from(assets: Json<'_>) -> Self {
		assets
			.as_inner()
			.as_array()
			.map(|arr| arr.iter().map(|asset| Json::new(asset).into()).collect())
			.unwrap_or_default()
	}
}

impl From<Json<'_>> for AssetsInfo {
	fn from(asset: Json<'_>) -> Self {
		let asset_info = asset.as_inner();
		Self {
			name: asset_info
				.get("name")
				.and_then(|name| name.as_str().map(|s| s.to_string()))
				.unwrap(),
			url: asset_info
				.get("browser_download_url")
				.and_then(|name| name.as_str().map(|s| s.to_string()))
				.unwrap(),
		}
	}
}
