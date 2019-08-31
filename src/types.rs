use chrono::Duration;

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

#[derive(Debug)]
pub enum Period {
	Fixed(Duration),
	Month(i32),
	Quarter(i32),
	Year(i32),
}

#[derive(Debug)]
pub enum IntervalSpecifier {
	Period(Period),
}
