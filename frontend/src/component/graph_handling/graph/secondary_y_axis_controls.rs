use std::collections::HashMap;

use shared::{solar_data::{cell::AvailableCells, controllers::AvailableControllers}, graph::graph_axis::{AxisDataOption, AxisDataType, AxisControlsRequest, DataUnit}};
use yew::prelude::*;

use crate::component::{control::{modal_window::ModalWindow, channel::Channel}, graph_handling::graph::x_axis_controls::{generate_y_axis_system_controls, generate_y_axis_controller_controls}};

pub struct SecYAxisControls {
    modal_open: bool,
}

#[derive(PartialEq, Properties)]
pub struct SecYAxisControlsProps {
    pub current_state: AxisControlsRequest,
    pub callback: Callback<AxisControlsRequest>,
    pub available_cells: AvailableCells,
    pub available_controllers: AvailableControllers,
}

pub enum SecYAxisControlsMessage {
    CloseModalWindow,
    OpenModalWindow,
    NewYAxisState(HashMap<(), HashMap<DataUnit, Vec<(AxisDataType, AxisDataOption)>>>),
}

impl Component for SecYAxisControls {
    type Message = SecYAxisControlsMessage;
    type Properties = SecYAxisControlsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        SecYAxisControls { 
            modal_open: false  
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let close_modal = ctx.link().callback(|_| {
            Self::Message::CloseModalWindow
        });

        let open_modal = ctx.link().callback(|_e| {
            Self::Message::OpenModalWindow
        });

        let on_channel_close = ctx.link().callback(|map: HashMap<(), HashMap<DataUnit, Vec<(AxisDataType, AxisDataOption)>>>| {
            Self::Message::NewYAxisState(map)
        });

        

        html!(
            <div>
                <p>{"SecY-Axis Controls go here"}</p>
                <ModalWindow visible={self.modal_open} close_modal_callback={close_modal}>
                    <div>
                        <p>{"Modal window content example"}</p>
                        // calculate all axis options, axis option names, and generate the checkboxes
                        <Channel<(),DataUnit,(AxisDataType,AxisDataOption)> on_destroy_callback={on_channel_close}>
                            {generate_y_axis_system_controls(&props.available_cells, &props.current_state)}
                            {generate_y_axis_controller_controls(&props.available_controllers, &props.current_state)}
                        </Channel<(),DataUnit,(AxisDataType,AxisDataOption)>>
                    </div>
                </ModalWindow>
                <button onclick={open_modal}>{"SecY-Axis Options"}</button>
            </div>
        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::CloseModalWindow => self.modal_open = false,
            Self::Message::OpenModalWindow => self.modal_open = true,
            Self::Message::NewYAxisState(map) => {
                let mut control_request = AxisControlsRequest::default();
                map.into_iter().for_each(|(_, channel_map)| {
                    channel_map.into_iter().for_each(|(_data_unit, axis_data)| {
                        axis_data.into_iter().for_each(|data| {
                            control_request.requests.push(data);
                        });
                    });
                });
                ctx.props().callback.emit(control_request);
            },
        }
        true
    }
}