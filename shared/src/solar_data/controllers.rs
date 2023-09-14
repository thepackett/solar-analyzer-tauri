use thiserror::Error;


#[derive(Clone, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AvailableControllers {
    controllers: Vec<u16>,
}

impl AvailableControllers {
    pub fn contains(&self, controller_id: u16) -> bool {
        self.controllers.contains(&controller_id)
    }

    pub fn insert(&mut self, controller_id: u16) -> Result<(), AvailableControllersError>{
        match self.controllers.binary_search(&controller_id) {
            Ok(_) => {
                Err(AvailableControllersError::ControllerAlreadyExists)
            },
            Err(index) => {
                self.controllers.insert(index, controller_id);
                Ok(())
            },
        }
    }

    pub fn get_controllers(&self) -> &Vec<u16> {
        &self.controllers
    }

    pub fn combine(&mut self, other: &Self) {
        other.controllers.iter().for_each(|id| {
            _ = self.insert(*id);
        })
    }
}

#[derive(Debug, Error)]
pub enum AvailableControllersError {
    #[error("Tried to insert controller id that already existed.")]
    ControllerAlreadyExists,
}