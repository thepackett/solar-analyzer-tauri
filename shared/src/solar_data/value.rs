use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use strum_macros::IntoStaticStr;


//Note that cell and controller are both zero indexed.
#[derive(IntoStaticStr, Clone, Debug, Serialize, Deserialize)]
pub enum DataValue {
    AlarmCode(i32),
    BatteryVoltage(f32),
    BatteryAmps(f32),
    SolarWatts(f32),
    LoadWatts(f32),
    StateOfChargePercent(f32),
    AmpHoursSinceMidnight(f32),
    CellVoltage { cell: u16, voltage: f32 },
    ControllerPanelVoltage { controller: u16, voltage: f32 },
    ControllerBatteryVoltage { controller: u16, voltage: f32 },
    ControllerAmps { controller: u16, amps: f32 },
    ControllerTemperatureF { controller: u16, temperature: f32 },
    StatisticsCellVoltageHigh{cell: u16, voltage: f32},
    StatisticsCellVoltageLow{cell: u16, voltage: f32},
    StatisticsSolarWatts(f32),
    StatisticsLoadWatts(f32),
    StatisticsStateOfChargePercentHigh(f32),
    StatisticsStateOfChargePercentLow(f32),
}


impl DataValue {

}

impl Ord for DataValue {
    //All enum types are the same, unless they have a controller / cell field, in which case they
    //are only the same if the controller / cell field is the same
    fn cmp(&self, other: &Self) -> Ordering {
        let self_as_str: &'static str = self.into();
        let other_as_str: &'static str = other.into();
        if let DataValue::CellVoltage { cell, voltage: _ } = self {
            let self_cell = cell;
            if let DataValue::CellVoltage { cell, voltage: _ } = other {
                return self_cell.cmp(cell)
            }
        }
        if let DataValue::ControllerPanelVoltage { controller, voltage: _ } = self {
            let self_controller = controller;
            if let DataValue::ControllerPanelVoltage { controller, voltage: _ } = other {
                return self_controller.cmp(controller)
            }
        }
        if let DataValue::ControllerBatteryVoltage { controller, voltage: _ } = self {
            let self_controller = controller;
            if let DataValue::ControllerBatteryVoltage { controller, voltage: _ } = other {
                return self_controller.cmp(controller)
            }
        }
        if let DataValue::ControllerAmps { controller, amps: _ } = self {
            let self_controller = controller;
            if let DataValue::ControllerAmps { controller, amps: _ } = other {
                return self_controller.cmp(controller)
            }
        }
        if let DataValue::ControllerTemperatureF { controller, temperature: _ } = self {
            let self_controller = controller;
            if let DataValue::ControllerTemperatureF { controller, temperature: _ } = other {
                return self_controller.cmp(controller)
            }
        }
        if let DataValue::StatisticsCellVoltageHigh { cell, voltage: _ } = self {
            let self_cell = cell;
            if let DataValue::StatisticsCellVoltageHigh { cell, voltage: _ } = other {
                return self_cell.cmp(cell)
            }
        }
        if let DataValue::StatisticsCellVoltageLow { cell, voltage: _ } = self {
            let self_cell = cell;
            if let DataValue::StatisticsCellVoltageLow { cell, voltage: _ } = other {
                return self_cell.cmp(cell)
            }
        }
        self_as_str.cmp(other_as_str)
    }
}

impl Eq for DataValue {}

impl PartialOrd for DataValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DataValue {
    fn eq(&self, other: &Self) -> bool {
        Into::<&'static str>::into(self) == Into::<&'static str>::into(other)
    }
}

// impl std::fmt::Display for DataValue {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             DataValue::AlarmCode(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//             DataValue::BatteryVoltage(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//             DataValue::BatteryAmps(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//             DataValue::SolarWatts(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//             DataValue::LoadWatts(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//             DataValue::StateOfChargePercent(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//             DataValue::AmpHoursSinceMidnight(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//             DataValue::CellVoltage { cell, voltage } => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}, Cell#{}: {}", self_as_str, cell, voltage).as_ref())
//             },
//             DataValue::ControllerPanelVoltage { controller, voltage } => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}, Controller#{}: {}", self_as_str, controller, voltage).as_ref())
//             },
//             DataValue::ControllerBatteryVoltage { controller, voltage } => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}, Controller#{}: {}", self_as_str, controller, voltage).as_ref())
//             },
//             DataValue::ControllerAmps { controller, amps } => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}, Controller#{}: {}", self_as_str, controller, amps).as_ref())
//             },
//             DataValue::ControllerTemperatureF { controller, temperature } => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}, Controller#{}: {}", self_as_str, controller, temperature).as_ref())
//             },
//             DataValue::StatisticsCellVoltageHigh { cell, voltage } =>{
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}, Cell#{}: {}", self_as_str, cell, voltage).as_ref())
//             },
//             DataValue::StatisticsCellVoltageLow { cell, voltage } => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}, Cell#{}: {}", self_as_str, cell, voltage).as_ref())
//             },
//             DataValue::StatisticsSolarWatts(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//             DataValue::StatisticsLoadWatts(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//             DataValue::StatisticsStateOfChargePercentHigh(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//             DataValue::StatisticsStateOfChargePercentLow(v) => {
//                 let self_as_str: &'static str = self.into();
//                 f.write_str(format!("{}: {}", self_as_str, v).as_ref())
//             },
//         }
//     }   
// }