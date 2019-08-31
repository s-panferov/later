use crate::interval::Interval;
use crate::utils::days_in_month;
use chrono::{Datelike, TimeZone, Timelike};

pub struct MonthIterator {
	interval: Interval,
	start_day: u32,
}

impl MonthIterator {
	pub fn new(interval: Interval) -> MonthIterator {
		let start_day = interval.from.day();
		MonthIterator {
			interval,
			start_day,
		}
	}
}

impl Iterator for MonthIterator {
	type Item = Interval;
	fn next(&mut self) -> Option<Interval> {
		let from = self.interval.from.clone();
		let month = from.month();

		let next_year = if month == 12 {
			from.year() + 1
		} else {
			from.year()
		};

		let next_month = if month == 12 { 1 } else { month + 1 };

		let days_in_next_month = days_in_month(next_year, next_month);
		let to = from;
		let next_day = if days_in_next_month < self.start_day {
			days_in_next_month
		} else {
			self.start_day
		};

		let to = to
			.offset()
			.ymd(next_year, next_month, next_day)
			.and_hms_nano(to.hour(), to.minute(), to.second(), to.nanosecond());

		if self.interval.to.is_some()
			&& to > *self.interval.to.as_ref().unwrap()
		{
			return None;
		}

		self.interval.from = to;

		Some(Interval { from, to: Some(to) })
	}
}
