use chrono::Duration;
use nom::{
	bytes::complete::tag,
	character::complete::{alpha1, digit1, space1},
	error::ErrorKind,
	Err,
};

use crate::types::{Dimension, Period};

pub enum ParseError<'a, I> {
	Layout(Err<(I, ErrorKind)>),
	UnknownDimension(I, &'a str),
	InvalidDuration(I, std::num::ParseIntError),
}

pub type ParseResult<'a, I, O> = Result<(I, O), ParseError<'a, I>>;

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

pub fn parse_period(input: &str) -> ParseResult<&str, Period> {
	let (input, _) = tag("every")(input).map_err(ParseError::Layout)?;
	let (input, _) = space1(input).map_err(ParseError::Layout)?;
	let (input, num) = digit1(input).map_err(ParseError::Layout)?;
	let (input, dim) = parse_dimension(input)?;
	let num: i64 = num
		.parse()
		.map_err(|e| ParseError::InvalidDuration(num, e))?;

	let duration = match dim {
		Dimension::Second => Period::Fixed(Duration::seconds(num)),
		Dimension::Minute => Period::Fixed(Duration::minutes(num)),
		Dimension::Hour => Period::Fixed(Duration::hours(num)),
		Dimension::Day => Period::Fixed(Duration::days(num)),
		Dimension::Week => Period::Fixed(Duration::weeks(num)),
		Dimension::Month => Period::Month(num as i32),
		Dimension::Quarter => Period::Quarter(num as i32),
		Dimension::Year => Period::Year(num as i32),
	};

	Ok((input, duration))
}

// pub fn parse(input: &str) -> IResult<&str, &str> {}
