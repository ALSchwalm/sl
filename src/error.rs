use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};

pub enum Error {
    Io(std::io::Error),
    InvalidAnimationSpeed(usize),
    EmptyAnimation,
    EmptyFrame,
    InvalidTrainIndex(usize),
    IntParseError(std::num::ParseIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::Io(err) => write!(f, "I/O Error: {}", err),
            Error::InvalidAnimationSpeed(speed) => write!(f, "Invalid animation speed '{}'", speed),
            Error::EmptyAnimation => write!(f, "Animations must have frames, none found"),
            Error::EmptyFrame => write!(f, "Attempt to load animation frame with no contents"),
            Error::InvalidTrainIndex(idx) => write!(f, "Invalid train index '{}'", idx),
            Error::IntParseError(err) => write!(f, "Error while parsing integer: {}", err),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // Call to the 'Display' impl so that main will show a readable
        // output on error (because it uses 'Debug' for some reason)
        write!(f, "{}", self)
    }
}

impl StdError for Error {}

pub type Result<T> = std::result::Result<T, Error>;
