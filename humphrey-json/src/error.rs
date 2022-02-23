//! Provides error handling functionality.

use std::error::Error;
use std::fmt::Display;

/// Represents an error during parsing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// An unknown error was encountered. This is likely a bug with Humphrey JSON.
    UnknownError,
    /// An invalid token was encountered.
    InvalidToken,
    /// The end of the file was reached but more data was expected.
    UnexpectedEOF,
    /// An escape sequence in a string was invalid.
    InvalidEscapeSequence,
    /// A trailing comma was encountered.
    TrailingComma,
}

/// Encapsulates a parse error and its location.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TracebackError {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) kind: ParseError,
}

impl Display for TracebackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error at {}:{}: {:?}", self.line, self.column, self.kind)
    }
}

impl Error for TracebackError {}
