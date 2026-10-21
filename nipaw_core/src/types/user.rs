use std::collections::BTreeMap;

use jiff::{ToSpan, civil::Date};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
	/// 登录用户名
	pub login: String,
	/// 用户昵称
	pub name: Option<String>,
	/// 用户邮箱
	pub email: Option<String>,
	/// 用户头像URL
	pub avatar_url: String,
	/// 关注者数量
	pub followers: u64,
	/// 关注的用户数量
	pub following: u64,
	/// 公开仓库数量
	pub public_repo_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionDay {
	pub date: Date,
	pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionWeek {
	pub days: [ContributionDay; 7],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contribution {
	pub total: u32,
	pub weeks: Vec<ContributionWeek>,
}

impl FromIterator<ContributionDay> for Contribution {
	fn from_iter<T: IntoIterator<Item = ContributionDay>>(iter: T) -> Self {
		let mut weeks = BTreeMap::new();

		for day in iter {
			let index = day.date.weekday().to_monday_zero_offset() as usize;
			let start = day.date - (index as i64).days();
			let week = weeks.entry(start).or_insert_with(|| ContributionWeek {
				days: std::array::from_fn(|i| ContributionDay {
					date: start + (i as i64).days(),
					count: 0,
				}),
			});
			week.days[index] = day;
		}

		let weeks: Vec<_> = weeks.into_values().collect();
		let total = weeks.iter().flat_map(|week| &week.days).map(|day| day.count).sum();
		Self { total, weeks }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn groups_unsorted_days_into_complete_weeks_across_years() {
		let contribution: Contribution = [("2026-01-05", 4), ("2025-12-31", 2), ("2026-01-04", 3)]
			.into_iter()
			.map(|(date, count)| ContributionDay { date: date.parse().unwrap(), count })
			.collect();

		assert_eq!(contribution.total, 9);
		assert_eq!(contribution.weeks.len(), 2);
		assert_eq!(
			contribution.weeks[0].days.each_ref().map(|day| day.date.to_string()),
			[
				"2025-12-29",
				"2025-12-30",
				"2025-12-31",
				"2026-01-01",
				"2026-01-02",
				"2026-01-03",
				"2026-01-04"
			]
		);
		assert_eq!(
			contribution.weeks[0].days.each_ref().map(|day| day.count),
			[0, 0, 2, 0, 0, 0, 3]
		);
		assert_eq!(contribution.weeks[1].days[0].date.to_string(), "2026-01-05");
		assert_eq!(contribution.weeks[1].days[6].date.to_string(), "2026-01-11");
		assert_eq!(
			contribution.weeks[1].days.each_ref().map(|day| day.count),
			[4, 0, 0, 0, 0, 0, 0]
		);
	}

	#[test]
	fn duplicate_dates_keep_the_last_count_without_inflating_total() {
		let contribution: Contribution = [("2026-01-01", 2), ("2026-01-01", 7)]
			.into_iter()
			.map(|(date, count)| ContributionDay { date: date.parse().unwrap(), count })
			.collect();

		assert_eq!(contribution.total, 7);
		assert_eq!(contribution.weeks.len(), 1);
		assert_eq!(contribution.weeks[0].days[3].count, 7);
	}

	#[test]
	fn empty_input_has_no_weeks_or_contributions() {
		let contribution: Contribution = std::iter::empty::<ContributionDay>().collect();

		assert_eq!(contribution.total, 0);
		assert!(contribution.weeks.is_empty());
	}
}
