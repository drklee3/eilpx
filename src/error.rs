use log::SetLoggerError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::io::Error as IoError;
use image::ImageError;

/// Common result type used throughout the program.
pub type Result<T> = StdResult<T, Error>;

/// Common error type used throughout the program, to be used as a holder for
/// errors from various other libraries.
#[derive(Debug)]
pub enum Error {
  /// A `std::io` module error.
  Io(IoError),
  /// A `log` crate error by `set_logger`.
  SetLogger(SetLoggerError),
  /// A `image` crate error.
  Image(ImageError),
}

impl From<IoError> for Error {
  fn from(err: IoError) -> Error {
    Error::Io(err)
  }
}

impl From<SetLoggerError> for Error {
  fn from(err: SetLoggerError) -> Error {
    Error::SetLogger(err)
  }
}

impl From<ImageError> for Error {
  fn from(err: ImageError) -> Error {
    Error::Image(err)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match *self {
      Error::Io(ref inner) => inner.fmt(f),
      Error::SetLogger(ref inner) => inner.fmt(f),
      Error::Image(ref inner) => inner.fmt(f),
    }
  }
}

impl StdError for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Io(ref inner) => inner.description(),
      Error::SetLogger(ref inner) => inner.description(),
      Error::Image(ref inner) => inner.description(),
    }
  }
}