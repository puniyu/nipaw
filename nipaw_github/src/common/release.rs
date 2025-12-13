use crate::common::JsonValue;
use nipaw_core::types::release::{AssetsInfo, AuthorInfo, ReleaseInfo};
impl From<JsonValue> for ReleaseInfo {
	fn from(release: JsonValue) -> Self {
		let release_info = release.0;
		let author_info = release_info.get("author").unwrap().clone();
		let assets_info = release_info.get("assets").unwrap().clone();
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
			author: JsonValue(author_info).into(),
			created_at: release_info
				.get("created_at")
				.and_then(|created_at| created_at.as_str().map(|s| s.to_string()))
				.unwrap()
				.parse()
				.unwrap(),
			assets: JsonValue(assets_info).into(),
		}
	}
}

impl From<JsonValue> for AuthorInfo {
	fn from(author: JsonValue) -> Self {
		let author_info = author.0;
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

impl From<JsonValue> for Vec<AssetsInfo> {
	fn from(assets: JsonValue) -> Self {
		assets.into_iter().map(|asset| asset.into()).collect()
	}
}

impl From<JsonValue> for AssetsInfo {
	fn from(asset: JsonValue) -> Self {
		let asset_info = asset.0;
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
