use plotters::{series::LineSeries, style::{BLACK, ShapeStyle}};
use serde::{Deserialize, Serialize};



#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AxisData {
    Time,
    BatteryVoltage,
    BatteryAmps,
    SolarWatts,
    LoadWatts,
    StateOfChargePercent,
    //AmpHoursSinceMidnight, //Not sure how useful this is to graph.
    CellVoltage(u16),
    ControllerPanelVoltage(u16),
    ControllerAmps(u16),
    ControllerTemperatureF(u16),
    Custom(String),
}

impl AxisData {
    pub fn get_name(&self) -> String {
        match self {
            AxisData::Time => "Time".to_owned(),
            AxisData::BatteryVoltage => "Battery Voltage".to_owned(),
            AxisData::BatteryAmps => "Battery Amps".to_owned(),
            AxisData::SolarWatts => "Solar Watts".to_owned(),
            AxisData::LoadWatts => "Load Watts".to_owned(),
            AxisData::StateOfChargePercent => "State Of Charge Percent".to_owned(),
            AxisData::CellVoltage(cell) => format!("Cell #{} Voltage", {cell}),
            AxisData::ControllerPanelVoltage(controller) => format!("Controller #{} Voltage", {controller}),
            AxisData::ControllerAmps(controller) => format!("Controller #{} Amps", {controller}),
            AxisData::ControllerTemperatureF(controller) => format!("Controller #{} TemperatureF", {controller}),
            AxisData::Custom(s) => todo!(),
        }
    }
}


#[derive(PartialEq, Serialize, Deserialize, Default)]
pub struct LineSeriesHolder {
    pub series: Vec<LineSeriesData>,
    pub secondary_series: Vec<LineSeriesData>,
}

#[derive(PartialEq, Serialize, Deserialize, Default)]
pub struct LineSeriesData {
    pub name: String,
    pub data_points: Vec<(f64, f64)>,
    pub series_type: LineSeriesType,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum LineSeriesType {
    Direct,
    Average,
    Minimum,
    Maximum,
}

impl Default for LineSeriesType {
    fn default() -> Self {
        Self::Direct
    }
}