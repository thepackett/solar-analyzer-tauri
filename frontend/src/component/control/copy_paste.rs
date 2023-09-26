use yew::prelude::*;

pub struct CopyPaste {

}

#[derive(PartialEq, Properties)]
pub struct CopyPasteProps {
    pub copy_visible: bool,
    pub paste_visible: bool,
    pub callback: Callback<Request>,
}

pub enum CopyPasteMessage {
    Copy,
    Paste,
}

pub enum Request {
    Copy,
    Paste,
}

impl Component for CopyPaste {
    type Message = CopyPasteMessage;
    type Properties = CopyPasteProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html!(
            <>
                if props.copy_visible {
                    <p onclick={ctx.link().callback(|_| {Self::Message::Copy})}>{"Copy visible"}</p>
                }
                if props.paste_visible {
                    <p onclick={ctx.link().callback(|_| {Self::Message::Paste})}>{"Paste visible"}</p>
                }
            </>
        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CopyPasteMessage::Copy => ctx.props().callback.emit(Request::Copy),
            CopyPasteMessage::Paste => ctx.props().callback.emit(Request::Paste),
        }
        false
    }
}