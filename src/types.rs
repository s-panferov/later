use chrono::{Duration, NaiveTime, Weekday};

#[derive(Debug)]
pub enum Dimension {
	Second,
	Minute,
	Hour,
	Day,
	Week,
	Month,
	Quarter,
	Year,
}

