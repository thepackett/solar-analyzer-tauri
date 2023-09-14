use yew::prelude::*;

use crate::component::control::modal_window::ModalWindow;

pub struct YAxisControls {
    modal_open: bool,
}

#[derive(PartialEq, Properties)]
pub struct YAxisControlsProps {
    pub callback: Callback<YAxisControlsRequest>,
}

pub enum YAxisControlsMessage {
    CloseModalWindow,
}

impl Component for YAxisControls {
    type Message = YAxisControlsMessage;
    type Properties = YAxisControlsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        YAxisControls {
            modal_open: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let close_modal = ctx.link().callback(|_| {
            Self::Message::CloseModalWindow
        });

        html!(
            <div>
                <p>{"Y-Axis Controls go here"}</p>
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

pub struct YAxisControlsRequest {

}