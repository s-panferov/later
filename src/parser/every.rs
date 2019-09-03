use chrono::Duration;
use nom::{
	bytes::complete::tag,
	character::complete::{digit1, space0, space1},
};

use crate::types::{Dimension, Period, RecurringInterval};

use super::dimension::parse_dimension;
use super::error::{ParseError, ParseResult};
use super::ordinal::parse_ordinal;
use super::weekday::parse_weekday;

pub fn parse_period(input: &str) -> ParseResult<Period> {
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

pub fn parse_every(input: &str) -> ParseResult<RecurringInterval> {
	let (input, _) = tag("every")(input)?;
	let (input, _) = space1(input)?;
	let (input, nth) = parse_ordinal(input)?;

	let res = parse_period(input);
	match res {
		Ok((input, period)) => {
			let result = match nth {
				Some(i) => RecurringInterval::NthPeriod(i, period),
				None => RecurringInterval::Period(period),
			};

			return Ok((input, result));
		}
		Err(e) => e,
	};

	let res = parse_weekday(input);
	if res.is_ok() {
		return res;
	}

	Err(ParseError::Unsupported.into_fail(input))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_every_10_days() {
		assert_eq!(
			parse_every("every 10 days").unwrap().1,
			RecurringInterval::Period(Period::Fixed(Duration::days(10)))
		)
	}

	#[test]
	fn parse_every_2nd_2_years() {
		assert_eq!(
			parse_every("every 2nd 2 years").unwrap().1,
			RecurringInterval::NthPeriod(2, Period::Year(2))
		)
	}
}
