use gloo_events::EventListener;
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use web_sys::CustomEvent;
use yew::prelude::*;

use crate::bindings::{teardown_graph_date_picker, setup_graph_date_picker, self};

pub struct TimeRangeSelector {
    input_node_ref: NodeRef,
    input_listener: Option<EventListener>,
}

#[derive(PartialEq, Properties)]
pub struct TimeRangeSelectorProps {
    pub id: AttrValue,
    pub callback: Callback<DateRange>,
    pub current_date_range: DateRange,
}

pub enum TimeRangeSelectorMessage {
    NewDateRange(DateRange)
}

impl Component for TimeRangeSelector {
    type Message = TimeRangeSelectorMessage;
    type Properties = TimeRangeSelectorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        TimeRangeSelector {
            input_node_ref: NodeRef::default(),
            input_listener: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimeRangeSelectorMessage::NewDateRange(date) => {
                ctx.props().callback.emit(date);
            },
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        html!(
            <div>
                <p>{"Date range goes here"}</p>
                <input ref={self.input_node_ref.clone()} type={"text"} id={props.id.clone()}/>
                //Include resolution selector that defaults to auto?
            </div>
        )
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return
        }
        let props = ctx.props();
        let current_date_range = &props.current_date_range;
        setup_graph_date_picker(props.id.to_string(), current_date_range.start, current_date_range.end);

        let on_selected = ctx.link().callback(|e: Event| {
            // web_sys::console::info_1(&wasm_bindgen::JsValue::from_str("on_selected callback called"));
            let casted_event = e.dyn_ref::<CustomEvent>().unwrap_throw();
            let payload = casted_event.detail().as_string().unwrap();
            let mut date_strings = payload.split(":");
            let start_time: i64 = date_strings.next().unwrap_throw().parse().unwrap_throw();
            let end_time: i64 = date_strings.next().unwrap_throw().parse().unwrap_throw();

            Self::Message::NewDateRange(DateRange { start: start_time, end: end_time })
        });

        let root = bindings::get_root().expect("We should always be able to get the root element");
        let input_listener = EventListener::new(
            &root, 
            format!("{}_selected", props.id), 
            move |e| on_selected.emit(e.clone())
        );

        self.input_listener = Some(input_listener);
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        teardown_graph_date_picker(ctx.props().id.to_string());
    }
}


#[derive(PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DateRange {
    pub start: i64,
    pub end: i64,
}