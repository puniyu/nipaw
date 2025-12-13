use crate::CnbConfig;
use crate::common::JsonValue;
use chrono::Utc;
use nipaw_core::types::issue::{IssueInfo, LabelInfo, StateType, UserInfo};

impl From<JsonValue> for IssueInfo {
	fn from(issue: JsonValue) -> Self {
		let issue_info = issue.0;
		let is_open =
			issue_info.get("state").and_then(|v| v.as_str()).map(|s| s == "open").unwrap_or(false);
		let user_info = issue_info.get("user").unwrap().clone();
		let labels_info = issue_info.get("labels").unwrap().clone();
		Self {
			number: issue_info.get("number").and_then(|v| v.as_str()).unwrap().to_string(),
			state: if is_open { StateType::Opened } else { StateType::Closed },
			title: issue_info.get("title").and_then(|v| v.as_str()).unwrap().to_string(),
			body: issue_info
				.get("body")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			labels: JsonValue(labels_info).into(),
			user: JsonValue(user_info).into(),
			created_at: issue_info
				.get("created_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
			updated_at: issue_info
				.get("updated_at")
				.and_then(|v| v.as_str())
				.unwrap()
				.to_string()
				.parse()
				.unwrap(),
			closed_at: issue_info
				.get("closed_at")
				.and_then(|v| v.as_str())
				.and_then(|s| s.parse::<chrono::DateTime<Utc>>().ok()),
		}
	}
}

impl From<JsonValue> for UserInfo {
	fn from(user: JsonValue) -> Self {
		let user_info = user.0;
		let login = user_info.get("login").and_then(|v| v.as_str()).unwrap().to_string();
		let base_url = CnbConfig::default().base_url;
		Self {
			login: login.clone(),
			avatar_url: format!("{}/users/{}/avatar/l", base_url, login),
			email: Some(user_info.get("email").and_then(|v| v.as_str()).unwrap().to_string()),
		}
	}
}

impl From<JsonValue> for LabelInfo {
	fn from(label: JsonValue) -> Self {
		let label_info = label.0;
		Self {
			name: label_info.get("name").and_then(|v| v.as_str()).unwrap().to_string(),
			color: label_info.get("color").and_then(|v| v.as_str()).unwrap().to_string(),
		}
	}
}

impl From<JsonValue> for Vec<LabelInfo> {
	fn from(label: JsonValue) -> Self {
		label
			.0
			.as_array()
			.map(|arr| arr.iter().map(|v| JsonValue(v.clone()).into()).collect())
			.unwrap_or_default()
	}
}
