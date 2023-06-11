use yew::prelude::*;

pub struct Sidebar {

}

#[derive(PartialEq, Properties)]
pub struct SidebarProperties {
    #[prop_or_default]
    pub children: Children,
}

pub enum SidebarMessages {

}

impl Component for Sidebar {
    type Message = SidebarMessages;
    type Properties = SidebarProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <div class={format!("sidebar")}>
                {for ctx.props().children.iter()}
            </div>
        )
    }
}