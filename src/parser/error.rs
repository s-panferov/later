use nom::{error::ErrorKind, Err};

pub type NomErr<I> = Err<(I, ErrorKind)>;

#[derive(Debug)]
pub enum ParseError<'a, I> {
	Layout(NomErr<I>),
	UnknownDimension(I, &'a str),
	InvalidNumericValue(I, std::num::ParseIntError),
	InvalidTime(chrono::format::ParseError),
}

pub type ParseResult<'a, I, O> = Result<(I, O), ParseError<'a, I>>;
