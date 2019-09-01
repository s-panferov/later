use nom::character::complete::alpha1;

use super::error::{ParseError, ParseResult};
use crate::types::Dimension;

pub fn parse_dimension(input: &str) -> ParseResult<&str, Dimension> {
	let (input, dim) = alpha1(input).map_err(ParseError::Layout)?;
	match dim {
		"s" => Ok((input, Dimension::Second)),
		"sec" => Ok((input, Dimension::Second)),
		"secs" => Ok((input, Dimension::Second)),
		"second" => Ok((input, Dimension::Second)),
		"seconds" => Ok((input, Dimension::Second)),
		"m" => Ok((input, Dimension::Minute)),
		"min" => Ok((input, Dimension::Minute)),
		"mins" => Ok((input, Dimension::Minute)),
		"minute" => Ok((input, Dimension::Minute)),
		"minutes" => Ok((input, Dimension::Minute)),
		"h" => Ok((input, Dimension::Hour)),
		"hr" => Ok((input, Dimension::Hour)),
		"hrs" => Ok((input, Dimension::Hour)),
		"hour" => Ok((input, Dimension::Hour)),
		"hours" => Ok((input, Dimension::Hour)),
		"d" => Ok((input, Dimension::Day)),
		"ds" => Ok((input, Dimension::Day)),
		"day" => Ok((input, Dimension::Day)),
		"days" => Ok((input, Dimension::Day)),
		"w" => Ok((input, Dimension::Week)),
		"wk" => Ok((input, Dimension::Week)),
		"wks" => Ok((input, Dimension::Week)),
		"week" => Ok((input, Dimension::Week)),
		"weeks" => Ok((input, Dimension::Week)),
		"mm" => Ok((input, Dimension::Month)),
		"month" => Ok((input, Dimension::Month)),
		"months" => Ok((input, Dimension::Month)),
		"q" => Ok((input, Dimension::Quarter)),
		"quarter" => Ok((input, Dimension::Quarter)),
		"quarters" => Ok((input, Dimension::Quarter)),
		"y" => Ok((input, Dimension::Year)),
		"yr" => Ok((input, Dimension::Year)),
		"yrs" => Ok((input, Dimension::Year)),
		"years" => Ok((input, Dimension::Year)),
		_ => Err(ParseError::UnknownDimension(input, dim)),
	}
}
