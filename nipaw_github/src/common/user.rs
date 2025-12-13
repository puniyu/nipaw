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

		let cell_selector = Selector::parse("td.ContributionCalendar-day").unwrap();
		let total_selector = Selector::parse("h2#js-contribution-activity-description").unwrap();
		let tooltip_selector = Selector::parse("tool-tip").unwrap();

		let total = document
			.select(&total_selector)
			.next()
			.map(|element| {
				element
					.text()
					.collect::<String>()
					.split_whitespace()
					.next()
					.unwrap_or("0")
					.replace(",", "")
					.parse::<u32>()
					.unwrap_or(0)
			})
			.unwrap_or(0);

		let contributions = document
			.select(&cell_selector)
			.filter_map(|element| {
				let date_str = element.value().attr("data-date")?;
				let id = element.value().attr("id")?;

				let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
					.ok()?
					.and_hms_opt(0, 0, 0)?
					.and_local_timezone(Utc)
					.latest()?;
				let mut count = 0;
				if let Some(tooltip) =
					document.select(&tooltip_selector).find(|e| e.value().attr("for") == Some(id))
				{
					let tooltip_text = tooltip.text().collect::<String>();
					if !tooltip_text.contains("No contributions") {
						let contributions_str = tooltip_text.split_whitespace().next().unwrap();
						count = contributions_str.parse().unwrap_or(1);
					}
				}

				Some(ContributionData { date, count })
			})
			.sorted_by_key(|c| c.date)
			.chunk_by(|c| {
				let naive_date = c.date.naive_utc().date();
				naive_date.week(Weekday::Mon)
			})
			.into_iter()
			.map(|(_, chunk)| chunk.collect::<Vec<_>>())
			.collect();

		Self { total, contributions }
	}
}
