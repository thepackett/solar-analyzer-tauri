use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AxisData {
    pub data_type: AxisDataType,
    pub required_data_option: AxisDataOptions,
    pub additional_data_options: Vec<AxisDataOptions>,
}


#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AxisDataType {
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

impl AxisDataType {
    pub fn get_name(&self) -> String {
        match self {
            AxisDataType::Time => "Time".to_owned(),
            AxisDataType::BatteryVoltage => "Battery Voltage".to_owned(),
            AxisDataType::BatteryAmps => "Battery Amps".to_owned(),
            AxisDataType::SolarWatts => "Solar Watts".to_owned(),
            AxisDataType::LoadWatts => "Load Watts".to_owned(),
            AxisDataType::StateOfChargePercent => "S.O.C. %".to_owned(),
            AxisDataType::CellVoltage(cell) => format!("Cell #{} Voltage", {cell}),
            AxisDataType::ControllerPanelVoltage(controller) => format!("Controller #{} Voltage", {controller}),
            AxisDataType::ControllerAmps(controller) => format!("Controller #{} Amps", {controller}),
            AxisDataType::ControllerTemperatureF(controller) => format!("Controller #{} TemperatureF", {controller}),
            AxisDataType::Custom(s) => todo!(),
        }
    }

    pub fn get_unit(&self) -> &str {
        match self {
            AxisDataType::Time => "Time",
            AxisDataType::BatteryVoltage
            | AxisDataType::CellVoltage(_) 
            | AxisDataType::ControllerPanelVoltage(_) => "Voltage",
            AxisDataType::BatteryAmps 
            | AxisDataType::ControllerAmps(_) => "Amps",
            AxisDataType::SolarWatts 
            | AxisDataType::LoadWatts => "Watts",
            AxisDataType::StateOfChargePercent => "Percent",
            AxisDataType::ControllerTemperatureF(_) => "°Fahrenheit",
            AxisDataType::Custom(s) => todo!(),
        }
    }
}

pub enum AxisZoom {
    Auto,
    Percent(f32),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AxisDataOptions {
    Sample,
    Average,
    Minimum,
    Maximum,
}


#[derive(PartialEq, Serialize, Deserialize, Default)]
pub struct LineSeriesHolder {
    pub series: Vec<LineSeriesData>,
    pub secondary_series: Vec<LineSeriesData>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct LineSeriesData {
    pub name: String,
    pub data_points: Vec<(f64, f64)>,
    pub x_axis: LineSeriesAxisData,
    pub y_axis: LineSeriesAxisData,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct LineSeriesAxisData {
    pub data_type: AxisDataType,
    pub data_option: AxisDataOptions,
}
