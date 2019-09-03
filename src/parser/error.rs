pub use nom::error::{ErrorKind, ParseError as ErrorExt};
pub use nom::Err;

#[derive(Debug)]
pub enum ParseError {
	Layout(ErrorKind),
	UnknownDimension,
	Unsupported,
	InvalidNumericValue(std::num::ParseIntError),
	InvalidTime(chrono::format::ParseError),
}

impl ParseError {
	pub fn into_err(self, input: &str) -> Err<ErrorContext> {
		Err::Error(ErrorContext { input, error: self })
	}

	pub fn into_fail(self, input: &str) -> Err<ErrorContext> {
		Err::Failure(ErrorContext { input, error: self })
	}
}

#[derive(Debug)]
pub struct ErrorContext<'a> {
	input: &'a str,
	error: ParseError,
}

impl<'a> ErrorExt<&'a str> for ErrorContext<'a> {
	fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
		ErrorContext {
			input,
			error: ParseError::Layout(kind),
		}
	}

	fn append(_: &str, _: ErrorKind, other: Self) -> Self {
		other
	}
}

pub type ParseResult<'a, O> = Result<(&'a str, O), Err<ErrorContext<'a>>>;
pub type SimpleResult<'a, O> = Result<(&'a str, O), Err<(&'a str, ErrorKind)>>;
