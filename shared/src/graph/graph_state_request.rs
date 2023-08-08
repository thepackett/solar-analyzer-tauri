use serde::{Serialize, Deserialize};

use super::graph_axis::AxisData;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphStateRequest {
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