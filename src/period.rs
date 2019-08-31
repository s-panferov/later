use crate::interval::{AsIntervals, Interval};
use crate::types::Period;
use chrono::Duration;

use crate::merge::MergeN;
use crate::month::MonthIterator;

impl AsIntervals for Period {
	fn duration(&self) -> Duration {
		match self {
			Period::Fixed(d) => d.clone(),
			Period::Month(n) => Duration::weeks(4) * *n,
			Period::Quarter(n) => Duration::weeks(4) * 3 * *n,
			Period::Year(n) => Duration::weeks(4) * 12 * *n,
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
