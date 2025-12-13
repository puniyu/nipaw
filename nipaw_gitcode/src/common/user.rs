use crate::common::JsonValue;
use chrono::{NaiveDate, Utc, Weekday};
use itertools::Itertools;
use nipaw_core::types::user::{ContributionData, ContributionResult, UserInfo};

impl From<JsonValue> for UserInfo {
	fn from(json_value: JsonValue) -> Self {
		let user_info = json_value.0;
		Self {
			login: user_info.get("login").and_then(|v| v.as_str()).unwrap().to_string(),
			name: user_info
				.get("name")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			avatar_url: user_info.get("avatar_url").and_then(|v| v.as_str()).unwrap().to_string(),
			email: user_info
				.get("email")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			followers: user_info.get("followers").and_then(|v| v.as_u64()).unwrap(),
			following: user_info.get("following").and_then(|v| v.as_u64()).unwrap(),
			public_repo_count: user_info.get("repo_count").and_then(|v| v.as_u64()).unwrap_or(0),
		}
	}
}

impl From<JsonValue> for ContributionResult {
	fn from(value: JsonValue) -> Self {
		let contribution_result = value.0;

		let contributions = contribution_result
			.as_object()
			.unwrap()
			.iter()
			.map(|(date, count)| ContributionData {
				date: NaiveDate::parse_from_str(date, "%Y-%m-%d")
					.map(|nd| nd.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Utc).unwrap())
					.unwrap(),
				count: count.as_u64().unwrap() as u32,
			})
			.sorted_by_key(|c| c.date)
			.chunk_by(|c| {
				let naive_date = c.date.naive_utc().date();
				naive_date.week(Weekday::Mon)
			})
			.into_iter()
			.map(|(_, week_data)| week_data.collect::<Vec<_>>())
			.collect::<Vec<Vec<ContributionData>>>();

		let total = contributions.iter().flatten().map(|c| c.count).sum();

		Self { contributions, total }
	}
}
