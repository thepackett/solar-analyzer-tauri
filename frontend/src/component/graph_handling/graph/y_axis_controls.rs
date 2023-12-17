use std::{collections::HashMap, rc::Rc};

use shared::{solar_data::{cell::AvailableCells, controllers::AvailableControllers}, graph::graph_axis::{AxisDataOption, AxisDataType, AxisControlsRequest, DataUnit}};
use yew::prelude::*;

use crate::component::{control::{modal_window::ModalWindow, channel::Channel, copy_paste::{CopyPaste, Request}}, graph_handling::graph::x_axis_controls::{generate_y_axis_system_controls, generate_y_axis_controller_controls}};

use super::graph_coordination::SharableGraphData;

pub struct YAxisControls {
    modal_open: bool,
    copy_state: Option<(Rc<Option<SharableGraphData>>, Callback<Rc<Option<SharableGraphData>>>)>,
    _context_handle: Option<ContextHandle<(Rc<Option<SharableGraphData>>, Callback<Rc<Option<SharableGraphData>>>)>>,
}

#[derive(PartialEq, Properties)]
pub struct YAxisControlsProps {
    pub current_state: AxisControlsRequest,
    pub callback: Callback<AxisControlsRequest>,
    pub available_cells: AvailableCells,
    pub available_controllers: AvailableControllers,
}

pub enum YAxisControlsMessage {
    CloseModalWindow,
    OpenModalWindow,
    NewYAxisState(HashMap<(), HashMap<DataUnit, Vec<(AxisDataType, AxisDataOption)>>>),
    ContextChanged((Rc<Option<SharableGraphData>>, Callback<Rc<Option<SharableGraphData>>>)),
    Copy,
    Paste,
}

impl Component for YAxisControls {
    type Message = YAxisControlsMessage;
    type Properties = YAxisControlsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (copy_state, _context_handle) = 
        match ctx.link().context::<(Rc<Option<SharableGraphData>>, Callback<Rc<Option<SharableGraphData>>>)>(ctx.link().callback(Self::Message::ContextChanged)) {
            Some((state, handle)) => (Some(state), Some(handle)),
            None => (None, None),
        };

        YAxisControls { 
            modal_open: false,
            copy_state,
            _context_handle,
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
        let oncopypaste = ctx.link().callback(|msg| {
            match msg {
                Request::Copy => Self::Message::Copy,
                Request::Paste => Self::Message::Paste,
            }
        });
        let paste_visible = match &self.copy_state {
            Some((data, _update_callback)) => {
                if let Some(SharableGraphData::YAxisData(_)) = data.as_ref() {
                    true
                } else {
                    false
                }
            },
            None => false,
        };
        

        html!(
            <div>
                <p>{"Y-Axis Controls go here"}</p>
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
                <button class="graph-button" onclick={open_modal}>{"Y-Axis Options"}</button>
                <CopyPaste copy_visible={true} paste_visible={paste_visible} callback={oncopypaste}/>
            </div>
        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
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
                props.callback.emit(control_request);
            },
            Self::Message::ContextChanged(new_copy_state) => {
                self.copy_state = Some(new_copy_state);
                return true;
            },
            Self::Message::Copy => {
                if let Some((_data, update_shared_data)) = &self.copy_state {
                    update_shared_data.emit(Rc::from(Some(SharableGraphData::YAxisData(props.current_state.clone()))));
                }
                return true;
            },
            Self::Message::Paste => {
                if let Some((data, _update_shared_data)) = &self.copy_state {
                    if let Some(SharableGraphData::YAxisData(saved_y_axis_request)) = data.as_ref() {
                        props.callback.emit(saved_y_axis_request.clone());
                    }
                }
            },
        }
        true
    }
}