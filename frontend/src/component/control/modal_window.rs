use yew::prelude::*;

use crate::component::visual::svg::close_button::CloseButton;

pub struct ModalWindow {}

#[derive(PartialEq, Properties)]
pub struct ModalWindowProps {
    #[prop_or_default]
    pub children: Children,
    pub visible: bool,
    pub close_modal_callback: Callback<()>,
}

pub enum ModalWindowMessage {
    CloseModalWindow,
}

impl Component for ModalWindow {
    type Message = ModalWindowMessage;
    type Properties = ModalWindowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        ModalWindow {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let onclick = ctx.link().callback(|_e: MouseEvent| {
            Self::Message::CloseModalWindow
        });

        html!(
            if props.visible {
                <div class={"modal-window-background"} onclick={onclick}>
                    <CloseButton class={"modal-close-button"}/>
                </div>
                <div class={"modal-window-content"}>
                    {for props.children.iter()}
                </div>
            }
        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ModalWindowMessage::CloseModalWindow => ctx.props().close_modal_callback.emit(()),
        }
        true
    }
}