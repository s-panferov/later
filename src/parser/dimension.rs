use nom::character::complete::alpha1;

use super::error::{ParseError, ParseResult};
use crate::types::Dimension;

pub fn parse_dimension(input: &str) -> ParseResult<Dimension> {
	let (input, dim) = alpha1(input)?;
	match dim {
		"s" | "sec" | "secs" | "second" | "seconds" => {
			Ok((input, Dimension::Second))
		}
		"m" | "min" | "mins" | "minute" | "minutes" => {
			Ok((input, Dimension::Minute))
		}
		"h" | "hr" | "hrs" | "hour" | "hours" => Ok((input, Dimension::Hour)),
		"d" | "ds" | "day" | "days" => Ok((input, Dimension::Day)),
		"w" | "wk" | "wks" | "week" | "weeks" => Ok((input, Dimension::Week)),
		"mm" | "month" | "months" => Ok((input, Dimension::Month)),
		"q" | "quarter" | "quarters" | "y" => Ok((input, Dimension::Year)),
		"yr" | "yrs" | "years" => Ok((input, Dimension::Year)),
		_ => Err(ParseError::UnknownDimension {}.into_err(input)),
	}
}
