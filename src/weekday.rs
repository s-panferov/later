use chrono::{Datelike, Duration, Weekday};

use crate::interval::{AsIntervals, Interval};
use crate::utils::{end_of, start_of, Of};

impl AsIntervals for Weekday {
	fn duration_hint(&self) -> Duration {
		Duration::days(1)
	}

	fn iter_within(
		&self,
		interval: Interval,
	) -> Box<dyn Iterator<Item = Interval>> {
		Box::new(WeekdayIterator {
			weekday: self.clone(),
			interval,
			completed: false,
		})
	}
}

struct WeekdayIterator {
	interval: Interval,
	weekday: Weekday,
	completed: bool,
}

impl Iterator for WeekdayIterator {
	type Item = Interval;

	fn next(&mut self) -> Option<Interval> {
		if self.completed {
			return None;
		}

		let mut from = self.interval.from.clone();
		let current_day = from.weekday();

		if self.weekday != current_day {
			let target_num = self.weekday.num_days_from_monday();
			let current_num = current_day.num_days_from_monday();
			let diff = target_num as i32 - current_num as i32;
			let days_to_add = if diff > 0 {
				diff as i64
			} else {
				7 + diff as i64
			};
			from = from + Duration::days(days_to_add)
		}

		let end_of_day = end_of(Of::Day, &from);
		if self.interval.to.is_some() && self.interval.to.unwrap() < end_of_day
		{
			self.completed = true;
			Some(Interval {
				from,
				to: self.interval.to.clone(),
			})
		} else {
			self.interval.from =
				start_of(Of::Day, &(end_of_day + Duration::seconds(10)));

			Some(Interval {
				from,
				to: Some(end_of_day),
			})
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::Weekday;
	use insta::assert_debug_snapshot_matches;

	#[test]
	fn every_sunday() {
		let what: Vec<Interval> = Weekday::Sun
			.iter_within(Interval::from(
				"2019-01-01T00:00:00Z".parse().unwrap(),
			))
			.take(10)
			.collect();

		assert_debug_snapshot_matches!("every_sunday", what);
	}
}
