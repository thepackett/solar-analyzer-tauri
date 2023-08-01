use time::{parsing::Parsed, macros::format_description, Date, Time};

use crate::solar_data::{storage::DataStorage, line::{DataLine, DataLineBuilder}, value::DataValue};

use super::{version::Version, traits::TryParse, parse_error::ParseError, utils::{check_version, convert_year}};

#[derive(Default, Debug)]
pub struct StoredData {
    pub version: Version,
    pub data: DataStorage,
}

impl TryParse for StoredData {
    fn try_parse(&mut self, line: &Vec<&str>) -> Result<DataLine, ParseError> {
        let data_line_builder = DataLineBuilder::default();
        if let Ok(version) = check_version(line) {
            self.version = version;
        }
        //Currently no incompatible versions, so no need to operate based on version

        let mut current = line.iter();
        //Test the first entry
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
        if date.midnight().assume_utc().unix_timestamp() < 1262325600 {
            return Err(ParseError::InvalidData)
        }
        let data_line_builder = data_line_builder.set_date(date);
        //Test the second entry...
        let time = Time::parse(
            current.next().ok_or(ParseError::InsufficientData)?,
            format_description!("[hour]:[minute]"),
        )?;
        let mut data_line_builder = data_line_builder.set_time(time);
        //Test the third entry...
        let battery_voltage = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::BatteryVoltage(battery_voltage));
        //Test the fouth entry...
        let state_of_charge_percent = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::StateOfChargePercent(state_of_charge_percent));
        //
        let solar_amps = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::SolarWatts(solar_amps * battery_voltage));
        //
        let load_amps = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::LoadWatts(load_amps * battery_voltage));
        //We can now validate the data before continuing:
        //For stored data after the date and time, if the battery voltage, state of charge, solar amps, and load amps are 5, 0, 0, 0
        //Then the data is invalid and should be discarded.
        if battery_voltage == 5.0 && state_of_charge_percent == 0.0 && solar_amps == 0.0 && load_amps == 0.0 {
          return Err(ParseError::InvalidData)
        }
        
        
        //Variable number of cell voltage statistics iff there is a delimeter
        let delimeter = current.next();
        if let None = delimeter {
            return Ok(data_line_builder.build());
        }
        let counter = current.clone().count();
        //There should be counter-4 entries related to cell voltage, two for each cell, one for the high voltage, one for the low voltage
        for n in 0..(counter - 4) / 2 {
            let cell_voltage_low = current
                .next()
                .ok_or(ParseError::InsufficientData)?
                .parse::<f32>()?;
            data_line_builder.add_data(DataValue::StatisticsCellVoltageLow {
                cell: n as u16,
                voltage: cell_voltage_low,
            });
            let cell_voltage_high = current
                .next()
                .ok_or(ParseError::InsufficientData)?
                .parse::<f32>()?;
            data_line_builder.add_data(DataValue::StatisticsCellVoltageHigh {
                cell: n as u16,
                voltage: cell_voltage_high,
            });
        }
        //
        let statistics_solar_watts = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::StatisticsSolarWatts(statistics_solar_watts));
        //
        let statistics_load_watts = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::StatisticsLoadWatts(statistics_load_watts));
        //
        let statistics_state_of_charge_low = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::StatisticsStateOfChargePercentLow(
            statistics_state_of_charge_low,
        ));
        //
        let statistics_state_of_charge_high = current
            .next()
            .ok_or(ParseError::InsufficientData)?
            .parse::<f32>()?;
        data_line_builder.add_data(DataValue::StatisticsStateOfChargePercentHigh(
            statistics_state_of_charge_high,
        ));
        //
        
        Ok(data_line_builder.build())
    }
}