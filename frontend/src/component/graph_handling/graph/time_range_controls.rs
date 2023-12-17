use std::rc::Rc;

use gloo_events::EventListener;
use shared::graph::{graph_axis::AxisTimeRequest, graph_state_request::Resolution};
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use web_sys::{CustomEvent, HtmlSelectElement};
use yew::prelude::*;

use crate::{bindings, component::control::copy_paste::{CopyPaste, Request}};

use super::graph_coordination::SharableGraphData;

pub struct TimeRangeSelector {
    input_node_ref: NodeRef,
    input_listener: Option<EventListener>,
    select_node_ref: NodeRef,
    copy_state: Option<(Rc<Option<SharableGraphData>>, Callback<Rc<Option<SharableGraphData>>>)>,
    _context_handle: Option<ContextHandle<(Rc<Option<SharableGraphData>>, Callback<Rc<Option<SharableGraphData>>>)>>,
}

#[derive(PartialEq, Properties)]
pub struct TimeRangeSelectorProps {
    pub id: AttrValue,
    pub callback: Callback<AxisTimeRequest>,
    pub current_date_range: AxisTimeRequest,
}

pub enum TimeRangeSelectorMessage {
    NewTimeFrame(i64,i64),
    NewResolution(Option<Resolution>),
    ContextChanged((Rc<Option<SharableGraphData>>, Callback<Rc<Option<SharableGraphData>>>)),
    Copy,
    Paste,
}

impl Component for TimeRangeSelector {
    type Message = TimeRangeSelectorMessage;
    type Properties = TimeRangeSelectorProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (copy_state, _context_handle) = 
        match ctx.link().context::<(Rc<Option<SharableGraphData>>, Callback<Rc<Option<SharableGraphData>>>)>(ctx.link().callback(Self::Message::ContextChanged)) {
            Some((state, handle)) => (Some(state), Some(handle)),
            None => (None, None),
        };

        TimeRangeSelector {
            input_node_ref: NodeRef::default(),
            input_listener: None,
            select_node_ref: NodeRef::default(),
            copy_state,
            _context_handle,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            Self::Message::NewTimeFrame(start, end) => {
                props.callback.emit(AxisTimeRequest { 
                    start, 
                    end, 
                    manual_resolution: props.current_date_range.manual_resolution.clone(), 
                });
            },
            Self::Message::NewResolution(manual_resolution) => {
                props.callback.emit(AxisTimeRequest { 
                    start: props.current_date_range.start, 
                    end: props.current_date_range.end, 
                    manual_resolution,
                });
            },
            Self::Message::ContextChanged(new_copy_state) => {
                self.copy_state = Some(new_copy_state);
                return true;
            },
            Self::Message::Copy => {
                if let Some((_data, update_shared_data)) = &self.copy_state {
                    update_shared_data.emit(Rc::from(Some(SharableGraphData::TimeData(props.current_date_range.clone()))));
                }
                return true;
            },
            Self::Message::Paste => {
                if let Some((data, _update_shared_data)) = &self.copy_state {
                    if let Some(SharableGraphData::TimeData(saved_time_request)) = data.as_ref() {
                        props.callback.emit(saved_time_request.clone());
                    }
                }
            },
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let reference = self.select_node_ref.clone();
        let onresolutionchange = ctx.link().callback(move |_event: Event| {
            let element = reference.cast::<HtmlSelectElement>().unwrap_throw();
            // web_sys::console::info_1(&wasm_bindgen::JsValue::from_str(format!("{}", element.value()).as_str()));
            let resolution = match element.value().as_ref() {
                "auto" => None,
                "1_minute" => Some(Resolution::OneMinute),
                "5_minutes" => Some(Resolution::FiveMinute),
                "15_minutes" => Some(Resolution::FifteenMinute),
                "1_hour" => Some(Resolution::OneHour),
                "1_day" => Some(Resolution::OneDay),
                _ => None,
            };
            Self::Message::NewResolution(resolution)
        });
        let oncopypaste = ctx.link().callback(|msg| {
            match msg {
                Request::Copy => Self::Message::Copy,
                Request::Paste => Self::Message::Paste,
            }
        });
        let paste_visible = match &self.copy_state {
            Some((data, _update_callback)) => {
                if let Some(SharableGraphData::TimeData(_)) = data.as_ref() {
                    true
                } else {
                    false
                }
            },
            None => false,
        };

        html!(
            <div>
                <p>{"Date range goes here"}</p>
                <input class="graph-text-input" ref={self.input_node_ref.clone()} type={"text"} id={props.id.clone()}/>
                //Include resolution selector that defaults to auto?
                <select class="graph-dropdown" onchange={onresolutionchange} ref={self.select_node_ref.clone()}>
                    <option value={"auto"} selected=true>{"Auto"}</option>
                    <option value={"1_minute"}>{"1 Minute"}</option>
                    <option value={"5_minutes"}>{"5 Minutes"}</option>
                    <option value={"15_minutes"}>{"15 Minutes"}</option>
                    <option value={"1_hour"}>{"1 Hour"}</option>
                    <option value={"1_day"}>{"1 Day"}</option>
                </select>
                <CopyPaste copy_visible={true} paste_visible={paste_visible} callback={oncopypaste}/>
            </div>
        )
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let props = ctx.props();
        let current_date_range = &props.current_date_range;
        if !first_render {
            bindings::teardown_graph_date_picker(ctx.props().id.to_string());
            bindings::setup_graph_date_picker(props.id.to_string(), current_date_range.start, current_date_range.end);
            return
        }
        
        bindings::setup_graph_date_picker(props.id.to_string(), current_date_range.start, current_date_range.end);

        let on_selected = ctx.link().callback(|e: Event| {
            // web_sys::console::info_1(&wasm_bindgen::JsValue::from_str("on_selected callback called"));
            let casted_event = e.dyn_ref::<CustomEvent>().unwrap_throw();
            let payload = casted_event.detail().as_string().unwrap();
            let mut date_strings = payload.split(":");
            let start_time: i64 = date_strings.next().unwrap_throw().parse().unwrap_throw();
            let end_time: i64 = date_strings.next().unwrap_throw().parse().unwrap_throw();

            

            Self::Message::NewTimeFrame(start_time, end_time)
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
        bindings::teardown_graph_date_picker(ctx.props().id.to_string());
    }
}