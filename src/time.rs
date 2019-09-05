use chrono::Timelike;
use chrono::{Duration, NaiveTime};

use crate::interval::{Interval, Timeline};
use crate::utils::{end_of, start_of, Of};

impl Timeline for NaiveTime {
	fn duration_hint(&self) -> Duration {
		Duration::days(1)
	}

	fn iter_within(
		&self,
		interval: Interval,
	) -> Box<dyn Iterator<Item = Interval>> {
		Box::new(TimeIterator {
			time: self.clone(),
			interval,
			completed: false,
		})
	}
}

struct TimeIterator {
	time: NaiveTime,
	interval: Interval,
	completed: bool,
}

impl Iterator for TimeIterator {
	type Item = Interval;

	fn next(&mut self) -> Option<Interval> {
		if self.completed {
			return None;
		}

		let mut from = self.interval.from.clone();
		let from_time = from.time();

		if from_time > self.time {
			// Jump to the next day
			from = start_of(
				Of::Day,
				&(end_of(Of::Day, &from) + Duration::seconds(1)),
			);
		}

		let datetime = from
			.with_hour(self.time.hour())
			.and_then(|time| time.with_minute(self.time.minute()))
      .and_then(|time| time.with_second(self.time.second()))
			.unwrap();

		if self.interval.to.is_some()
			&& datetime > *self.interval.to.as_ref().unwrap()
		{
			self.completed = true;
			return None;
		}

		self.interval.from = datetime + Duration::seconds(1);

		Some(Interval {
			from: datetime,
			to: Some(datetime),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::interval::{Interval, Timeline};
	use insta::assert_debug_snapshot_matches;

	#[test]
	fn every_10_am() {
		let am_10 = NaiveTime::from_hms(10, 0, 0);

		assert_debug_snapshot_matches!(
			"every_10_am",
			am_10
				.iter_within(Interval::from(
					"2019-10-31T12:00:00Z".parse().unwrap(),
				))
				.take(10)
				.collect::<Vec<_>>()
		);
	}
}
