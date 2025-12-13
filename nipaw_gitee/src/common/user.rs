use crate::common::{Html, JsonValue};
use chrono::{NaiveDate, Utc, Weekday};
use itertools::Itertools;
use nipaw_core::types::user::{ContributionData, ContributionResult, UserInfo};
use scraper::Selector;

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
			public_repo_count: user_info.get("public_repos").and_then(|v| v.as_u64()).unwrap(),
		}
	}
}

impl From<Html> for ContributionResult {
	fn from(value: Html) -> Self {
		let html = value.0;
		let document = scraper::Html::parse_document(&html);

		let selector = Selector::parse("div.right-side div.box").unwrap();

		let contributions = document
			.select(&selector)
			.filter_map(|element| {
				let data_content = element.value().attr("data-content")?;
				let date_str = element.value().attr("date")?;

				let count = data_content
					.split('个')
					.next()?
					.rsplit(':')
					.next()?
					.parse::<u32>()
					.unwrap_or(0);
				let date = NaiveDate::parse_from_str(date_str, "%Y%m%d")
					.ok()?
					.and_hms_opt(0, 0, 0)?
					.and_local_timezone(Utc)
					.unwrap();

				Some(ContributionData { date, count })
			})
			.sorted_by_key(|c| c.date)
			.chunk_by(|c| {
				let naive_date = c.date.naive_utc().date();
				naive_date.week(Weekday::Mon)
			})
			.into_iter()
			.map(|(_, chunk)| chunk.collect::<Vec<_>>())
			.collect::<Vec<Vec<ContributionData>>>();

		let total = contributions.iter().flatten().map(|c| c.count).sum();

		Self { total, contributions }
	}
}
