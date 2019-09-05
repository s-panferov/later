use chrono::{Duration, NaiveTime, Timelike};

use crate::interval::{Interval, Timeline};
use crate::utils::{end_of, start_of, Of};

#[derive(Debug, Clone)]
struct TimeFrame {
	pub from: NaiveTime,
	pub to: NaiveTime,
}

struct TimeFrameIterator {
	frame: TimeFrame,
	interval: Interval,
	completed: bool,
}

impl Timeline for TimeFrame {
	fn duration_hint(&self) -> Duration {
		if self.to > self.from {
			self.to - self.from
		} else {
			Duration::days(1)
				- self
					.from
					.signed_duration_since(NaiveTime::from_hms(0, 0, 0))
				+ self.to.signed_duration_since(NaiveTime::from_hms(0, 0, 0))
		}
	}

	fn iter_within(
		&self,
		interval: Interval,
	) -> Box<dyn Iterator<Item = Interval>> {
		Box::new(TimeFrameIterator {
			frame: self.clone(),
			interval,
			completed: false,
		})
	}
}

impl Iterator for TimeFrameIterator {
	type Item = Interval;

	fn next(&mut self) -> Option<Interval> {
		if self.completed {
			return None;
		}

		let mut from = self.interval.from.clone();
		if from.time() > self.frame.from {
			// Jump to the next day
			from = start_of(
				Of::Day,
				&(end_of(Of::Day, &from) + Duration::seconds(1)),
			);
		}

		from = from
			.with_hour(self.frame.from.hour())
			.and_then(|time| time.with_minute(self.frame.from.minute()))
			.and_then(|time| time.with_second(self.frame.from.second()))
			.unwrap();

		if self.interval.to.is_some()
			&& from > *self.interval.to.as_ref().unwrap()
		{
			self.completed = true;
			return None;
		}

		let mut to = from;
		if to.time() > self.frame.to {
			// Jump to the next day
			to = start_of(
				Of::Day,
				&(end_of(Of::Day, &from) + Duration::seconds(1)),
			);
		}

		to = to
			.with_hour(self.frame.to.hour())
			.and_then(|time| time.with_minute(self.frame.to.minute()))
      .and_then(|time| time.with_second(self.frame.to.second()))
			.unwrap();

		if self.interval.to.is_some()
			&& to > *self.interval.to.as_ref().unwrap()
		{
			self.completed = true;
			return None;
		}

		self.interval.from = to + Duration::seconds(1);

		Some(Interval { from, to: Some(to) })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::interval::{Interval, Timeline};
	use insta::assert_debug_snapshot_matches;

	#[test]
	fn from_23_00_to_1_00() {
		let frame = TimeFrame {
			from: NaiveTime::from_hms(23, 0, 0),
			to: NaiveTime::from_hms(1, 0, 0),
		};

		assert_debug_snapshot_matches!(
			"from_23_00_to_1_00",
			frame
				.iter_within(Interval::from(
					"2019-10-31T00:00:00Z".parse().unwrap(),
				))
				.take(10)
				.collect::<Vec<_>>()
		);
	}
}
