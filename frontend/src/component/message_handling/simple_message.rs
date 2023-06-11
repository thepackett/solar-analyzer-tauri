use yew::prelude::*;

//Component that represents a message
#[derive(Clone)]
pub struct SimpleMessage {

}

pub enum SimpleMessageMessage {
}

#[derive(Clone, PartialEq, Properties)]
pub struct SimpleMessageProperties {
    pub class: AttrValue,
    pub message: AttrValue,
}


impl Component for SimpleMessage {
    type Message = SimpleMessageMessage;
    type Properties = SimpleMessageProperties;

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html!(
            <div class={format!("message {}", props.class)}>
                <p>{props.message.clone()}</p>
            </div>
        )
    }
}