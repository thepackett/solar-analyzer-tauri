use shared::graph::graph_axis::{AxisTimeRequest, AxisControlsRequest};


#[derive(PartialEq)]
pub struct SharedGraphData {
    data: SharableGraphData,
}

#[derive(PartialEq)]
pub enum SharableGraphData {
    TimeData(AxisTimeRequest),
    XAxisData(AxisControlsRequest),
    YAxisData(AxisControlsRequest),
    SecYAxisData(AxisControlsRequest),
}