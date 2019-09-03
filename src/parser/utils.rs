use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{space0, space1},
	combinator::opt,
	sequence::tuple,
};

use super::error::ParseResult;

pub fn parse_chain<T, F>(input: &str, parser: F) -> ParseResult<Vec<T>>
where
	F: Fn(&str) -> ParseResult<T>,
{
	let mut res = vec![];
	let sep = opt(alt((
		tuple((space0, tag(","), space0)),
		tuple((space1, tag("and"), space1)),
	)));

	let (mut input, value) = parser(input)?;
	res.push(value);

	loop {
		let (i, s) = sep(input)?;
		match s {
			Some(_) => {
				let (i, value) = parser(i)?;
				res.push(value);
				input = i
			}
			None => break,
		}
	}

	Ok((input, res))
}
