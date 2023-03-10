use crate::giveup::GiveupFormatError;

/// Add an example message to an object.
pub trait Example<'a> {
	/// Consumes and returns `self` combined with the
	/// given `example` message.
	fn example(self, example: &'a str) -> Self;
}

/// Implementation of `Example` on any `Result`s returned by
/// [`hint`](crate::Giveup::hint)
impl<'a, T, E> Example<'a> for Result<T, HintedError<'a, E>> {
	/// Set the `example` field in `self` to the
	/// given string if `self` is an error.
	fn example(mut self, example: &'a str) -> Self {
		if let Err(ref mut e) = self {
			e.hint.example = Some(example);
		}
		self
	}
}


/// Combination of an error with user information.
#[derive(Debug)]
pub struct HintedError<'a, E> {
	/// The wrapped error.
	e: E,
	/// Additional user information about the error.
	hint: Hint<'a>,
}

impl<'a, E> HintedError<'a, E>
where
	E: GiveupFormatError,
{
	/// Create a new error wrapper which combines the given error with
	/// a hint on how to resolve the error.
	pub fn with_hint(e: E, hint: &'a str) -> Self {
		Self {
			e,
			hint: Hint{ hint, example: None },
		}
	}
}

impl<'a, E> GiveupFormatError for HintedError<'a, E>
where
	E: GiveupFormatError,
{
	fn format_err_msg(&self) -> String {
		// The hinted error message is made up of the raw error
		// message followed by the hint.
		format!("{}{}\n", self.e.format_err_msg(), self.hint)
	}
}

/// Information on events which is meant
/// for users to act on the event correctly.
#[derive(Debug)]
struct Hint<'a> {
	hint: &'a str,
	example: Option<&'a str>,
}

impl<'a> std::fmt::Display for Hint<'a> {
	/// Print a `Hint` instance.
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.example {
			Some(example) => write!(f, "{}: `{}`", self.hint, example),
			None => write!(f, "{}", self.hint),
		}
	}
}
