use crate::solar_data::line::DataLine;
use super::parse_error::ParseError;

pub trait TryParse {
    //A short circuiting function that returns a new DataLine if it succeeds
    fn try_parse(&mut self, line: &Vec<&str>) -> Result<DataLine, ParseError>;
}
