use serde::{Serialize, Deserialize};
use time::{PrimitiveDateTime, macros::{date, time}};

use super::graph_axis::{AxisDataType, AxisDataOption, AxisControlsRequest, AxisTimeRequest};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphStateRequest {
    pub graph_id: String,
    pub x_axis: AxisControlsRequest,
    pub y_axis: (AxisControlsRequest, AxisControlsRequest),
    pub time_frame: AxisTimeRequest,
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
            x_axis: AxisControlsRequest {
                requests: vec![(AxisDataType::Time, AxisDataOption::Average)],
            },
            y_axis: (
                AxisControlsRequest {
                    requests: vec![(AxisDataType::BatteryVoltage, AxisDataOption::Average)],
                },
                AxisControlsRequest::default()), 
            time_frame: AxisTimeRequest { 
                start: PrimitiveDateTime::new(date!(2022-01-01), time!(0:00)).assume_utc().unix_timestamp(),
                end: PrimitiveDateTime::new(date!(2023-06-01), time!(0:00)).assume_utc().unix_timestamp(), 
                manual_resolution: Some(Resolution::OneDay),
            }
        }
    }
}