use yew::prelude::*;

pub struct Button {

}

#[derive(PartialEq, Properties)]
pub struct ButtonProperties {
    pub class: AttrValue,
    pub onclick: Callback<MouseEvent, ()>,
    #[prop_or_default]
    pub children: Children,
}

pub enum ButtonMessages {

}

impl Component for Button {
    type Message = ButtonMessages;
    type Properties = ButtonProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html!(
            <div class={format!("button {}", props.class.to_string())} onclick={props.onclick.clone()}>
                {for props.children.iter()}
            </div>
        )
    }
}