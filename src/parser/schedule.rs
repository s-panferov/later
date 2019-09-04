use nom::multi::many1;

use crate::schedule::Schedule;

use super::error::{ParseError, ParseResult};
use super::every::parse_every;

pub fn parse_schedule(input: &str) -> ParseResult<Schedule> {
	let (input, items) = many1(parse_every)(input)?;
	Err(ParseError::Unsupported.into_fail(input))
}

#[cfg(test)]
mod tests {
	use super::*;
	// at 3:00am
	// every sat, sun at 3:00am
	// every friday at 5am
	// every 2nd friday at 10am
	// every sunday from 3:00 to 4:00 until 2010-30-10T20:00
	// once at 20:30
}
