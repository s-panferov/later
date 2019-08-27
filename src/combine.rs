use crate::interval::Interval;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
enum Variant {
	Finished(usize),
	Fresh(usize, Interval),
	Taken(usize),
}

type Iterators = Vec<Box<dyn Iterator<Item = Interval>>>;

/// An iterator that combines a set of other iterators
/// and produces a consequest stream of Intervals without overlaps.
struct Combine {
	iterators: Iterators,
	state: Vec<Variant>,
	prev: Option<Interval>,
	completed: bool,
}

impl Combine {
	fn new(iterators: Iterators) -> Self {
		let state = iterators
			.iter()
			.enumerate()
			.map(|(i, _)| Variant::Taken(i))
			.collect();

		Combine {
			iterators,
			state,
			prev: None,
			completed: false,
		}
	}
}

impl Iterator for Combine {
	type Item = Interval;

	fn next(&mut self) -> Option<Interval> {
		if self.completed {
			return None;
		}

		loop {
			let mut indexes: Vec<(usize, usize)> = vec![];

			self.state.iter().enumerate().for_each(|(i, v)| match v {
				Variant::Finished(_) => {}
				Variant::Fresh(_, _) => {}
				Variant::Taken(j) => indexes.push((i, j.clone())),
			});

			for (i, j) in indexes {
				let iterator = self.iterators.get_mut(j).unwrap();
				self.state[i] = match iterator.next() {
					Some(interval) => Variant::Fresh(j, interval),
					None => Variant::Finished(j),
				}
			}

			self.state.sort_by(|a, b| match a {
				Variant::Finished(_) => Ordering::Less,
				Variant::Taken(_) => Ordering::Less,
				Variant::Fresh(_, a) => match b {
					Variant::Finished(_) => Ordering::Greater,
					Variant::Taken(_) => Ordering::Greater,
					Variant::Fresh(_, b) => {
						if a.from > b.from {
							Ordering::Greater
						} else if a.from < b.from {
							Ordering::Less
						} else {
							if a.to.is_none() {
								Ordering::Greater
							} else if b.to.is_none() {
								Ordering::Less
							} else {
								a.to.unwrap().cmp(&b.to.unwrap())
							}
						}
					}
				},
			});

			println!("{:?}", self.state);

			let mut result: Option<Interval> = None;
			for v in &mut self.state {
				match v {
					Variant::Finished(_) => {}
					Variant::Taken(_) => {}
					Variant::Fresh(i, _) => {
						let mut tmp = Variant::Taken(i.clone());
						std::mem::swap(v, &mut tmp);
						match tmp {
							Variant::Finished(_) => {}
							Variant::Taken(_) => {}
							Variant::Fresh(_, v) => result = Some(v),
						}
						break;
					}
				}
			}

			if result.is_some() {
				let interval = result.as_mut().unwrap();
				if interval.to.is_none() {
					// If our current interval does not have an end we don't need to produce
					// anything else because it would overlap.
					self.completed = true;
					return result;
				} else {
					if self.prev.is_some() {
						let to = self.prev.as_ref().unwrap().to.unwrap();
						if interval.from < to {
							interval.from = to.clone();
							if interval.from > interval.to.unwrap() {
								// We consumed the whole interval
								continue;
							}
						}
					}
					self.prev = result.clone();
					return result;
				}
			} else {
				self.completed = true;
				return None;
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::interval::AsIntervals;
	use chrono::Weekday;
	use insta::assert_debug_snapshot_matches;

	#[test]
	fn combine_weekend() {
		let stream = Combine::new(vec![
			Weekday::Sat.iter_within(Interval::from(
				"2019-01-01T00:00:00Z".parse().unwrap(),
			)),
			Weekday::Sun.iter_within(Interval::from(
				"2019-01-01T00:00:00Z".parse().unwrap(),
			)),
		]);

		let what: Vec<Interval> = stream.take(10).collect();
		assert_debug_snapshot_matches!("combine_weekend", what);
	}
}
