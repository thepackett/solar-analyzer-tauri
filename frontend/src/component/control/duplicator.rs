use yew::prelude::*;

pub struct Duplicator{}

#[derive(PartialEq, Properties)]
pub struct DuplicatorProps{

}

pub struct DuplicatorMessage{

}

impl Component for Duplicator {
    type Message = DuplicatorMessage;
    type Properties = DuplicatorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Duplicator {  }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}