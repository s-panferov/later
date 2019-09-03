use chrono::Weekday;
use nom::character::complete::alpha1;

use super::error::{ParseError, ParseResult};
use crate::types::RecurringInterval;

pub fn parse_weekday(input: &str) -> ParseResult<RecurringInterval> {
	let (input, dim) = alpha1(input)?;
	match dim {
		"weekday" => Ok((input, RecurringInterval::Weekday)),
		"weekend" => Ok((input, RecurringInterval::Weekend)),
		"monday" | "mon" => {
			Ok((input, RecurringInterval::DayOfWeek(Weekday::Mon)))
		}
		"tuesday" | "tue" => {
			Ok((input, RecurringInterval::DayOfWeek(Weekday::Tue)))
		}
		"wednesday" | "wed" => {
			Ok((input, RecurringInterval::DayOfWeek(Weekday::Wed)))
		}
		"thursday" | "thu" => {
			Ok((input, RecurringInterval::DayOfWeek(Weekday::Thu)))
		}
		"friday" | "fri" => {
			Ok((input, RecurringInterval::DayOfWeek(Weekday::Fri)))
		}
		"saturday" | "sat" => {
			Ok((input, RecurringInterval::DayOfWeek(Weekday::Sat)))
		}
		"sunday" | "sun" => {
			Ok((input, RecurringInterval::DayOfWeek(Weekday::Sun)))
		}
		_ => Err(ParseError::Unsupported.into_err(input)),
	}
}
