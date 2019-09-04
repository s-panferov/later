use chrono::{format::Parsed, NaiveTime};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{digit1, space0, space1},
	combinator::opt,
	sequence::tuple,
};

use crate::at::At;

use super::error::{ParseError, ParseResult};
use super::utils::parse_chain;

#[derive(PartialEq, Debug)]
enum Abbr {
	AM,
	PM,
}

fn parse_at(input: &str) -> ParseResult<Vec<At>> {
	let (input, _) = tag("at")(input)?;
	let (input, _) = space1(input)?;
	parse_chain(input, |input| {
		let (input, time) = parse_time(input)?;
		Ok((input, At::Time(time)))
	})
}

pub fn parse_time(input: &str) -> ParseResult<NaiveTime> {
	let (input, h) = digit1(input)?;
	let (input, m) = opt(tuple((tag(":"), digit1)))(input)?;
	let (input, s) = opt(tuple((tag(":"), digit1)))(input)?;
	let (input, ampm) = opt(tuple((space0, parse_abbr)))(input)?;

	let h: i64 = h
		.parse()
		.map_err(|e| ParseError::InvalidNumericValue(e).into_fail(input))?;

	let m: i64 = match m {
		Some((_, m)) => m
			.parse()
			.map_err(|e| ParseError::InvalidNumericValue(e).into_fail(input))?,
		None => 0,
	};

	let s: i64 = match s {
		Some((_, s)) => s
			.parse()
			.map_err(|e| ParseError::InvalidNumericValue(e).into_fail(input))?,
		None => 0,
	};

	let mut parsed = Parsed::new();
	parsed.set_minute(m);
	parsed.set_second(s);

	match ampm {
		Some((_, ampm)) => {
			parsed.set_hour12(h);
			parsed.set_ampm(if ampm == Abbr::AM { false } else { true });
		}
		None => {
			parsed.set_hour(h);
		}
	}

	let time = parsed
		.to_naive_time()
		.map_err(|e| ParseError::InvalidTime(e).into_fail(input))?;

	Ok((input, time))
}

fn parse_am(input: &str) -> ParseResult<Abbr> {
	let (input, _) =
		alt((tag("AM"), tag("am"), tag("A.M"), tag("a.m.")))(input)?;

	Ok((input, Abbr::AM))
}

fn parse_pm(input: &str) -> ParseResult<Abbr> {
	let (input, _) =
		alt((tag("PM"), tag("pm"), tag("P.M"), tag("p.m.")))(input)?;

	Ok((input, Abbr::PM))
}

fn parse_abbr(input: &str) -> ParseResult<Abbr> {
	alt((parse_am, parse_pm))(input)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn parse_at_10_00() {
		assert_eq!(
			parse_at("at 10:00").unwrap().1,
			vec![NaiveTime::from_hms(10, 0, 0)]
		)
	}

	#[test]
	fn parse_at_7_pm() {
		assert_eq!(
			parse_at("at 7 pm").unwrap().1,
			vec![NaiveTime::from_hms(19, 0, 0)]
		)
	}

	#[test]
	fn parse_at_7_830pm_2030() {
		assert_eq!(
			parse_at("at 7pm, 8:30pm and 20:30").unwrap().1,
			vec![
				NaiveTime::from_hms(19, 0, 0),
				NaiveTime::from_hms(20, 30, 0),
				NaiveTime::from_hms(20, 30, 0)
			]
		)
	}
}
