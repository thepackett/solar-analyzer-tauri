use serde::{Serialize, Deserialize};
use time::{PrimitiveDateTime, macros::{date, time}};

use super::graph_axis::{AxisData, AxisDataType, AxisDataOptions};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphStateRequest {
    pub graph_id: String,
    pub x_axis: Vec<AxisData>,
    pub y_axis: (Vec<AxisData>, Vec<AxisData>),
    pub start_time: i64,
    pub end_time: i64,
    pub resolution: Resolution,
}

//Note that Resolution must uphold the invariant that any of its members MUST be evenly divisible into 24 hours.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Resolution {
    OneMinute,
    FiveMinute,
    FifteenMinute,
    OneHour,
    OneDay,
}

impl Resolution {
    pub fn get_timestamp_offset(&self) -> i64 {
        match self {
            Resolution::OneMinute => 60,
            Resolution::FiveMinute => 300,
            Resolution::FifteenMinute => 900,
            Resolution::OneHour => 3600,
            Resolution::OneDay => 86400,
        }
    }
}

impl GraphStateRequest {
    pub fn default_with_name(name: String) -> Self {
        Self { 
            graph_id: name,
            x_axis: vec![AxisData { 
                data_type: AxisDataType::Time, 
                required_data_option: AxisDataOptions::Sample,
                additional_data_options: Vec::new()
                }], 
            y_axis: (vec![ 
                AxisData { 
                    data_type: AxisDataType::StateOfChargePercent, 
                    required_data_option: AxisDataOptions::Average,
                    additional_data_options: vec![AxisDataOptions::Minimum, AxisDataOptions::Maximum, AxisDataOptions::Sample], 
                }],
                Vec::new()), 
            start_time: PrimitiveDateTime::new(date!(2022-01-01), time!(0:00)).assume_utc().unix_timestamp(), 
            end_time: PrimitiveDateTime::new(date!(2023-06-01), time!(0:00)).assume_utc().unix_timestamp(), 
            resolution: Resolution::OneDay,
        }
    }
}