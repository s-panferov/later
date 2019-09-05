use chrono::Timelike;
use chrono::{Duration, NaiveTime};

use crate::interval::{Interval, Timeline};
use crate::utils::{end_of, start_of, Of};

#[derive(Debug, PartialEq)]
pub struct At(Vec<NaiveTime>);

impl At {
	pub fn new(times: Vec<NaiveTime>) -> Self {
		At(times)
	}
}