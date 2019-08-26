use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};

pub enum Of {
	Day,
	Month,
	Year,
}

pub fn end_of(of: Of, date: &DateTime<Utc>) -> DateTime<Utc> {
	match of {
		Of::Year => end_of(Of::Month, &date.with_month(12).unwrap()),
		Of::Month => end_of(
			Of::Day,
			&date
				.with_day(days_in_month(date.year(), date.month()))
				.unwrap(),
		),
		Of::Day => date
			.with_hour(23)
			.unwrap()
			.with_minute(59)
			.unwrap()
			.with_second(59)
			.unwrap()
			.with_nanosecond(999999999)
			.unwrap(),
	}
}

pub fn start_of(of: Of, date: &DateTime<Utc>) -> DateTime<Utc> {
	match of {
		Of::Year => start_of(Of::Month, &date.with_month(1).unwrap()),
		Of::Month => start_of(Of::Day, &date.with_day(1).unwrap()),
		Of::Day => date
			.with_hour(0)
			.unwrap()
			.with_minute(0)
			.unwrap()
			.with_second(0)
			.unwrap()
			.with_nanosecond(0)
			.unwrap(),
	}
}

pub fn days_in_month(year: i32, m: u32) -> u32 {
	if m == 12 {
		NaiveDate::from_ymd(year + 1, 1, 1)
	} else {
		NaiveDate::from_ymd(year, m + 1, 1)
	}
	.signed_duration_since(NaiveDate::from_ymd(year, m, 1))
	.num_days() as u32
}
