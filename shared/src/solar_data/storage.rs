use serde::{Serialize, Deserialize};

use crate::{solar_data::line::DataLine};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataStorage {
    pub data: Vec<DataLine>,
}

impl Default for DataStorage {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl DataStorage {
    pub fn push_data_line(&mut self, line: DataLine) {
        match self.data.binary_search(&line) {
            Ok(pos) => {
                let current_data = &mut self.data[pos];
                for data in line.into_iter() {
                    current_data.add_data(data);
                }
                // if (current_data.len() < line.len()) {
                //     *current_data = line;
                // } else if ((current_data.len() == line.len()) && (current_data.get_sanity() < line.get_sanity())){
                //     *current_data = line;
                // }
            },
            Err(pos) => self.data.insert(pos, line),
        }
    }

    pub fn combine_data(&mut self, new_data: &DataStorage) {
        for new_data_line in &new_data.data {
            self.push_data_line(new_data_line.clone());
        }
    }
}