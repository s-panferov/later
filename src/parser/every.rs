use chrono::Duration;
use nom::{
	bytes::complete::tag,
	character::complete::{digit1, space0, space1},
};

use crate::every::Every;
use crate::period::Period;
use crate::types::{Dimension};

use super::dimension::parse_dimension;
use super::error::{ParseError, ParseResult};
use super::ordinal::parse_ordinal;
use super::utils::parse_chain;
use super::weekday::parse_weekday;

pub fn parse_numeric_period(input: &str) -> ParseResult<Period> {
	let (input, num) = digit1(input)?;
	let (input, _) = space0(input)?;
	let (input, dim) = parse_dimension(input)?;
	let num: i64 = num
		.parse()
		.map_err(|e| ParseError::InvalidNumericValue(e).into_fail(input))?;

	let period = match dim {
		Dimension::Second => Period::Fixed(Duration::seconds(num)),
		Dimension::Minute => Period::Fixed(Duration::minutes(num)),
		Dimension::Hour => Period::Fixed(Duration::hours(num)),
		Dimension::Day => Period::Fixed(Duration::days(num)),
		Dimension::Week => Period::Fixed(Duration::weeks(num)),
		Dimension::Month => Period::Month(num as i32),
		Dimension::Quarter => Period::Quarter(num as i32),
		Dimension::Year => Period::Year(num as i32),
	};

	Ok((input, period))
}

pub fn parse_period_inner(input: &str) -> ParseResult<Period> {
	let res = parse_numeric_period(input);
	if res.is_ok() {
		return res;
	}

	let res = parse_weekday(input);
	if res.is_ok() {
		return res;
	}

	Err(ParseError::Unsupported.into_fail(input))
}

pub fn parse_period(input: &str) -> ParseResult<Period> {
	let (input, ord) = parse_ordinal(input)?;
	let (input, inner) = parse_period_inner(input)?;

	match ord {
		None => Ok((input, inner)),
		Some(ord) => Ok((input, Period::Ordinal(ord, Box::new(inner)))),
	}
}

pub fn parse_every(input: &str) -> ParseResult<Every> {
	let (input, _) = tag("every")(input)?;
	let (input, _) = space1(input)?;
	let (input, res) = parse_chain(input, |input| parse_period(input))?;
	Ok((input, Every::new(res)))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_every_10_days() {
		assert_eq!(
			parse_every("every 10 days").unwrap().1,
			Every::new(vec![Period::Fixed(Duration::days(10))])
		)
	}

	#[test]
	fn parse_every_2nd_2_years() {
		assert_eq!(
			parse_every("every 2nd 2 years").unwrap().1,
			Every::new(vec![Period::Ordinal(2, Box::new(Period::Year(2)))])
		)
	}
}
