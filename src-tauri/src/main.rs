#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Mutex, OnceLock};

use shared::{parse::{live_data::LiveData, stored_data::StoredData, traits::TryParse, utils::ParseCompleteReturnValue}, solar_data::{storage::DataStorage, line::DataLine, controllers::AvailableControllers, cell::AvailableCells, value::DataValue}, graph::{graph_axis::{LineSeriesHolder, LineSeriesData, AxisDataType, AxisDataOption, LineSeriesAxisData}, graph_state_request::{GraphStateRequest, Resolution}}};
use tauri::{AppHandle, Manager};

static DATA: OnceLock<Mutex<DataStorage>> = OnceLock::new();
static AVAILABLE_CELLS: OnceLock<Mutex<AvailableCells>> = OnceLock::new();
static AVAILABLE_CONTROLLERS: OnceLock<Mutex<AvailableControllers>> = OnceLock::new();

fn main() {
    DATA.get_or_init(|| {Mutex::from(DataStorage::default())}); 
    AVAILABLE_CELLS.get_or_init(|| {Mutex::from(AvailableCells::default())});
    AVAILABLE_CONTROLLERS.get_or_init(|| {Mutex::from(AvailableControllers::default())});

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_solar_data, retrieve_solar_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(async)]
fn parse_solar_data(name: String, data: String, app: AppHandle) {
    let mut live_data = LiveData::default();
    let mut stored_data = StoredData::default();
    let mut controller_ids = AvailableControllers::default();
    let mut cell_ids = AvailableCells::default();
    for line in data.split('\r') {
        //file string returned by javascript uses carriage return newline delimeters for some reason...
        let whole_line = line.to_owned();
        let line: Vec<&str> = whole_line.split(',').map(|x| x.trim()).collect();
        let live_data_data_line = live_data.try_parse(&line);
        let stored_data_data_line = stored_data.try_parse(&line);
        if let Ok(good_data_line) = live_data_data_line {
          good_data_line.line.iter().for_each(|datavalue| {
            match datavalue {
              DataValue::CellVoltage { cell, voltage: _ } => {
                _ = cell_ids.insert(*cell);
              },
              DataValue::ControllerPanelVoltage { controller, voltage: _ } => {
                _ = controller_ids.insert(*controller);
              },
              DataValue::ControllerBatteryVoltage { controller, voltage: _ } => {
                _ = controller_ids.insert(*controller);
              },
              DataValue::ControllerAmps { controller, amps: _ } => {
                _ = controller_ids.insert(*controller);
              },
              DataValue::ControllerTemperatureF { controller, temperature: _ } => {
                _ = controller_ids.insert(*controller);
              },
              _ => {}
            }
          });
          live_data.data.push_data_line(good_data_line);
        } 
        if let Ok(good_data_line) = stored_data_data_line {
          stored_data.data.push_data_line(good_data_line);
        } 
    }
    let mut combined_data = DataStorage::default();
    combined_data.combine_data(&live_data.data);
    combined_data.combine_data(&stored_data.data);
    let mut data_guard = DATA.get().expect("Value set at program init").lock().unwrap();
    data_guard.combine_data(&combined_data);
    let mut cell_guard = AVAILABLE_CELLS.get().expect("Value set at program init").lock().unwrap();
    cell_guard.combine(&cell_ids);
    let mut controller_guard = AVAILABLE_CONTROLLERS.get().expect("Value set at program init").lock().unwrap();
    controller_guard.combine(&controller_ids);
    let payload = ParseCompleteReturnValue { 
      name, 
      cell_ids: cell_guard.clone(),
      controller_ids: controller_guard.clone(),
    };
    let payload = serde_json::to_string(&payload).unwrap();

    app.emit_all("solar_parse_complete", payload).expect("Failed to emit event");
}





#[tauri::command(async)]
fn retrieve_solar_data(graph_state_request: String, app: AppHandle) {
  let graph_state_request = serde_json::from_str::<GraphStateRequest>(&graph_state_request).unwrap();
  let data_guard = DATA.get().unwrap().lock().unwrap();
  let slice = &data_guard.data[
    match data_guard.data.binary_search(&DataLine::from(graph_state_request.time_frame.start)) {
      Ok(index) => index,
      Err(index) => index,
    }..
    match data_guard.data.binary_search(&DataLine::from(graph_state_request.time_frame.end)) {
      Ok(index) => index,
      Err(index) => index,
    }
  ]; 

  let series_data = {
    let mut container = LineSeriesHolder::default();
    let resolution =  graph_state_request.time_frame.get_resolution();

    graph_state_request.x_axis.requests.iter().cloned().for_each(|(x_data_type, x_data_option)| {
      graph_state_request.y_axis.0.requests.iter().cloned().for_each(|(y_data_type, y_data_option)| {
        //We know the x axis data type and current option, and we know the y_axis data type and current option. We have all we need to collect data.
        let data = get_line_series_data(&slice, &resolution, &x_data_type, &x_data_option, &y_data_type, &y_data_option);
        let name = generage_series_name(
          &x_data_type, 
          &x_data_option, 
          &y_data_type, 
          &y_data_option);

        container.series.push(
          LineSeriesData { 
            name: name, 
            data_points: data, 
            x_axis: LineSeriesAxisData { data_type: x_data_type.clone(), data_option: x_data_option.clone() }, 
            y_axis: LineSeriesAxisData { data_type: y_data_type, data_option: y_data_option },
          });
      });

      //Secondary y axis
      graph_state_request.y_axis.1.requests.iter().cloned().for_each(|(y_data_type, y_data_option)| {
        //We know the x axis data type and current option, and we know the y_axis data type and current option. We have all we need to collect data.
        let data = get_line_series_data(&slice, &resolution, &x_data_type, &x_data_option, &y_data_type, &y_data_option);
        let name = generage_series_name(
          &x_data_type, 
          &x_data_option, 
          &y_data_type, 
          &y_data_option);


        container.secondary_series.push(
          LineSeriesData { 
            name: name, 
            data_points: data, 
            x_axis: LineSeriesAxisData { data_type: x_data_type.clone(), data_option: x_data_option.clone() }, 
            y_axis: LineSeriesAxisData { data_type: y_data_type, data_option: y_data_option },
          });
        });
    });
    container
  };
  // series_data.series.iter().for_each(|x| {
  //   println!("Series name: {}, Data Points: {}", x.name.clone(), x.data_points.len());
  // });
  let line_series_holder = format!("{}\\{}", serde_json::to_string(&series_data).unwrap(), graph_state_request.graph_id);
  app.emit_all("data_request_complete", line_series_holder).expect("Failed to emit event");
}


fn get_line_series_data(data: &[DataLine], resolution: &Resolution, x_axis_data_type: &AxisDataType, x_axis_data_option: &AxisDataOption, 
                          y_axis_data_type: &AxisDataType, y_axis_data_option: &AxisDataOption) -> Vec<(f64, f64)> {
  let data = data.iter().cloned().filter_map(|line| {
    let time = line.unix_time;
    let x = line.calculate_axis_data(x_axis_data_type.clone());
    let y = line.calculate_axis_data(y_axis_data_type.clone());
    match x {
      Some(x) => match y {
          Some(y) => {
            Some((time, x, y))
          },
          None => None,
        },
      None => None,
    }
  }).fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, element| {
    acc.0.push(element.0);
    acc.1.push(element.1);
    acc.2.push(element.2);
    acc
  });

  let x_vec = process_axis_data(data.0.iter().cloned().zip(data.1.into_iter()), resolution, x_axis_data_option);
  let y_vec = process_axis_data(data.0.into_iter().zip(data.2.into_iter()), resolution, y_axis_data_option);

  x_vec.into_iter().zip(y_vec.into_iter()).collect::<Vec<_>>()
}



fn process_axis_data<T>(mut data: T, resolution: &Resolution, axis_option: &AxisDataOption) -> Vec<f64> 
where
  T: Iterator<Item = (i64, f64)>
{
  let time_interval = resolution.get_timestamp_offset();

  let processed_axis_data = match axis_option {
    // AxisDataOptions::Sample => {
    //   let mut last: i64 = 0;
    //   let sampled_data = data.into_iter().filter_map(|current| {
    //     if current.0 - last >= time_interval {
    //       //Set the last value to the start of the current time "bucket"
    //       last = current.0 - current.0 % time_interval;
    //       Some(current.1)
    //     } else {
    //       None
    //     }
    //   }).collect::<Vec<_>>();
    //   sampled_data
    // },
    AxisDataOption::Average => {
      let series_data_average: Vec<f64> = if let Some(first) = data.nth(0) {
        let mut storage = vec![first];
        let mut average_data = data.filter_map(|current| {
          if current.0 - storage.last().expect("Storage is never empty").0 >= time_interval {
            let count = storage.len();
            let sum = storage.drain(..).fold((0i64, 0f64), |acc, point| {
              (acc.0 + point.0, acc.1 + point.1)
            });
            let average = sum.1 / count as f64;
            storage.push((current.0 - current.0 % time_interval, current.1));
            Some(average)
          } else {
            storage.push((current.0 - current.0 % time_interval, current.1));
            None
          }
        }).collect::<Vec<_>>();
        if storage.len() > 0 {
          let count = storage.len();
          let sum = storage.drain(..).fold((0i64, 0f64), |acc, point| {
            (acc.0 + point.0, acc.1 + point.1)
          });
          let average = sum.1 / count as f64;
          average_data.push(average);
        }
        average_data
      } else {
        Vec::new()
      };
      series_data_average
    },
    AxisDataOption::Minimum => {
      let series_data_minimum: Vec<f64> = if let Some(first) = data.nth(0) {
        let mut storage = vec![first];
        let mut minimum_data = data.filter_map(|current| {
          if current.0 - storage.last().expect("Storage is never empty").0 >= time_interval {
            let minimum = storage.drain(..).reduce(|acc, current| {
              if current.1 < acc.1 {
                current
              } else {
                acc
              }
            }).expect("Storage is never empty");
            let minimum = minimum.1;
            storage.push((current.0 - current.0 % time_interval, current.1));
            Some(minimum)
          } else {
            storage.push((current.0 - current.0 % time_interval, current.1));
            None
          }
        }).collect::<Vec<_>>();
        if storage.len() > 0 {
          let minimum = storage.drain(..).reduce(|acc, current| {
            if current.1 < acc.1 {
              current
            } else {
              acc
            }
          }).expect("Storage is proven non-empty");
          let minimum = minimum.1;
          minimum_data.push(minimum);
        }
        minimum_data
      } else {
        Vec::new()
      };
      series_data_minimum
    },
    AxisDataOption::Maximum => {
      let series_data_maximum: Vec<f64> = if let Some(first) = data.nth(0) {
        let mut storage = vec![first];
        let mut maximum_data = data.filter_map(|current| {
          if current.0 - storage.last().expect("Storage is never empty").0 >= time_interval {
            let maximum = storage.drain(..).reduce(|acc, current| {
              if current.1 > acc.1 {
                current
              } else {
                acc
              }
            }).expect("Storage is never empty");
            let maximum = maximum.1;
            storage.push((current.0 - current.0 % time_interval, current.1));
            Some(maximum)
          } else {
            storage.push((current.0 - current.0 % time_interval, current.1));
            None
          }
        }).collect::<Vec<_>>();
        if storage.len() > 0 {
          let maximum = storage.drain(..).reduce(|acc, current| {
            if current.1 < acc.1 {
              current
            } else {
              acc
            }
          }).expect("Storage is proven non-empty");
          let maximum = maximum.1;
          maximum_data.push(maximum);
        }
        maximum_data
      } else {
        Vec::new()
      };
      series_data_maximum
    },
  };
  // println!("Processed axis has {} elements", processed_axis_data.len());
  processed_axis_data
}

fn generage_series_name(x_axis_data_type: &AxisDataType, x_axis_option: &AxisDataOption, y_axis_data_type: &AxisDataType, y_axis_option: &AxisDataOption) -> String {
  let name_prefix = if *x_axis_data_type != AxisDataType::Time {
    match x_axis_option {
      // AxisDataOptions::Sample => format!("{}", x_axis_data_type.get_name()),
      AxisDataOption::Average => format!("{} {}", x_axis_data_type.get_name(), "Avg"),
      AxisDataOption::Minimum => format!("{} {}", x_axis_data_type.get_name(), "Min"),
      AxisDataOption::Maximum => format!("{} {}", x_axis_data_type.get_name(), "Max"),
    }
  } else {
    "".to_string()
  };
  let name_suffix = if *y_axis_data_type != AxisDataType::Time {
    match y_axis_option {
      // AxisDataOptions::Sample => format!("{}", y_axis_data_type.get_name()),
      AxisDataOption::Average => format!("{} {}", y_axis_data_type.get_name(), "Avg"),
      AxisDataOption::Minimum => format!("{} {}", y_axis_data_type.get_name(), "Min"),
      AxisDataOption::Maximum => format!("{} {}", y_axis_data_type.get_name(), "Max"),
    }
  } else {
    "".to_string()
  };
  let name = match name_prefix.is_empty() {
    true => {
      match name_suffix.is_empty() {
        true => {
          "Time vs Time".to_string()
        },
        false => {
          name_suffix
        }
      }
    },
    false => {
      match name_suffix.is_empty() {
        true => {
          name_prefix
        },
        false => {
          format!("{}, {}", name_prefix, name_suffix)
        }
      }
    }
  };
  name
}

