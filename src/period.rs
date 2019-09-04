use crate::interval::{AsIntervals, Interval};
use chrono::{Duration, Weekday};

use crate::merge::MergeN;
use crate::month::MonthIterator;

#[derive(Debug, PartialEq)]
pub enum Period {
	Fixed(Duration),
	Month(i32),
	Quarter(i32),
	Year(i32),
	Weekend,
	Weekday,
	DayOfWeek(Weekday),
	Ordinal(usize, Box<Period>),
}

impl AsIntervals for Period {
	fn duration_hint(&self) -> Duration {
		match self {
			Period::Fixed(d) => d.clone(),
			Period::Month(n) => Duration::weeks(4) * *n,
			Period::Quarter(n) => Duration::weeks(4) * 3 * *n,
			Period::Year(n) => Duration::weeks(4) * 12 * *n,
			Period::Weekday | Period::Weekend | Period::DayOfWeek(_) => {
				Duration::days(1)
			}
			Period::Ordinal(_, p) => p.duration_hint(),
		}
	}

	fn iter_within(
		&self,
		interval: Interval,
	) -> Box<dyn Iterator<Item = Interval>> {
		match self {
			Period::Fixed(d) => d.iter_within(interval),
			Period::Month(m) => {
				Box::new(MergeN::new(MonthIterator::new(interval), *m as usize))
			}
			Period::Quarter(q) => Box::new(MergeN::new(
				MonthIterator::new(interval),
				(q * 3) as usize,
			)),
			Period::Year(y) => Box::new(MergeN::new(
				MonthIterator::new(interval),
				(y * 12) as usize,
			)),
			Period::DayOfWeek(w) => Box::new(w.iter_within(interval)),
			Period::Weekday => unimplemented!(),
			Period::Weekend => unimplemented!(),
			Period::Ordinal(m, p) => {
				Box::new(p.iter_within(interval).step_by(*m))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::interval::{AsIntervals, Interval};
	use insta::assert_debug_snapshot_matches;

	#[test]
	fn every_month() {
		let every_month = Period::Month(1);

		assert_debug_snapshot_matches!(
			"every_month",
			every_month
				.iter_within(Interval::from(
					"2019-10-31T12:00:00Z".parse().unwrap(),
				))
				.take(10)
				.collect::<Vec<_>>()
		);
	}
}
