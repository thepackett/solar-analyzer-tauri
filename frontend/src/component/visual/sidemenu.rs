use yew::prelude::*;

pub struct Sidemenu {

}

#[derive(PartialEq, Properties)]
pub struct SidemenuProperties {
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub children: Children
}

pub enum SidemenuMessages {

}

impl Component for Sidemenu {
    type Message = SidemenuMessages;
    type Properties = SidemenuProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        html!(
            <div class={format!("sidemenu {}", props.class.to_string())}>
                {for props.children.iter()}
            </div>
        )
    }
}