use chrono::{DateTime, Duration, Utc};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Interval {
	pub from: DateTime<Utc>,
	pub to: Option<DateTime<Utc>>,
}

impl Interval {
	pub fn from_now() -> Self {
		Interval {
			from: Utc::now(),
			to: None,
		}
	}

	pub fn from(from: DateTime<Utc>) -> Self {
		Interval { from, to: None }
	}
}

/// Something that resolves to the range of time
pub trait AsIntervals: Debug {
	/// Get the estimate of the interval
	fn duration_hint(&self) -> Duration;

	/// Get an interator to resolve intervals
	fn iter_within(
		&self,
		interval: Interval,
	) -> Box<dyn Iterator<Item = Interval>>;
}
