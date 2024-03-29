use crate::interval::{Interval, Timeline};
use chrono::Duration;

impl Timeline for Duration {
	fn duration_hint(&self) -> Duration {
		self.clone()
	}

	fn iter_within(
		&self,
		interval: Interval,
	) -> Box<dyn Iterator<Item = Interval>> {
		Box::new(DurationIterator {
			duration: self.clone(),
			interval,
		})
	}
}

struct DurationIterator {
	duration: Duration,
	interval: Interval,
}

impl Iterator for DurationIterator {
	type Item = Interval;
	fn next(&mut self) -> Option<Interval> {
		let from = self.interval.from.clone();
		let to = self.interval.from + self.duration;

		if self.interval.to.is_some()
			&& to > *self.interval.to.as_ref().unwrap()
		{
			return None;
		}

		self.interval.from = to;
		Some(Interval { from, to: Some(to) })
	}
}
