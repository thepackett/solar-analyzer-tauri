use shared::solar_data::{cell::AvailableCells, controllers::AvailableControllers};
use yew::prelude::*;

use crate::component::control::modal_window::ModalWindow;

pub struct XAxisControls {
    modal_open: bool,
}

#[derive(PartialEq, Properties)]
pub struct XAxisControlsProps {
    pub callback: Callback<XAxisControlsRequest>,
    pub available_cells: AvailableCells,
    pub available_controllers: AvailableControllers,
}

pub enum XAxisControlsMessage {
    CloseModalWindow,
    OpenModalWindow,
}

impl Component for XAxisControls {
    type Message = XAxisControlsMessage;
    type Properties = XAxisControlsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        XAxisControls { 
            modal_open: false  
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_modal = ctx.link().callback(|_| {
            Self::Message::CloseModalWindow
        });

        let open_modal = ctx.link().callback(|_e| {
            Self::Message::OpenModalWindow
        });

        

        html!(
            <div>
                <p>{"X-Axis Controls go here"}</p>
                <ModalWindow visible={self.modal_open} close_modal_callback={close_modal}>
                    <div>
                        <p>{"Modal window content example"}</p>
                        //calculate all axis options, axis option names, and generate the checkboxes
                    </div>
                </ModalWindow>
                <button onclick={open_modal}>{"X-Axis Options"}</button>
            </div>
        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::CloseModalWindow => self.modal_open = false,
            Self::Message::OpenModalWindow => self.modal_open = true,
        }
        true
    }
}

pub struct XAxisControlsRequest {

}

enum AxisOptions {

}

fn generate_x_axis_time_controls() -> Html {
    html!(

    )
}

fn generate_y_axis_system_controls() -> Html {
    html!(

    )
}

fn generate_y_axis_controller_controls() -> Html {
    html!(

    )
}