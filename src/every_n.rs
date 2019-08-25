use chrono::Duration;

use crate::interval::{AsIntervals, Interval};

#[derive(Debug)]
pub struct EveryN(Duration, i32);

impl EveryN {
	pub fn new(d: Duration, n: i32) -> EveryN {
		EveryN(d, n)
	}
}

impl AsIntervals for EveryN {
	fn duration(&self) -> &Duration {
		&self.0
	}

	fn iter_within(
		&self,
		interval: Interval,
	) -> Box<dyn Iterator<Item = Interval>> {
		Box::new(EveryNIterator {
			duration: self.0.clone(),
			interval,
			n: self.1,
		})
	}
}

struct EveryNIterator {
	duration: Duration,
	interval: Interval,
	n: i32,
}

impl Iterator for EveryNIterator {
	type Item = Interval;
	fn next(&mut self) -> Option<Interval> {
		let from = self.interval.from.clone();
		let to = self.interval.from + self.duration;

		if self.interval.to.is_some()
			&& to >= *self.interval.to.as_ref().unwrap()
		{
			return None;
		}

		self.interval.from = to + self.duration * (self.n - 1);
		Some(Interval { from, to: Some(to) })
	}
}
