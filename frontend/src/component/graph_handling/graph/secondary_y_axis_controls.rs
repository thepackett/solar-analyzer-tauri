use yew::prelude::*;

use crate::component::control::modal_window::ModalWindow;

pub struct SecYAxisControls {
    modal_open: bool,
}

#[derive(PartialEq, Properties)]
pub struct SecYAxisControlsProps {
    pub callback: Callback<SecYAxisControlsRequest>,
}

pub enum SecYAxisControlsMessage {
    CloseModalWindow,
}

impl Component for SecYAxisControls {
    type Message = SecYAxisControlsMessage;
    type Properties = SecYAxisControlsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        SecYAxisControls {
            modal_open: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_modal = ctx.link().callback(|_| {
            Self::Message::CloseModalWindow
        });

        html!(
            <div>
                <p>{"Secondary Y-Axis Controls go here"}</p>
                <ModalWindow visible={self.modal_open} close_modal_callback={close_modal}>
                </ModalWindow>
            </div>
        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::CloseModalWindow => self.modal_open = false,
        }
        true
    }
}

pub struct SecYAxisControlsRequest {

}