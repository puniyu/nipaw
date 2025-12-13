use crate::common::JsonValue;
use nipaw_core::types::commit::{
	CommitData, CommitInfo, FileInfo, FileStatus, StatsInfo, UserInfo,
};

impl From<JsonValue> for CommitInfo {
	fn from(value: JsonValue) -> Self {
		let commit_info = value.0;
		let commit_value = commit_info.get("commit").unwrap().clone();
		let stats_value = commit_info.get("stats").unwrap().clone();
		Self {
			sha: commit_info.get("sha").and_then(|v| v.as_str()).unwrap().to_string(),
			commit: JsonValue(commit_value).into(),
			stats: JsonValue(stats_value).into(),
			files: commit_info
				.get("files")
				.and_then(|s| s.as_array())
				.map(|arr| arr.iter().map(|v| JsonValue(v.clone()).into()).collect())
				.unwrap_or_default(),
		}
	}
}

impl From<JsonValue> for CommitData {
	fn from(value: JsonValue) -> Self {
		let commit_data = value.0;
		let author_value = commit_data.get("author").unwrap().clone();
		let committer_value = commit_data.get("committer").unwrap().clone();
		Self {
			author: JsonValue(author_value).into(),
			committer: JsonValue(committer_value).into(),
			message: commit_data.get("message").and_then(|v| v.as_str()).unwrap().to_string(),
		}
	}
}

impl From<JsonValue> for UserInfo {
	fn from(value: JsonValue) -> Self {
		let user_info = value.0;
		Self {
			name: user_info.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
			email: user_info
				.get("email")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			avatar_url: user_info.get("avatar_url").and_then(|v| v.as_str()).unwrap().to_string(),
			date: user_info
				.get("date")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
		}
	}
}

impl From<JsonValue> for StatsInfo {
	fn from(value: JsonValue) -> Self {
		let stats_info = value.0;
		Self {
			total: stats_info.get("total").and_then(|v| v.as_u64()).unwrap_or(0),
			additions: stats_info.get("additions").and_then(|v| v.as_u64()).unwrap_or(0),
			deletions: stats_info.get("deletions").and_then(|v| v.as_u64()).unwrap_or(0),
		}
	}
}

impl From<JsonValue> for FileInfo {
	fn from(value: JsonValue) -> Self {
		let file_info = value.0;
		let status_value = file_info.get("status").cloned().unwrap();
		let additions_value = file_info.get("additions").and_then(|v| v.as_u64()).unwrap();
		let deletions_value = file_info.get("deletions").and_then(|v| v.as_u64()).unwrap();
		Self {
			file_name: file_info
				.get("name")
				.and_then(|v| v.as_str())
				.unwrap_or_default()
				.to_string(),
			status: JsonValue(status_value).into(),
			additions: additions_value,
			deletions: deletions_value,
			changes: additions_value + deletions_value,
		}
	}
}

impl From<JsonValue> for FileStatus {
	fn from(value: JsonValue) -> Self {
		match value.0.as_str().unwrap() {
			"add" => Self::Added,
			"modify" => Self::Modified,
			"delete" => Self::Deleted,
			"rename" => Self::Renamed,
			"copy" => Self::Copied,
			"change" => Self::Changed,
			_ => Self::UnChanged,
		}
	}
}
