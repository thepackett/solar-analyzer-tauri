#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Mutex, OnceLock};

use shared::{parse::{live_data::LiveData, stored_data::StoredData, traits::TryParse}, solar_data::{storage::DataStorage, line::DataLine}, graph::{graph_axis::{LineSeriesHolder, LineSeriesData, LineSeriesType}, graph_state_request::{GraphStateRequest, Resolution}}};
use tauri::{AppHandle, Manager};
use time::OffsetDateTime;

static DATA: OnceLock<Mutex<DataStorage>> = OnceLock::new();

fn main() {
    DATA.get_or_init(|| {Mutex::from(DataStorage::default())}); 

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_solar_data, retrieve_solar_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(async)]
fn parse_solar_data(data: String, app: AppHandle) {
    let mut live_data = LiveData::default();
    let mut stored_data = StoredData::default();
    for line in data.split('\r') {
        //file string returned by javascript uses carriage return newline delimeters for some reason...
        //println!("Line: {}", &line);
        let whole_line = line.to_owned();
        let line: Vec<&str> = whole_line.split(',').map(|x| x.trim()).collect();
        let live_data_data_line = live_data.try_parse(&line);
        let stored_data_data_line = stored_data.try_parse(&line);
        if let Ok(good_data_line) = live_data_data_line {
          live_data.data.push_data_line(good_data_line);
          // let mut good_data = DataStorage::default();
          // good_data.push_data_line(good_data_line);
          // app.emit_all("solar_parse_complete", good_data).expect("Failed to emit event");
        } 
        if let Ok(good_data_line) = stored_data_data_line {
          stored_data.data.push_data_line(good_data_line);
          // let mut good_data = DataStorage::default();
          // good_data.push_data_line(good_data_line);
          // app.emit_all("solar_parse_complete", good_data).expect("Failed to emit event");
        } 

        // if stored_data.data.len() > 100 || live_data.data.len() > 100 {
        //   let mut combined_data = DataStorage::default();
        //   combined_data.combine_data(&live_data.data);
        //   combined_data.combine_data(&stored_data.data);
        //   app.emit_all("solar_parse_complete", combined_data).expect("Failed to emit event");
        //   live_data.data = DataStorage::default();
        //   stored_data.data = DataStorage::default();
        // }
    }
    let mut combined_data = DataStorage::default();
    combined_data.combine_data(&live_data.data);
    combined_data.combine_data(&stored_data.data);
    println!("Parsing complete. Min date = {}", combined_data.data.first().unwrap().unix_time);
    let mut guard = DATA.get().unwrap().lock().unwrap();
    guard.combine_data(&combined_data);
    //println!("{:#?}", combined_data);
    app.emit_all("solar_parse_complete", "Message... Probably the file name in future.").expect("Failed to emit event");
}





#[tauri::command(async)]
fn retrieve_solar_data(graph_state_request: String, app: AppHandle) {
  let graph_state_request = serde_json::from_str::<GraphStateRequest>(&graph_state_request).unwrap();
  let data_guard = DATA.get().unwrap().lock().unwrap();
  let slice = &data_guard.data[
    data_guard.data.binary_search(&DataLine::from(graph_state_request.start_time)).unwrap_or(0)..
    data_guard.data.binary_search(&DataLine::from(graph_state_request.end_time)).unwrap_or(data_guard.len())
  ]; 

  //We have the slice of relevant time frame. Now to get the relevant data, respecting the desired resolution.
  let series_data = match graph_state_request.resolution {
    Resolution::OneMinute | Resolution::FiveMinute | Resolution::FifteenMinute | Resolution::OneHour => {
      //For these resolutions we can simply iterate over the data checking to see if the required amount of time has passed, and filter the results
      let mut last: i64 = 0;
      let data = slice.iter().filter_map(|current| {
        if current.unix_time - last >= graph_state_request.resolution.get_timestamp_offset() {
          last = current.unix_time;
          Some(current)
        } else {
          None
        }
      }).collect::<Vec<_>>();
      //Now we extract the data we're interested in into different LineSeriesData structs, and then add those to a LineSeriesHolder.
      let mut container = LineSeriesHolder::default();

      let x_axis = graph_state_request.x_axis;
      let y_axis_primary = graph_state_request.y_axis.0;
      let y_axis_secondary = graph_state_request.y_axis.1;
      
      for axis in y_axis_primary {
        let series_data = data.iter().copied().filter_map(|line| {
          let px = line.calculate_axis_data(x_axis.clone());
          let py = line.calculate_axis_data(axis.clone());
          match px {
            Some(x) => match py {
              Some(y) => Some((x, y)),
              None => None,
            },
            None => None,
          }
        }).collect::<Vec<_>>();

        container.series.push(LineSeriesData { name: axis.get_name(), data_points: series_data, series_type: LineSeriesType::Direct});
      }
      for axis in y_axis_secondary {
        let series_data = data.iter().copied().filter_map(|line| {
          let px = line.calculate_axis_data(x_axis.clone());
          let py = line.calculate_axis_data(axis.clone());
          match px {
            Some(x) => match py {
              Some(y) => Some((x, y)),
              None => None,
            },
            None => None,
          }
        }).collect::<Vec<_>>();

        container.secondary_series.push(LineSeriesData { name: axis.get_name(), data_points: series_data, series_type: LineSeriesType::Direct});
      }
      container
    },
    Resolution::OneDay => {
      let WARNING_UNFINISHED: i32;
      //This part of the function is incorrectly implemented. It assumes that time is the x axis, whereas this may not be the case.
      //Leaving as is for now because it is uncertain whether or not I want to be able to graph something other than time on the x axis.



      //For this resolution, it doesn't make sense to only pick out one value, so we should take statistics. Report values as Low, Average, High
      //First take up to 96 samples per day.
      let mut last: i64 = 0;
      let data = slice.iter().filter_map(|current| {
        if current.unix_time - last >= Resolution::FifteenMinute.get_timestamp_offset() {
          last = current.unix_time;
          Some(current)
        } else {
          None
        }
      }).collect::<Vec<_>>();
      //Now we extract the data we're interested in into different LineSeriesData structs, and then add those to a LineSeriesHolder.
      let mut container = LineSeriesHolder::default();

      let x_axis = graph_state_request.x_axis;
      let y_axis_primary = graph_state_request.y_axis.0;
      let y_axis_secondary = graph_state_request.y_axis.1;

      for axis in y_axis_primary {
        let series_data = data.iter().copied().filter_map(|line| {
          let px = line.calculate_axis_data(x_axis.clone());
          let py = line.calculate_axis_data(axis.clone());
          match px {
            Some(x) => match py {
              Some(y) => Some((x, y)),
              None => None,
            },
            None => None,
          }
        }).collect::<Vec<_>>();

        //Collect data from above into 24 hour averages and make it into a line series. 
        let series_data_average: Vec<(f64,f64)> = if let Some(first) = series_data.first() {
          let mut storage = vec![first.clone()];
          let mut average_data = series_data.iter().skip(1).copied().filter_map(|point| {
            if OffsetDateTime::from_unix_timestamp(storage.last().expect("Storage is never empty").0 as i64).expect("All stored data are valid unix timestamps").date() 
            == OffsetDateTime::from_unix_timestamp(point.0 as i64).expect("All stored data are valid unix timestamps").date() {
              storage.push(point);
              None
            } else {
              let mut average = storage.iter().fold((0f64, 0f64), |acc, point| {
                (acc.0 + point.0, acc.1 + point.1)
              });
              average = (average.0 / storage.len() as f64, average.1 / storage.len() as f64);
              storage.clear();
              storage.push(point);
              Some(average)
            }
          }).collect::<Vec<(f64,f64)>>();
            let mut average = storage.iter().copied().reduce(|acc, point| {
              (acc.0 + point.0, acc.1 + point.1)
            }).expect("Storage is never empty");
            average = (average.0 / storage.len() as f64, average.1 / storage.len() as f64);
            average_data.push(average);
            average_data
        } else {
          Vec::new()
        };
        
        container.series.push(LineSeriesData { name: format!("{}: Daily Average", axis.get_name()), data_points: series_data_average, series_type: LineSeriesType::Average});


        //Collect the data from above into 24 hour minimas and make it into a line series.
        let series_data_minimum: Vec<(f64,f64)> = if let Some(first) = series_data.first() {
          let mut storage = vec![first.clone()];
          let mut min_data = series_data.iter().skip(1).copied().filter_map(|point| {
            if OffsetDateTime::from_unix_timestamp(storage.last().expect("Storage is never empty").0 as i64).expect("All stored data are valid unix timestamps").date() 
            == OffsetDateTime::from_unix_timestamp(point.0 as i64).expect("All stored data are valid unix timestamps").date() {
              storage.push(point);
              None
            } else {
              let min = storage.drain(..).reduce(|acc, point| {
                if point.1 < acc.1 {
                  point
                } else {
                  acc
                }
              }).expect("Storage is never empty");
              storage.push(point);
              Some(min)
            }
          }).collect::<Vec<(f64,f64)>>();
          let min = storage.drain(..).reduce(|acc, point| {
            if point.1 < acc.1 {
              point
            } else {
              acc
            }
          }).expect("Storage is never empty");
          min_data.push(min);
          min_data
        } else {
          Vec::new()
        };
        
        container.series.push(LineSeriesData { name: format!("{}: Daily Minimum", axis.get_name()), data_points: series_data_minimum, series_type: LineSeriesType::Minimum});


        //Collect the data from above into 24 hour maximas and make it into a line series.
        let series_data_maximum: Vec<(f64,f64)> = if let Some(first) = series_data.first() {
          let mut storage = vec![first.clone()];
          let mut max_data = series_data.iter().skip(1).copied().filter_map(|point| {
            if OffsetDateTime::from_unix_timestamp(storage.last().expect("Storage is never empty").0 as i64).expect("All stored data are valid unix timestamps").date() 
            == OffsetDateTime::from_unix_timestamp(point.0 as i64).expect("All stored data are valid unix timestamps").date() {
              storage.push(point);
              None
            } else {
              let max = storage.drain(..).reduce(|acc, point| {
                if point.1 > acc.1 {
                  point
                } else {
                  acc
                }
              }).expect("Storage is never empty");
              storage.push(point);
              Some(max)
            }
          }).collect::<Vec<(f64,f64)>>();
          let min = storage.drain(..).reduce(|acc, point| {
            if point.1 > acc.1 {
              point
            } else {
              acc
            }
          }).expect("Storage is never empty");
          max_data.push(min);
          max_data
        } else {
          Vec::new()
        };
        
        container.series.push(LineSeriesData { name: format!("{}: Daily Maximum", axis.get_name()), data_points: series_data_maximum, series_type: LineSeriesType::Maximum});

      }


      //----------------------------------------- Repeat the same process for the secondary y axis -------------------------------

      for axis in y_axis_secondary {
        let series_data = data.iter().copied().filter_map(|line| {
          let px = line.calculate_axis_data(x_axis.clone());
          let py = line.calculate_axis_data(axis.clone());
          match px {
            Some(x) => match py {
              Some(y) => Some((x, y)),
              None => None,
            },
            None => None,
          }
        }).collect::<Vec<_>>();

        //Collect data from above into 24 hour averages and make it into a line series. 
        let series_data_average: Vec<(f64,f64)> = if let Some(first) = series_data.first() {
          let mut storage = vec![first.clone()];
          let mut average_data = series_data.iter().skip(1).copied().filter_map(|point| {
            if OffsetDateTime::from_unix_timestamp(storage.last().expect("Storage is never empty").0 as i64).expect("All stored data are valid unix timestamps").date() 
            == OffsetDateTime::from_unix_timestamp(point.0 as i64).expect("All stored data are valid unix timestamps").date() {
              storage.push(point);
              None
            } else {
              let mut average = storage.iter().fold((0f64, 0f64), |acc, point| {
                (acc.0 + point.0, acc.1 + point.1)
              });
              average = (average.0 / storage.len() as f64, average.1 / storage.len() as f64);
              storage.clear();
              storage.push(point);
              Some(average)
            }
          }).collect::<Vec<(f64,f64)>>();
            let mut average = storage.iter().copied().reduce(|acc, point| {
              (acc.0 + point.0, acc.1 + point.1)
            }).expect("Storage is never empty");
            average = (average.0 / storage.len() as f64, average.1 / storage.len() as f64);
            average_data.push(average);
            average_data
        } else {
          Vec::new()
        };
        
        container.secondary_series.push(LineSeriesData { name: format!("{}: Daily Average", axis.get_name()), data_points: series_data_average, series_type: LineSeriesType::Average});


        //Collect the data from above into 24 hour minimas and make it into a line series.
        let series_data_minimum: Vec<(f64,f64)> = if let Some(first) = series_data.first() {
          let mut storage = vec![first.clone()];
          let mut min_data = series_data.iter().skip(1).copied().filter_map(|point| {
            if OffsetDateTime::from_unix_timestamp(storage.last().expect("Storage is never empty").0 as i64).expect("All stored data are valid unix timestamps").date() 
            == OffsetDateTime::from_unix_timestamp(point.0 as i64).expect("All stored data are valid unix timestamps").date() {
              storage.push(point);
              None
            } else {
              let min = storage.drain(..).reduce(|acc, point| {
                if point.1 < acc.1 {
                  point
                } else {
                  acc
                }
              }).expect("Storage is never empty");
              storage.push(point);
              Some(min)
            }
          }).collect::<Vec<(f64,f64)>>();
          let min = storage.drain(..).reduce(|acc, point| {
            if point.1 < acc.1 {
              point
            } else {
              acc
            }
          }).expect("Storage is never empty");
          min_data.push(min);
          min_data
        } else {
          Vec::new()
        };
        
        container.secondary_series.push(LineSeriesData { name: format!("{}: Daily Minimum", axis.get_name()), data_points: series_data_minimum, series_type: LineSeriesType::Minimum});


        //Collect the data from above into 24 hour maximas and make it into a line series.
        let series_data_maximum: Vec<(f64,f64)> = if let Some(first) = series_data.first() {
          let mut storage = vec![first.clone()];
          let mut max_data = series_data.iter().skip(1).copied().filter_map(|point| {
            if OffsetDateTime::from_unix_timestamp(storage.last().expect("Storage is never empty").0 as i64).expect("All stored data are valid unix timestamps").date() 
            == OffsetDateTime::from_unix_timestamp(point.0 as i64).expect("All stored data are valid unix timestamps").date() {
              storage.push(point);
              None
            } else {
              let max = storage.drain(..).reduce(|acc, point| {
                if point.1 > acc.1 {
                  point
                } else {
                  acc
                }
              }).expect("Storage is never empty");
              storage.push(point);
              Some(max)
            }
          }).collect::<Vec<(f64,f64)>>();
          let min = storage.drain(..).reduce(|acc, point| {
            if point.1 > acc.1 {
              point
            } else {
              acc
            }
          }).expect("Storage is never empty");
          max_data.push(min);
          max_data
        } else {
          Vec::new()
        };
        
        container.secondary_series.push(LineSeriesData { name: format!("{}: Daily Maximum", axis.get_name()), data_points: series_data_maximum, series_type: LineSeriesType::Maximum});

      }

      container
    },
  };


  let line_series_holder = serde_json::to_string(&series_data).unwrap();
  app.emit_all("data_request_complete", line_series_holder).expect("Failed to emit event");
}