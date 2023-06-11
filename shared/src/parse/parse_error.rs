use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub enum ParseError {
    IntParseError,
    FloatParseError,
    DateTimeParseError,
    InsufficientData,
    InvalidDelimeter,
    InvalidData,
    ImproperFormat,
    NoVersion,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::IntParseError => f.write_str("Coult not parse int."),
            ParseError::FloatParseError => f.write_str("Could not parse float"),
            ParseError::DateTimeParseError => f.write_str("Could not parse date/time"),
            ParseError::InsufficientData => f.write_str("Insufficient data, expected more entries"),
            ParseError::InvalidDelimeter => f.write_str("Invalid delimeter, expected empty entry"),
            ParseError::InvalidData => f.write_str("Line contained invalid data"),
            ParseError::ImproperFormat => {
                f.write_str("Could not parse data out of improperly formatted entry")
            }
            ParseError::NoVersion => f.write_str("Version string not found"),
        }
    }
}

impl Error for ParseError {}

impl From<std::num::ParseFloatError> for ParseError {
    fn from(_value: std::num::ParseFloatError) -> Self {
        ParseError::FloatParseError
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(_value: std::num::ParseIntError) -> Self {
        ParseError::IntParseError
    }
}

impl From<time::error::Parse> for ParseError {
    fn from(_value: time::error::Parse) -> Self {
        ParseError::DateTimeParseError
    }
}

impl From<time::error::ParseFromDescription> for ParseError {
    fn from(_value: time::error::ParseFromDescription) -> Self {
        ParseError::DateTimeParseError
    }
}

impl From<time::error::TryFromParsed> for ParseError {
    fn from(_value: time::error::TryFromParsed) -> Self {
        ParseError::DateTimeParseError
    }
}