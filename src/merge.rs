use crate::interval::Interval;
use chrono::Duration;

pub use crate::merge_n::MergeN;

/// An iterator that merges consequest periods of time emitted by
/// another iterator.
pub struct Merge<T: Iterator<Item = Interval>> {
	prev: Option<Interval>,
	base: T,
	diff: Duration,
	completed: bool,
	max_merges: usize,
	merges: usize,
}

impl<T: Iterator<Item = Interval>> Merge<T> {
	pub fn new(base: T, diff: Duration, max_merges: usize) -> Self {
		Merge {
			base,
			prev: None,
			diff,
			completed: false,
			max_merges,
			merges: 0,
		}
	}
}

impl<T: Iterator<Item = Interval>> Iterator for Merge<T> {
	type Item = Interval;

	fn next(&mut self) -> Option<Interval> {
		if self.completed {
			return None;
		}

		self.merges = 0;

		loop {
			if self.merges > self.max_merges {
				self.completed = true;
				return None;
			}

			let value = self.base.next();
			if value.is_none() {
				self.completed = true;
				return std::mem::replace(&mut self.prev, None);
			}

			let value = value.unwrap();

			match &mut self.prev {
				None => {
					if value.to.is_none() {
						// all other intervals will be merged into this one anyway
						self.completed = true;
						return Some(value);
					} else {
						self.prev = Some(value);
					}
				}
				Some(prev) => {
					let to = prev.to.as_ref().unwrap();
					if value.from < *to || value.from - *to <= self.diff {
						// merge intervals
						prev.to = value.to;
						self.merges += 1;
						continue;
					} else {
						return std::mem::replace(&mut self.prev, Some(value));
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::combine::Combine;
	use crate::interval::AsIntervals;
	use chrono::Weekday;
	use insta::assert_debug_snapshot_matches;

	#[test]
	fn merge_weekend() {
		let stream = Combine::new(vec![
			Weekday::Sat.iter_within(Interval::from(
				"2019-01-01T00:00:00Z".parse().unwrap(),
			)),
			Weekday::Sun.iter_within(Interval::from(
				"2019-01-01T00:00:00Z".parse().unwrap(),
			)),
		]);

		let stream = Merge::new(stream, Duration::seconds(1), 100);
		let what: Vec<Interval> = stream.take(10).collect();
		assert_debug_snapshot_matches!("merge_weekend", what);
	}
}
