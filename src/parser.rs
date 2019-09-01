use chrono::{format::Parsed, Duration, NaiveTime};
use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{alpha1, digit1, space0, space1},
	combinator::opt,
	error::ErrorKind,
	multi::many1,
	sequence::tuple,
	Err,
};

use crate::types::{Dimension, Period, RecurringInterval};

pub type NomErr<I> = Err<(I, ErrorKind)>;

#[derive(Debug)]
pub enum ParseError<'a, I> {
	Layout(NomErr<I>),
	UnknownDimension(I, &'a str),
	InvalidNumericValue(I, std::num::ParseIntError),
	InvalidTime(chrono::format::ParseError),
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

pub fn parse_period(input: &str) -> ParseResult<&str, RecurringInterval> {
	let (input, _) = tag("every")(input).map_err(ParseError::Layout)?;
	let (input, _) = space1(input).map_err(ParseError::Layout)?;
	let (input, nth): (&str, Option<(&str, &str, &str, &str)>) = opt(tuple((
		digit1,
		alt((space0, tag("-"))),
		alt((tag("st"), tag("nd"), tag("rd"), tag("th"))),
		space1,
	)))(input)
	.map_err(ParseError::Layout)?;

	let nth: Option<u32> = match nth {
		Some((digit, _, _, _)) => Some(
			digit
				.parse()
				.map_err(|e| ParseError::InvalidNumericValue(digit, e))?,
		),
		None => None,
	};

	let (input, num) = digit1(input).map_err(ParseError::Layout)?;
	let (input, _) = space0(input).map_err(ParseError::Layout)?;
	let (input, dim) = parse_dimension(input)?;
	let num: i64 = num
		.parse()
		.map_err(|e| ParseError::InvalidNumericValue(num, e))?;

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

	let result = match nth {
		Some(i) => RecurringInterval::NthPeriod(i, period),
		None => RecurringInterval::Period(period),
	};

	Ok((input, result))
}

#[derive(PartialEq, Debug)]
enum Abbr {
	AM,
	PM,
}

fn parse_am(input: &str) -> Result<(&str, Abbr), NomErr<&str>> {
	let (input, _) =
		alt((tag("AM"), tag("am"), tag("A.M"), tag("a.m.")))(input)?;

	Ok((input, Abbr::AM))
}

fn parse_pm(input: &str) -> Result<(&str, Abbr), NomErr<&str>> {
	let (input, _) =
		alt((tag("PM"), tag("pm"), tag("P.M"), tag("p.m.")))(input)?;

	Ok((input, Abbr::PM))
}

fn parse_abbr(input: &str) -> Result<(&str, Abbr), NomErr<&str>> {
	alt((parse_am, parse_pm))(input)
}

fn parse_time(input: &str) -> ParseResult<&str, NaiveTime> {
	let (input, h) = digit1(input).map_err(ParseError::Layout)?;

	let (input, m) =
		opt(tuple((tag(":"), digit1)))(input).map_err(ParseError::Layout)?;

	let (input, s) =
		opt(tuple((tag(":"), digit1)))(input).map_err(ParseError::Layout)?;

	let (input, ampm) =
		opt(tuple((space0, parse_abbr)))(input).map_err(ParseError::Layout)?;

	let h: i64 = h
		.parse()
		.map_err(|e| ParseError::InvalidNumericValue(h, e))?;

	let m: i64 = match m {
		Some((_, m)) => m
			.parse()
			.map_err(|e| ParseError::InvalidNumericValue(m, e))?,
		None => 0,
	};

	let s: i64 = match s {
		Some((_, s)) => s
			.parse()
			.map_err(|e| ParseError::InvalidNumericValue(s, e))?,
		None => 0,
	};

	let mut parsed = Parsed::new();
	parsed.set_minute(m);
	parsed.set_second(s);

	println!("{:?}", ampm);

	match ampm {
		Some((_, ampm)) => {
			parsed.set_hour12(h);
			parsed.set_ampm(if ampm == Abbr::AM { false } else { true });
		}
		None => {
			parsed.set_hour(h);
		}
	}

	let time = parsed.to_naive_time().map_err(ParseError::InvalidTime)?;

	Ok((input, time))
}

fn parse_at(input: &str) -> ParseResult<&str, Vec<NaiveTime>> {
	let (input, _) = tag("at")(input).map_err(ParseError::Layout)?;
	let (input, _) = space1(input).map_err(ParseError::Layout)?;

	let mut res = vec![];
	let sep = opt(alt((
		tuple((space0, tag(","), space0)),
		tuple((space1, tag("and"), space1)),
	)));

	let (mut input, time) = parse_time(input)?;
	res.push(time);

	loop {
		let (i, s) = sep(input).map_err(ParseError::Layout)?;
		match s {
			Some(_) => {
				let (i, time) = parse_time(i)?;
				res.push(time);
				input = i
			}
			None => break,
		}
	}

	Ok((input, res))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_every_10_days() {
		assert_eq!(
			parse_period("every 10 days").unwrap().1,
			RecurringInterval::Period(Period::Fixed(Duration::days(10)))
		)
	}

	#[test]
	fn parse_every_2nd_2_years() {
		assert_eq!(
			parse_period("every 2nd 2 years").unwrap().1,
			RecurringInterval::NthPeriod(2, Period::Year(2))
		)
	}

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
