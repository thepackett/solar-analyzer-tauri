use std::collections::HashMap;

use shared::{solar_data::{cell::AvailableCells, controllers::AvailableControllers}, graph::graph_axis::{AxisDataOption, AxisDataType, AxisControlsRequest, DataUnit}};
use yew::prelude::*;

use crate::component::control::{modal_window::ModalWindow, channel::Channel, channel_checkbox::ChannelCheckbox};

pub struct XAxisControls {
    modal_open: bool,
}

#[derive(PartialEq, Properties)]
pub struct XAxisControlsProps {
    pub current_state: AxisControlsRequest,
    pub callback: Callback<AxisControlsRequest>,
    pub available_cells: AvailableCells,
    pub available_controllers: AvailableControllers,
}

pub enum XAxisControlsMessage {
    CloseModalWindow,
    OpenModalWindow,
    NewXAxisState(HashMap<(), HashMap<DataUnit, Vec<(AxisDataType, AxisDataOption)>>>),
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
        let props = ctx.props();
        let close_modal = ctx.link().callback(|_| {
            Self::Message::CloseModalWindow
        });

        let open_modal = ctx.link().callback(|_e| {
            Self::Message::OpenModalWindow
        });

        let on_channel_close = ctx.link().callback(|map: HashMap<(), HashMap<DataUnit, Vec<(AxisDataType, AxisDataOption)>>>| {
            Self::Message::NewXAxisState(map)
        });

        

        html!(
            <div>
                <p>{"X-Axis Controls go here"}</p>
                <ModalWindow visible={self.modal_open} close_modal_callback={close_modal}>
                    <div>
                        <p>{"Modal window content example"}</p>
                        // calculate all axis options, axis option names, and generate the checkboxes
                        <Channel<(),DataUnit,(AxisDataType,AxisDataOption)> on_destroy_callback={on_channel_close}>
                            {generate_x_axis_time_controls(&props.current_state)}
                            {generate_y_axis_system_controls(&props.available_cells, &props.current_state)}
                            {generate_y_axis_controller_controls(&props.available_controllers, &props.current_state)}
                        </Channel<(),DataUnit,(AxisDataType,AxisDataOption)>>
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
            Self::Message::NewXAxisState(map) => {
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


pub fn generate_x_axis_time_controls(current_state: &AxisControlsRequest) -> Html {
    let active_checkboxes = current_state.requests.iter().filter_map(|(data_type, data_option)| {
        match data_type {
            AxisDataType::Time
            | AxisDataType::PeriodicTime => Some((data_type.clone(), data_option.clone())),
            _ => None,
        }
    }).collect::<Vec<_>>();
    html!(
        <>
        <p>{"Time"}</p>
            {get_channel_checkbox(&active_checkboxes, AxisDataType::Time, AxisDataOption::Average)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::PeriodicTime, AxisDataOption::Average)}
        </>
    )
}

pub fn generate_y_axis_system_controls(cells: &AvailableCells, current_state: &AxisControlsRequest) -> Html {
    let active_checkboxes = current_state.requests.iter().filter_map(|(data_type, data_option)| {
        match data_type {
            AxisDataType::BatteryVoltage 
            | AxisDataType::BatteryAmps 
            | AxisDataType::SolarWatts 
            | AxisDataType::LoadWatts 
            | AxisDataType::StateOfChargePercent 
            | AxisDataType::CellVoltage(_) => Some((data_type.clone(), data_option.clone())),
            _ => None,
        }
    }).collect::<Vec<_>>();
    html!(
        <>
        <p>{"Battery Voltage"}</p>
            {get_channel_checkbox(&active_checkboxes, AxisDataType::BatteryVoltage, AxisDataOption::Average)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::BatteryVoltage, AxisDataOption::Minimum)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::BatteryVoltage, AxisDataOption::Maximum)}
        <p>{"Battery Amps"}</p>
            {get_channel_checkbox(&active_checkboxes, AxisDataType::BatteryAmps, AxisDataOption::Average)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::BatteryAmps, AxisDataOption::Minimum)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::BatteryAmps, AxisDataOption::Maximum)}
        <p>{"Solar Watts"}</p>
            {get_channel_checkbox(&active_checkboxes, AxisDataType::SolarWatts, AxisDataOption::Average)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::SolarWatts, AxisDataOption::Minimum)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::SolarWatts, AxisDataOption::Maximum)}
        <p>{"Load Watts"}</p>
            {get_channel_checkbox(&active_checkboxes, AxisDataType::LoadWatts, AxisDataOption::Average)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::LoadWatts, AxisDataOption::Minimum)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::LoadWatts, AxisDataOption::Maximum)}
        <p>{"State of Charge Percent"}</p>
            {get_channel_checkbox(&active_checkboxes, AxisDataType::StateOfChargePercent, AxisDataOption::Average)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::StateOfChargePercent, AxisDataOption::Minimum)}
            {get_channel_checkbox(&active_checkboxes, AxisDataType::StateOfChargePercent, AxisDataOption::Maximum)}

            {cells.get_cells().iter().map(|cell| {
                html!(
                    <>
                    <p>{format!("Cell #{} Voltage", cell)}</p>
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::CellVoltage(*cell), AxisDataOption::Average)}
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::CellVoltage(*cell), AxisDataOption::Minimum)}
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::CellVoltage(*cell), AxisDataOption::Maximum)}
                    </>
                )
            }).collect::<Html>()}
        </>
    )
}

pub fn generate_y_axis_controller_controls(controllers: &AvailableControllers, current_state: &AxisControlsRequest) -> Html {
    let active_checkboxes = current_state.requests.iter().filter_map(|(data_type, data_option)| {
        match data_type {
            AxisDataType::ControllerPanelVoltage(_) 
            | AxisDataType::ControllerAmps(_)
            | AxisDataType::ControllerTemperatureF(_) => Some((data_type.clone(), data_option.clone())),
            _ => None,
        }
    }).collect::<Vec<_>>();
    html!(
        <>
            {controllers.get_controllers().iter().map(|controller| {
                html!(
                    <>
                    <p>{format!("Controller #{} Pannel Voltage", controller)}</p>
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::ControllerPanelVoltage(*controller), AxisDataOption::Average)}
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::ControllerPanelVoltage(*controller), AxisDataOption::Minimum)}
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::ControllerPanelVoltage(*controller), AxisDataOption::Maximum)}
                    <p>{format!("Controller #{} Amps", controller)}</p>
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::ControllerAmps(*controller), AxisDataOption::Average)}
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::ControllerAmps(*controller), AxisDataOption::Minimum)}
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::ControllerAmps(*controller), AxisDataOption::Maximum)}
                    <p>{format!("Controller #{} TempF", controller)}</p>
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::ControllerTemperatureF(*controller), AxisDataOption::Average)}
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::ControllerTemperatureF(*controller), AxisDataOption::Minimum)}
                        {get_channel_checkbox(&active_checkboxes, AxisDataType::ControllerTemperatureF(*controller), AxisDataOption::Maximum)}
                    </>
                )
            }).collect::<Html>()}
        </>
    )
}

pub fn get_channel_checkbox(active_checkboxes: &Vec<(AxisDataType, AxisDataOption)>, axis_type: AxisDataType, axis_option: AxisDataOption) -> Html {
    html!(
        if active_checkboxes.contains(&(axis_type.clone(), axis_option.clone())) {
            <ChannelCheckbox<(),DataUnit,(AxisDataType,AxisDataOption)> system={()} channel={axis_type.get_unit()} id={(axis_type,axis_option)} init_checked={true}/>
        } else {
            <ChannelCheckbox<(),DataUnit,(AxisDataType,AxisDataOption)> system={()} channel={axis_type.get_unit()} id={(axis_type,axis_option)} init_checked={false}/>
        }
    )
}