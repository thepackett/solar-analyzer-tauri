use time::{parsing::Parsed, macros::format_description, Time, Date};

use crate::solar_data::{storage::DataStorage, value::DataValue, line::{DataLineBuilder, DataLine}};

use super::{version::Version, parse_error::ParseError, utils::{check_version, convert_year}, traits::TryParse};

#[derive(Default, Debug)]
pub struct LiveData {
    pub version: Version,
    pub data: DataStorage,
}

impl TryParse for LiveData {
    fn try_parse(&mut self, line: &Vec<&str>) -> Result<DataLine, ParseError> {
        let mut data_line_builder = DataLineBuilder::default();
        if let Ok(version) = check_version(line) {
            self.version = version;
        }

        //Currently no incompatible versions, so no need to operate based on version
        let mut current = line.iter();
        //Test the first entry
        let alarm_code = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<i32>()?;
        data_line_builder.add_data(DataValue::AlarmCode(alarm_code));
        //Test the second entry...
        //We need to use the time::Parsed struct directly since the last_two representation of the year is ambiguous
        let mut parsed = Parsed::new();
        parsed.parse_items(
            current
                .next()
                .ok_or(ParseError::InsufficientData)?
                .as_bytes(),
            &format_description!("[month padding:none]/[day]/[year repr:last_two]"),
        )?;
        parsed.set_year(convert_year(
            parsed
                .year_last_two()
                .ok_or(ParseError::DateTimeParseError)?,
        ));
        let date = Date::try_from(parsed)?;
        let data_line_builder = data_line_builder.set_date(date);
        //Test the third entry...
        let time = Time::parse(current.next().ok_or(ParseError::InsufficientData)?, format_description!("[hour repr:24]:[minute]:[second]"))?;
        let mut data_line_builder = data_line_builder.set_time(time);
        //Test the fourth entry...
        let battery_voltage = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::BatteryVoltage(battery_voltage));
        //Test the fifth entry...
        let battery_amps = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::BatteryAmps(battery_amps));
        //Test the sixth entry
        let solar_watts = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::SolarWatts(solar_watts));
        //...
        let load_watts = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::LoadWatts(load_watts));
        //
        let state_of_charge_percent = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::StateOfChargePercent(state_of_charge_percent));
        //
        let amp_hours = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::AmpHoursSinceMidnight(amp_hours));
        //Assert that the next value is empty
        if !current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .is_empty()
        {
            return Err(ParseError::InvalidDelimeter);
        }
        //Begin processing the cell voltage loop
        for n in 0.. {
            let entry = current.next().ok_or(ParseError::InsufficientData)?;
            if entry.is_empty() {
                break;
            }
            let cell_voltage = entry.parse::<f32>()?;
            data_line_builder.add_data(DataValue::CellVoltage {
                cell: n,
                voltage: cell_voltage,
            });
        }
        //Begin processing the solar controller data loop
        loop {
            let entry = current.next().ok_or(ParseError::InsufficientData)?;
            if entry.is_empty() {
                break;
            }
            let address = entry.parse::<u16>()?;
            let panel_voltage = current
                .next()
                .ok_or(ParseError::InsufficientData)?
                .parse::<f32>()?;
            data_line_builder.add_data(DataValue::ControllerPanelVoltage {
                controller: address,
                voltage: panel_voltage,
            });
            let battery_voltage = current
                .next()
                .ok_or(ParseError::InsufficientData)?
                .parse::<f32>()?;
            data_line_builder.add_data(DataValue::ControllerBatteryVoltage {
                controller: address,
                voltage: battery_voltage,
            });
            let amps = current
                .next()
                .ok_or(ParseError::InsufficientData)?
                .parse::<f32>()?;
            data_line_builder.add_data(DataValue::ControllerAmps {
                controller: address,
                amps: amps,
            });
            let temp = current
                .next()
                .ok_or(ParseError::InsufficientData)?
                .parse::<f32>()?;
            data_line_builder.add_data(DataValue::ControllerTemperatureF {
                controller: address,
                temperature: temp,
            });
            if !current
                .next()
                .ok_or(ParseError::InsufficientData)?
                .is_empty()
            {
                return Err(ParseError::InvalidDelimeter);
            }
        }

        Ok(data_line_builder.build())
    }
}
