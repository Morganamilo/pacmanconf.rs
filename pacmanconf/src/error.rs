use std::error;
use std::fmt;
use std::io;
use std::str;

/// Error Line holds a line of text and the line number the line is from.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ErrorLine {
    /// The line number that the  error occurred at
    pub number: usize,
    /// The full line containing the error
    pub line: String,
}

impl ErrorLine {
    /// Create a new Error from a given ErrorKind and ErrorLine.
    ///
    /// If the line is none then Errors can be created using the the From/Into traits.
    pub fn new<S: Into<String>>(number: usize, line: S) -> ErrorLine {
        ErrorLine {
            number,
            line: line.into(),
        }
    }
}

/// A list of possible errors that may occur when parsing a pacman.conf
#[derive(Debug)]
pub enum ErrorKind {
    /// A directive was specified outside of a section.
    /// The variant holds the key name.
    NoSection(String),
    /// A directive that requires a value was specified without a value.
    /// The variant holds the section and key.
    MissingValue(String, String),
    /// A directive was given with an invalid value.
    /// The variant holds the section, key and value.
    InvalidValue(String, String, String),
    /// A directive was given with an unknown key.
    /// The variant holds the section and key.
    UnknownKey(String, String),
    /// An error occurred while executing pacman-conf.
    /// This variant hold the stdout of pacman-coonf
    Runtime(String),
    /// A utf8 error occurred.
    Utf8(str::Utf8Error),
    /// An IO error occurred.
    Io(io::Error),
}

impl From<io::Error> for ErrorKind {
    fn from(e: io::Error) -> ErrorKind {
        ErrorKind::Io(e)
    }
}

impl From<str::Utf8Error> for ErrorKind {
    fn from(e: str::Utf8Error) -> ErrorKind {
        ErrorKind::Utf8(e)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::NoSection(k) => write!(fmt, "Key '{}' must appear in a section", k),
            ErrorKind::MissingValue(s, k) => {
                write!(fmt, "Key '{}' in section '{}' requires a value", k, s)
            }
            ErrorKind::InvalidValue(s, k, v) => {
                write!(fmt, "Invalid value for '{}' in section '{}': '{}'", k, s, v)
            }
            ErrorKind::Runtime(s) => write!(fmt, "Failed to execute pacman-conf: {}", s),
            ErrorKind::UnknownKey(s, k) => write!(fmt, "Unknown key: '{}' in section '{}'", s, k),
            ErrorKind::Io(err) => err.fmt(fmt),
            ErrorKind::Utf8(err) => err.fmt(fmt),
        }
    }
}

/// The error type for pacman.conf parsing.
#[derive(Debug)]
pub struct Error {
    /// The kind of Error that occurred
    pub kind: ErrorKind,
    /// The line where the error occurred
    pub line: Option<ErrorLine>,
}

impl error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind, line: None }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        ErrorKind::Io(err).into()
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Error {
        ErrorKind::Utf8(err).into()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.line {
            Some(ref line) => write!(fmt, "Line {}: {}: {}", line.number, self.kind, line.line),
            None => write!(fmt, "{}", self.kind),
        }
    }
}
