use serde::{Serialize, Deserialize};

use super::graph_axis::AxisData;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphStateRequest {
    pub x_axis: AxisData,
    pub y_axis: (Vec<AxisData>, Vec<AxisData>),
    pub start_time: i64,
    pub end_time: i64,
    pub resolution: Resolution,
}

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
            Resolution::FifteenMinute => 1500,
            Resolution::OneHour => 6000,
            Resolution::OneDay => 144000,
        }
    }
}