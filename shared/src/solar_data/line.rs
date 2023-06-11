use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use time::{Date, Time};
use crate::{solar_data::value::DataValue};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataLine {
    unix_time: i64,
    line: Vec<DataValue>,
}

impl DataLine {
    pub fn new(date: Date, time: Time) -> DataLine {
        DataLine {
            unix_time: time::PrimitiveDateTime::new(date, time).assume_utc().unix_timestamp(),
            line: Vec::new(),
        }
    }

    pub fn add_data(&mut self, data: DataValue) {
        match self.line.binary_search(&data) {
            Ok(_) => {
                //Same enum type already exists
            },
            Err(pos) => {
                self.line.insert(pos, data);
            }
        }
    }

    // pub fn len(&self) -> usize {
    //     self.line.len()
    // }

    // pub fn get_date(&self) -> &Date {
    //     &self.date
    // }

    // pub fn get_time(&self) -> &Time {
    //     &self.time
    // }
}

impl std::iter::IntoIterator for DataLine {
    type Item = DataValue;

    type IntoIter = std::vec::IntoIter<DataValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.line.into_iter()
    }
}

impl PartialEq for DataLine {
    fn eq(&self, other: &Self) -> bool {
        self.unix_time == other.unix_time
    }
}

impl PartialOrd for DataLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Eq for DataLine {}

impl Ord for DataLine {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unix_time.cmp(&other.unix_time)
    }
}


mod private_state{
    //Type state pattern
    pub struct NoDateSet;
    pub struct DateIsSet;
    pub struct NoTimeSet;
    pub struct TimeIsSet;
}

pub struct DataLineBuilder<IsDateSet = private_state::NoDateSet, IsTimeSet = private_state::NoTimeSet>{
    date: Date,
    time: Time,
    line: Vec<DataValue>,
    is_date_set: std::marker::PhantomData<IsDateSet>,
    is_time_set: std::marker::PhantomData<IsTimeSet>,
}

impl<IsDateSet, IsTimeSet> DataLineBuilder<IsDateSet, IsTimeSet> {
    pub fn add_data(&mut self, data: DataValue) {
        self.line.push(data);
    }
}

impl DataLineBuilder<private_state::NoDateSet, private_state::NoTimeSet> {
    pub fn set_date(self, date: Date) -> DataLineBuilder<private_state::DateIsSet, private_state::NoTimeSet> {
        DataLineBuilder {
            date,
            time: self.time,
            line: self.line,
            is_date_set: std::marker::PhantomData,
            is_time_set: std::marker::PhantomData,
        }
    }

    pub fn set_time(self, time: Time) -> DataLineBuilder<private_state::NoDateSet, private_state::TimeIsSet>{
        DataLineBuilder {
            date: self.date,
            time: time,
            line: self.line,
            is_date_set: std::marker::PhantomData,
            is_time_set: std::marker::PhantomData,
        }
    }
}

impl DataLineBuilder<private_state::DateIsSet, private_state::NoTimeSet> {
    pub fn set_time(self, time: Time) -> DataLineBuilder<private_state::DateIsSet, private_state::TimeIsSet>{
        DataLineBuilder {
            date: self.date,
            time: time,
            line: self.line,
            is_date_set: std::marker::PhantomData,
            is_time_set: std::marker::PhantomData,
        }
    }
}

impl DataLineBuilder<private_state::NoDateSet, private_state::TimeIsSet> {
    pub fn set_date(self, date: Date) -> DataLineBuilder<private_state::DateIsSet, private_state::TimeIsSet>{
        DataLineBuilder {
            date: date,
            time: self.time,
            line: self.line,
            is_date_set: std::marker::PhantomData,
            is_time_set: std::marker::PhantomData,
        }
    }
}

impl DataLineBuilder<private_state::DateIsSet, private_state::TimeIsSet>{
    pub fn build(self) -> DataLine{
        let mut dl = DataLine::new(self.date, self.time);
        for data in self.line.into_iter() {
            dl.add_data(data);
        }
        dl
    }
}

impl Default for DataLineBuilder {
    fn default() -> Self {
        Self { date: Date::MIN, time: Time::MIDNIGHT, line: Default::default(), is_date_set: Default::default(), is_time_set: Default::default() }
    }
}