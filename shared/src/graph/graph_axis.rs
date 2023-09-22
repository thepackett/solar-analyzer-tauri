use serde::{Deserialize, Serialize};


#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct AxisControlsRequest {
    pub requests: Vec<(AxisDataType,AxisDataOption)>
}


#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AxisDataType {
    Time,
    PeriodicTime,
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
    // Custom(String),
}

impl AxisDataType {
    pub fn get_name(&self) -> String {
        match self {
            AxisDataType::Time => "Time".to_owned(),
            AxisDataType::PeriodicTime => "Time".to_owned(),
            AxisDataType::BatteryVoltage => "Battery Voltage".to_owned(),
            AxisDataType::BatteryAmps => "Battery Amps".to_owned(),
            AxisDataType::SolarWatts => "Solar Watts".to_owned(),
            AxisDataType::LoadWatts => "Load Watts".to_owned(),
            AxisDataType::StateOfChargePercent => "S.O.C. %".to_owned(),
            AxisDataType::CellVoltage(cell) => format!("Cell #{} Voltage", {cell}),
            AxisDataType::ControllerPanelVoltage(controller) => format!("Controller #{} Voltage", {controller}),
            AxisDataType::ControllerAmps(controller) => format!("Controller #{} Amps", {controller}),
            AxisDataType::ControllerTemperatureF(controller) => format!("Controller #{} TemperatureF", {controller}),
            // AxisDataType::Custom(s) => todo!(),
        }
    }

    pub fn get_unit(&self) -> DataUnit {
        match self {
            AxisDataType::Time => DataUnit::Time,
            AxisDataType::PeriodicTime => DataUnit::PeriodicTime,
            AxisDataType::BatteryVoltage
            | AxisDataType::CellVoltage(_) 
            | AxisDataType::ControllerPanelVoltage(_) => DataUnit::Voltage,
            AxisDataType::BatteryAmps 
            | AxisDataType::ControllerAmps(_) => DataUnit::Amps,
            AxisDataType::SolarWatts 
            | AxisDataType::LoadWatts => DataUnit::Watts,
            AxisDataType::StateOfChargePercent => DataUnit::Percent,
            AxisDataType::ControllerTemperatureF(_) => DataUnit::Farenheight,
            // AxisDataType::Custom(s) => todo!(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub enum DataUnit {
    Time,
    PeriodicTime,
    Voltage,
    Amps,
    Watts,
    Percent,
    Farenheight,
}

impl DataUnit {
    pub fn get_name(&self) -> &'static str {
        match self {
            DataUnit::Time => "Time",
            DataUnit::PeriodicTime => "Time",
            DataUnit::Voltage => "Voltage",
            DataUnit::Amps => "Amps",
            DataUnit::Watts => "Watts",
            DataUnit::Percent => "Percent",
            DataUnit::Farenheight => "°Fahrenheit",
        }
    }
}

pub enum AxisZoom {
    Auto,
    Percent(f32),
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum AxisDataOption {
    // Sample,
    Average,
    Minimum,
    Maximum,
}


#[derive(PartialEq, Serialize, Deserialize, Default, Debug)]
pub struct LineSeriesHolder {
    pub series: Vec<LineSeriesData>,
    pub secondary_series: Vec<LineSeriesData>,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct LineSeriesData {
    pub name: String,
    pub data_points: Vec<(f64, f64)>,
    pub x_axis: LineSeriesAxisData,
    pub y_axis: LineSeriesAxisData,
}

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub struct LineSeriesAxisData {
    pub data_type: AxisDataType,
    pub data_option: AxisDataOption,
}
