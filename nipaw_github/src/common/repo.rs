use crate::common::JsonValue;
use nipaw_core::types::repo::{CollaboratorResult, RepoInfo, Visibility};

impl From<JsonValue> for RepoInfo {
	fn from(json_value: JsonValue) -> Self {
		let repo_info = json_value.0;
		let is_public = repo_info
			.get("visibility")
			.and_then(|v| v.as_str())
			.map(|s| s.to_lowercase() == "public")
			.unwrap_or(false);
		let owner = repo_info
			.get("owner")
			.and_then(|v| v.get("login"))
			.and_then(|v| v.as_str())
			.unwrap()
			.to_string();
		let name = repo_info.get("name").and_then(|v| v.as_str()).unwrap().to_string();
		Self {
			owner: owner.clone(),
			name: name.clone(),
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

impl From<JsonValue> for CollaboratorResult {
	fn from(json_value: JsonValue) -> Self {
		let collaborator = json_value.0;
		Self {
			login: collaborator
				.get("inviter")
				.unwrap()
				.get("login")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string(),
			avatar_url: collaborator
				.get("inviter")
				.unwrap()
				.get("avatar_url")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string(),
		}
	}
}
