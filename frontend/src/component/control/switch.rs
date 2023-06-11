use yew::prelude::*;

pub struct Switch {

}

#[derive(Clone, PartialEq, Properties)]
pub struct SwitchProperties {
    pub class: AttrValue,
    pub initial: bool,
    pub onclick: Callback<MouseEvent, ()>,
}

pub enum SwitchMessage {

}

impl Component for Switch {
    type Message = SwitchMessage;
    type Properties = SwitchProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html! {
            <label class={format!("switch-box {}", props.class.to_string())}>
                if props.initial {
                    <input onclick={props.onclick.clone()} type="checkbox" checked=true/>
                } else {
                    <input onclick={props.onclick.clone()} type="checkbox"/>
                }
                <span class={format!("switch-slider")}></span>
            </label>
        }
    }
}