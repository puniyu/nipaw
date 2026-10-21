use crate::{CnbClientInner, Error, Json, Result};
use async_trait::async_trait;
use nipaw_core::{
	Release,
	release::{AssetsInfo, AuthorInfo, ReleaseInfo, ReleaseUpdateOption},
	repo::RepoPath,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CnbRelease(pub(crate) Arc<CnbClientInner>);

impl CnbRelease {
	pub(crate) async fn get_release_id(&self, repo_path: &RepoPath, tag_name: &str) -> Result<u64> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = format!(
			"{}/{}/{}/-/releases/tags/{}",
			api_url, repo_path.owner, repo_path.repo, tag_name
		);
		let client = self.0.http();
		let request = client.get(url).bearer_auth(token.as_ref().unwrap());
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
impl Release for CnbRelease {
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
		let client = self.0.http();
		let url = format!("{}/{}/{}/-/releases", api_url, repo_path.owner, repo_path.repo);
		let request = client.post(url).bearer_auth(token.as_ref().unwrap());
		let json_body = serde_json::json!({
			"tag_name": tag_name,
			"name": name.unwrap_or(tag_name),
			"body": body.unwrap_or(tag_name),
			"target_commitish": target_commitish.unwrap_or("HEAD")
		});

		let res = request.json(&json_body).send().await?.json::<Json>().await?;
		Ok(res.into())
	}

	async fn info(&self, repo_path: RepoPath, tag_name: Option<&str>) -> Result<ReleaseInfo> {
		let config = self.0.config();
		let (token, api_url) = (&config.token, &config.api_url);
		if token.is_none() {
			return Err(Error::TokenEmpty);
		}
		let url = if let Some(tag_name) = tag_name {
			format!(
				"{}/{}/{}/-/releases/tags/{}",
				api_url, repo_path.owner, repo_path.repo, tag_name
			)
		} else {
			format!("{}/{}/{}/-/releases/latest", api_url, repo_path.owner, repo_path.repo)
		};
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
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
		let url = format!("{}/{}/{}/-/releases", api_url, repo_path.owner, repo_path.repo);
		let client = self.0.http();
		let mut request = client.get(url);
		if let Some(token) = token {
			request = request.bearer_auth(token);
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
		let url =
			format!("{}/{}/{}/-/releases/{}", api_url, repo_path.owner, repo_path.repo, release_id);
		let client = self.0.http();
		let request = client.patch(url).bearer_auth(token.as_ref().unwrap());
		let json_body = serde_json::json!({
			"name": option.name.as_deref().unwrap_or(tag_name),
			"body": option.body.as_deref().unwrap_or(tag_name)
		});
		let res = request.json(&json_body).send().await?.json::<Json>().await?;
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
				.get("tag_commitish")
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
				.get("username")
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
		let base_url = "https://cnb.cool";
		let path =
			asset_info.get("path").and_then(|name| name.as_str().map(|s| s.to_string())).unwrap();
		Self {
			name: asset_info
				.get("name")
				.and_then(|name| name.as_str().map(|s| s.to_string()))
				.unwrap(),
			url: format!("{}/{}", base_url, path),
		}
	}
}
