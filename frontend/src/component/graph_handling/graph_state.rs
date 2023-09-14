// use std::rc::Rc;
// use gloo_events::EventListener;
// use shared::graph::{graph_axis::{AxisData, LineSeriesHolder, AxisDataType, AxisDataOptions}, graph_state_request::{GraphStateRequest, Resolution}};
// use time::{PrimitiveDateTime, macros::date, macros::time};
// use wasm_bindgen::{JsCast, UnwrapThrowExt};
// use web_sys::{HtmlElement, CustomEvent};
// use yew::prelude::*;

// use crate::{app_state::AppState, bindings, component::message_handling::simple_message::SimpleMessageProperties};


// #[derive(PartialEq)]
// pub struct GraphState {
//     pub update_graph_state_callback: Callback<Rc<GraphState>>,
//     pub line_series: LineSeriesHolder,
//     pub details: GraphStateRequest,
// }

// pub struct GraphStateHolder {
//     state: Rc<GraphState>,
//     app_state: Rc<AppState>,
//     node_ref: NodeRef,
//     parse_listener: Option<EventListener>,
//     data_listener: Option<EventListener>,
//     _context_handle: ContextHandle<Rc<AppState>>,
// }

// #[derive(PartialEq, Properties)]
// pub struct GraphStateHolderProperties {
//     #[prop_or_default]
//     pub children: Children
// }

// pub enum GraphStateHolderMessage {
//     NewState(Rc<GraphState>),
//     ContextChanged(Rc<AppState>),
//     NewData(LineSeriesHolder),
//     ParseComplete(String),
// }

// impl Component for GraphStateHolder {
//     type Message = GraphStateHolderMessage;
//     type Properties = GraphStateHolderProperties;

//     fn create(ctx: &Context<Self>) -> Self {
//         let update_graph_state_callback = ctx.link().callback(GraphStateHolderMessage::NewState);

//         let (app_state, _context_handle) = 
//             ctx.link().context::<Rc<AppState>>(ctx.link().callback(Self::Message::ContextChanged))
//             .expect("AppState context must be set for GraphState to function.");

//         GraphStateHolder { 
//             state: Rc::from(
//                 GraphState { 
//                     update_graph_state_callback: update_graph_state_callback,
//                     line_series: LineSeriesHolder::default(),
//                     details: GraphStateRequest { 
//                         x_axis: vec![AxisData { 
//                             data_type: AxisDataType::Time, 
//                             required_data_option: AxisDataOptions::Sample,
//                             additional_data_options: Vec::new()
//                          }], 
//                         y_axis: (vec![ 
//                             AxisData { 
//                                 data_type: AxisDataType::StateOfChargePercent, 
//                                 required_data_option: AxisDataOptions::Average,
//                                 additional_data_options: vec![AxisDataOptions::Minimum, AxisDataOptions::Maximum, AxisDataOptions::Sample], 
//                             }],
//                             Vec::new()), 
//                         start_time: PrimitiveDateTime::new(date!(2022-01-01), time!(0:00)).assume_utc().unix_timestamp(),
//                         end_time: PrimitiveDateTime::new(date!(2023-06-01), time!(0:00)).assume_utc().unix_timestamp(), 
//                         resolution: Resolution::OneDay,
//                     }
//                 }
//             ),
//             node_ref: NodeRef::default(),
//             parse_listener: None,
//             data_listener: None,
//             app_state: app_state,
//             _context_handle: _context_handle, 
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             GraphStateHolderMessage::NewState(graph_state) => {
//                 self.state = graph_state;
//             },
//             GraphStateHolderMessage::ContextChanged(app_state) => {
//                 self.app_state = app_state;
//             },
//             GraphStateHolderMessage::NewData(line_series) => {
//                 self.state = Rc::from(
//                     GraphState {
//                         update_graph_state_callback: self.state.update_graph_state_callback.clone(),
//                         line_series: line_series,
//                         details: self.state.details.clone(),
//                     }
//                 )
//             }
//             GraphStateHolderMessage::ParseComplete(name) => {
                // let message = SimpleMessageProperties { 
                //     class: AttrValue::from("notification"), 
                //     message:  AttrValue::from(format!("{} parsing complete.", name)),
                // };
                // self.app_state.notification_callback.clone().expect("Notification callback must be set").emit(message);
                // bindings::retrieve_solar_data(serde_json::to_string(&self.state.details).unwrap())
//             },
//         }
//         true
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
        
//         html!(
//             <div id={"graph_state_holder"} ref={self.node_ref.clone()}>
//                 // <p>{self.state.line_series.series.len()}</p>
//                 // {self.state.line_series.series.iter().fold("".to_owned(), |mut accumulator, e| {
//                 //     accumulator.push_str(format!("{}: {}, ", e.name, e.data_points.len()).as_ref());
//                 //     accumulator
//                 // })}
//                 <ContextProvider<Rc<GraphState>> context={self.state.clone()}>
//                     {for ctx.props().children.iter()}
//                 </ContextProvider<Rc<GraphState>>>
//             </div>
//         )
//     }

//     fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
//         if !first_render {
//             return
//         }

        // if let Some(element) = self.node_ref.cast::<HtmlElement>() {
        //     let on_new_data_available = ctx.link().callback(|e: Event| {
        //         let casted_event = e.dyn_ref::<CustomEvent>().unwrap_throw();
        //         let payload = casted_event.detail();

        //         let file_name = payload.as_string();

        //         Self::Message::ParseComplete(file_name.unwrap_throw())
        //     });

        //     let listener = EventListener::new(
        //         &element, 
        //         "solar_parse_complete", 
        //         move |e| on_new_data_available.emit(e.clone())
        //     );

        //     self.parse_listener = Some(listener);
        // }

        // if let Some(element) = self.node_ref.cast::<HtmlElement>() {
        //     let on_new_data = ctx.link().callback(|e: Event| {
        //         let casted_event = e.dyn_ref::<CustomEvent>().unwrap_throw();
        //         let payload = casted_event.detail();
   
        //         let data = serde_json::from_str::<LineSeriesHolder>(payload.as_string().unwrap().as_ref());

        //         Self::Message::NewData(data.unwrap_throw())
        //     });

        //     let listener = EventListener::new(
        //         &element, 
        //         "data_request_complete", 
        //         move |e| on_new_data.emit(e.clone())
        //     );

        //     self.data_listener = Some(listener);
        // }
//     }
// }