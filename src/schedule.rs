use crate::interval::{AsIntervals, Interval};

pub struct Schedule {
	items: Vec<Box<dyn AsIntervals>>,
	bounds: Option<Interval>,
}

impl Schedule {
	pub fn new(
		mut items: Vec<Box<dyn AsIntervals>>,
		bounds: Option<Interval>,
	) -> Self {
		items.sort_by(|a, b| b.duration_hint().cmp(&a.duration_hint()));
		Schedule { items, bounds }
	}

	/// Get an interator to resolve intervals
	pub fn iter_within(&self, interval: Interval) -> ScheduleIterator {
		ScheduleIterator::new(self, interval)
	}
}

pub struct ScheduleIterator<'a> {
	interval: Interval,
	schedule: &'a Schedule,
	initialized: bool,
	state: Vec<Box<dyn Iterator<Item = Interval>>>,
}

impl<'a> ScheduleIterator<'a> {
	fn new(schedule: &'a Schedule, interval: Interval) -> Self {
		ScheduleIterator {
			schedule,
			interval,
			initialized: false,
			state: Vec::new(),
		}
	}
}

impl<'a> ScheduleIterator<'a> {
	fn init(&mut self, mut i: usize, interval: Interval) -> Option<Interval> {
		let mut int = interval.clone();
		let len = self.schedule.items.len();
		while i < len {
			let mut iter =
				self.schedule.items.get(i).unwrap().iter_within(int.clone());
			let next = iter.next();
			match next {
				Some(next) => {
					int = next;
					self.state.push(iter);
				}
				None => return None,
			}
			i += 1;
		}

		Some(int)
	}
}

impl<'a> Iterator for ScheduleIterator<'a> {
	type Item = Interval;

	fn next(&mut self) -> Option<Interval> {
		if !self.initialized {
			let len = self.schedule.items.len();
			if len == 0 {
				return None;
			}

			let int = self.init(0, self.interval.clone());
			match int {
				Some(int) => {
					self.initialized = true;
					return Some(int);
				}
				None => return None,
			}
		}

		let curr_iter = self.state.last_mut().unwrap();
		let next = curr_iter.next();

		match next {
			Some(next) => return Some(next),
			None => loop {
				self.state.pop();
				let len = self.state.len();
				if len == 0 {
					return None;
				}
				let curr_iter = self.state.last_mut().unwrap();
				let next = curr_iter.next();
				match next {
					Some(next) => return self.init(len, next),
					None => continue,
				}
			},
		}
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use chrono::Duration;

// 	use insta::assert_debug_snapshot_matches;

// 	#[test]
// 	fn every() {
// 		let schedule = Schedule::from_parts(vec![
// 			SchedulePart::Every(Box::new(Duration::minutes(10))),
// 			SchedulePart::Every(Box::new(Duration::hours(1).nth(2))),
// 		]);§1

// 		let what: Vec<Interval> = schedule
// 			.iter_within(Interval::from(
// 				"2019-01-01T00:00:00Z".parse().unwrap(),
// 			))
// 			.take(10)
// 			.collect();

// 		assert_debug_snapshot_matches!("every", what);
// 	}
// }
