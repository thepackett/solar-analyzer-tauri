use thiserror::Error;


#[derive(Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AvailableCells {
    cells: Vec<u16>,
}

impl AvailableCells {
    pub fn contains(&self, cell_id: u16) -> bool {
        self.cells.contains(&cell_id)
    }

    pub fn insert(&mut self, cell_id: u16) -> Result<(), AvailableCellsError>{
        match self.cells.binary_search(&cell_id) {
            Ok(_) => {
                Err(AvailableCellsError::CellAlreadyExists)
            },
            Err(index) => {
                self.cells.insert(index, cell_id);
                Ok(())
            },
        }
    }

    pub fn get_cells(&self) -> &Vec<u16> {
        &self.cells
    }

    pub fn combine(&mut self, other: &Self) {
        other.cells.iter().for_each(|id| {
            _ = self.insert(*id);
        })
    }
}

#[derive(Debug, Error)]
pub enum AvailableCellsError {
    #[error("Tried to insert cell id that already existed.")]
    CellAlreadyExists,
}