use crate::common::JsonValue;
use chrono::{NaiveDate, Utc, Weekday};
use itertools::Itertools;
use nipaw_core::types::user::{ContributionData, ContributionResult, UserInfo};

impl From<JsonValue> for UserInfo {
	fn from(json_value: JsonValue) -> Self {
		let user_info = json_value.0;
		let login = user_info.get("username").and_then(|v| v.as_str()).unwrap();
		let base_url = "https://cnb.cool";
		Self {
			login: login.to_string(),
			name: user_info
				.get("nickname")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			avatar_url: format!("{}/{}", base_url, login),
			email: user_info
				.get("email")
				.and_then(|v| v.as_str())
				.filter(|s| !s.is_empty())
				.map(|s| s.to_string()),
			followers: user_info.get("follower_count").and_then(|v| v.as_u64()).unwrap(),
			following: user_info.get("follow_count").and_then(|v| v.as_u64()).unwrap(),
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
			.map(|(date_str, data)| {
				let date = NaiveDate::parse_from_str(date_str, "%Y%m%d")
					.unwrap()
					.and_hms_opt(0, 0, 0)
					.unwrap()
					.and_local_timezone(Utc)
					.unwrap();
				let count = data.get("score").and_then(|v| v.as_u64()).unwrap_or(0) as u32;

				ContributionData { date, count }
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

		Self { total, contributions }
	}
}
