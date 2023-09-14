// use std::rc::Rc;

// use yew::prelude::*;

// use crate::component::message_handling::simple_message::SimpleMessageProperties;

// #[derive(PartialEq)]
// pub struct AppState {
//     pub update_app_state_callback: Callback<Rc<AppState>>,
//     pub notification_callback: Option<Callback<SimpleMessageProperties>>,

// }

// pub struct AppStateHolder {
//     state: Rc<AppState>,
// }

// #[derive(PartialEq, Properties)]
// pub struct AppStateHolderProperties {
//     #[prop_or_default]
//     pub children: Children
// }

// pub enum AppStateHolderMessage {
//     NewState(Rc<AppState>),
// }

// impl Component for AppStateHolder {
//     type Message = AppStateHolderMessage;
//     type Properties = AppStateHolderProperties;

//     fn create(ctx: &Context<Self>) -> Self {
//         let update_app_state_callback = ctx.link().callback(AppStateHolderMessage::NewState);

//         AppStateHolder { 
//             state: Rc::from(
//                 AppState { 
//                     update_app_state_callback: update_app_state_callback,
//                     notification_callback: None 
//                 }
//             ) 
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             AppStateHolderMessage::NewState(app_state) => {
//                 self.state = app_state;
//                 true
//             },
//         }
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html!(
//             <ContextProvider<Rc<AppState>> context={self.state.clone()}>
//                 {for ctx.props().children.iter()}
//             </ContextProvider<Rc<AppState>>>
//         )
//     }
// }