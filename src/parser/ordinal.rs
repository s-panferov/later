
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{digit1, space0, space1},
	combinator::opt,
	sequence::tuple,
};

use super::error::{ParseError, ParseResult};

pub fn parse_ordinal(input: &str) -> ParseResult<Option<u32>> {
	let (input, nth): (&str, Option<(&str, &str, &str, &str)>) = opt(tuple((
		digit1,
		alt((space0, tag("-"))),
		alt((tag("st"), tag("nd"), tag("rd"), tag("th"))),
		space1,
	)))(input)?;

	let nth = match nth {
		Some((digit, _, _, _)) => Some(digit.parse().map_err(|e| {
			ParseError::InvalidNumericValue(e).into_fail(input)
		})?),
		None => None,
	};

	Ok((input, nth))
}
