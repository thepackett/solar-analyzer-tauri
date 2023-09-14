use crate::solar_data::cell::AvailableCells;
use crate::solar_data::controllers::AvailableControllers;

use super::parse_error::ParseError;
use super::version::Version;


pub fn check_version(line: &Vec<&str>) -> Result<Version, ParseError> {
    let last_entry = line.iter().last().ok_or(ParseError::InsufficientData)?;
    if last_entry.contains("Ver") {
        let mut version_number = last_entry
            .split(' ')
            .last()
            .ok_or(ParseError::ImproperFormat)?
            .split('.');
        let major = version_number
            .next()
            .ok_or(ParseError::ImproperFormat)?
            .parse::<u32>()?;
        let minor = version_number
            .next()
            .ok_or(ParseError::ImproperFormat)?
            .parse::<u32>()?;
        Ok(Version {
            major: major,
            minor: minor,
        })
    } else {
        Err(ParseError::NoVersion)
    }
}

//Attempts to convert a last_two year representation to a full representation. Correct for all dates within the past 100 years from the current year.
pub fn convert_year(last_two_year: u8) -> i32 {
    let current_time = time::OffsetDateTime::now_utc().saturating_add(time::Duration::days(1));
    let last_two = Into::<i32>::into(last_two_year);
    match (current_time.year() % 100).cmp(&last_two) {
        //If the current last two digits of the year are less than the recorded last two digits of the data's year, then assumue it must be from the previous century.
        std::cmp::Ordering::Less => {
            current_time.year() - (current_time.year() % 100) - 100 + last_two
        }
        //If the current last two digits of the year are greater or equal than the recorded last two digits of the data's year, then assume it must be from this century.
        std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
            current_time.year() - (current_time.year() % 100) + last_two
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ParseCompleteReturnValue {
    pub name: String,
    pub cell_ids: AvailableCells,
    pub controller_ids: AvailableControllers,
}