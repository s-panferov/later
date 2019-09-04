use chrono::Weekday;
use nom::character::complete::alpha1;

use super::error::{ParseError, ParseResult};
use crate::period::Period;

pub fn parse_weekday(input: &str) -> ParseResult<Period> {
	let (input, dim) = alpha1(input)?;
	match dim {
		"weekday" => Ok((input, Period::Weekday)),
		"weekend" => Ok((input, Period::Weekend)),
		"monday" | "mon" => Ok((input, Period::DayOfWeek(Weekday::Mon))),
		"tuesday" | "tue" => Ok((input, Period::DayOfWeek(Weekday::Tue))),
		"wednesday" | "wed" => Ok((input, Period::DayOfWeek(Weekday::Wed))),
		"thursday" | "thu" => Ok((input, Period::DayOfWeek(Weekday::Thu))),
		"friday" | "fri" => Ok((input, Period::DayOfWeek(Weekday::Fri))),
		"saturday" | "sat" => Ok((input, Period::DayOfWeek(Weekday::Sat))),
		"sunday" | "sun" => Ok((input, Period::DayOfWeek(Weekday::Sun))),
		_ => Err(ParseError::Unsupported.into_err(input)),
	}
}
