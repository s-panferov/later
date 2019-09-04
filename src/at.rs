use chrono::Timelike;
use chrono::{Duration, NaiveTime};

use crate::interval::{AsIntervals, Interval};
use crate::utils::{end_of, Of};

impl AsIntervals for NaiveTime {
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
		let from = self.interval.from.clone();
		let from_time = from.time();

		let time = if from_time < self.time {
			from.with_hour(self.time.hour()).and_then(|time| time.with_minute(self.time.minute()))
		} else {
			let from = end_of(Of::Day, from) + Duration::seconds(1);
		};

		if self.interval.to.is_some()
			&& to > *self.interval.to.as_ref().unwrap()
		{
			return None;
		}

		self.interval.from = to;
		Some(Interval { from, to: Some(to) })
	}
}

#[derive(Debug, PartialEq)]
pub enum At {
	Time(NaiveTime),
}
