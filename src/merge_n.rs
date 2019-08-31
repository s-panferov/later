use chrono::Duration;

use crate::interval::Interval;
use crate::merge::Merge;

/// An iterator that merges consequest periods of time emitted by
/// another iterator.
pub struct MergeN<T: Iterator<Item = Interval>> {
	base: T,
	how_many: usize,
	completed: bool,
}

impl<T: Iterator<Item = Interval>> MergeN<T> {
	pub fn new(base: T, how_many: usize) -> Self {
		MergeN {
			base,
			how_many,
			completed: false,
		}
	}
}

impl<T: Iterator<Item = Interval>> Iterator for MergeN<T> {
	type Item = Interval;

	fn next(&mut self) -> Option<Interval> {
		if self.completed {
			return None;
		}

		let items: Vec<_> = self.base.by_ref().take(self.how_many).collect();

		if items.len() < self.how_many {
			self.completed = true;
			return None;
		}

		let mut merge =
			Merge::new(items.into_iter(), Duration::seconds(1), self.how_many);

		merge.next()
	}
}
