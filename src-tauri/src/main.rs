#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use shared::{parse::{live_data::LiveData, stored_data::StoredData, traits::TryParse}, solar_data::storage::DataStorage};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_solar_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[tauri::command(async)]
// fn plot_chart(canvas: DataStorage) -> Result<(),()> {
//     todo!()
// }

#[tauri::command(async)]
fn parse_solar_data(data: String) -> Result<DataStorage, String> {
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
        } 
        if let Ok(good_data_line) = stored_data_data_line {
          stored_data.data.push_data_line(good_data_line);
        } 
    }
    let mut combined_data = DataStorage::default();
    combined_data.combine_data(&live_data.data);
    combined_data.combine_data(&stored_data.data);
    println!("Parsing complete.");
    println!("{:#?}", combined_data);
    //app_window.emit("parse_complete", combined_data).expect("Failed to emit event");
    Ok(combined_data)
}

// struct GlobalSolarDataSingleton {
//   cell: OnceLock<RwLock<DataStorage>>,
// }

// impl GlobalSolarDataSingleton {
//   const fn new() -> GlobalSolarDataSingleton {
//     GlobalSolarDataSingleton { cell: OnceLock::new() }
//   } 

//   fn init(&self) {
//     self.cell.set(RwLock::from(DataStorage::default())).expect("GlobalSolarDataSingleton can only be initialized once");
//   }

//   fn get(&self) -> &RwLock<DataStorage> {
//     self.cell.get().expect("GlobalSolarDataSingleton must be initialized before use")
//   }
// }

