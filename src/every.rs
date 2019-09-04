use chrono::Duration;

use crate::combine::Combine;
use crate::interval::{AsIntervals, Interval};
use crate::period::Period;

#[derive(Debug, PartialEq)]
pub struct Every {
	periods: Vec<Period>,
}

impl Every {
	pub fn new(periods: Vec<Period>) -> Self {
		Every { periods }
	}
}

impl AsIntervals for Every {
	fn duration_hint(&self) -> Duration {
		self.periods.get(0).map(|p| p.duration_hint()).unwrap()
	}

	fn iter_within(
		&self,
		interval: Interval,
	) -> Box<dyn Iterator<Item = Interval>> {
		Box::new(Combine::new(
			self.periods
				.iter()
				.map(|p| p.iter_within(interval.clone()))
				.collect(),
		))
	}
}
