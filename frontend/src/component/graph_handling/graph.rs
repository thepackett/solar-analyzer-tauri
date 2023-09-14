pub mod graph_draw;
pub mod graph_draw_utils;
pub mod x_axis_controls;
pub mod y_axis_controls;
pub mod secondary_y_axis_controls;
pub mod time_range_controls;

use std::ops::Range;

use gloo_events::EventListener;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use shared::{graph::{graph_axis::{AxisDataType, LineSeriesHolder}, graph_type::GraphType, graph_state_request::GraphStateRequest}, parse::utils::ParseCompleteReturnValue, solar_data::{cell::AvailableCells, controllers::AvailableControllers}};
use wasm_bindgen::{UnwrapThrowExt, JsCast};
use web_sys::{HtmlElement, CustomEvent};
use yew::prelude::*;

use crate::{bindings::{setup_canvas_events, teardown_canvas_events, resize_canvas, get_theme_data, get_canvas_width, get_canvas_height, self, teardown_graph_date_picker, setup_graph_date_picker}, component::{visual::theme_data::ThemeData, message_handling::simple_message::SimpleMessageProperties, graph_handling::graph::{time_range_controls::TimeRangeSelector, x_axis_controls::{XAxisControlsRequest, XAxisControls}, secondary_y_axis_controls::{SecYAxisControls, SecYAxisControlsRequest}, y_axis_controls::{YAxisControls, YAxisControlsRequest}}}, component_channel::ComponentChannelTx};

use self::time_range_controls::DateRange;


pub struct Graph {
    available_cells: AvailableCells,
    available_controllers: AvailableControllers,
    graph_state: GraphStateRequest,
    line_series: LineSeriesHolder,
    canvas_id: AttrValue,
    // _context_handle: ContextHandle<Rc<GraphState>>,
    previous_mouse_input: Option<MouseInput>,
    previous_x_range: Option<Range<f64>>,
    previous_y_range: Option<Range<f64>>,
    previous_sec_y_range: Option<Range<f64>>,
    markpoints: Vec<(f64, f64)>,
    periodic: Option<i64>,
    parse_complete_listener: Option<EventListener>,
    data_complete_listener: Option<EventListener>,
    pub canvas_node_ref: NodeRef,
    pub draw_listener: Option<EventListener>,
}

//For some reason this macro is fussy about naming conventions if the variables contain underscores.
#[derive(PartialEq, Properties)]
pub struct GraphProperties {
    pub canvas_id: AttrValue,
    pub canvas_container_id: AttrValue,
    pub notification_tx: ComponentChannelTx<SimpleMessageProperties>
}

pub enum GraphMessage {
    // ContextChanged(Rc<GraphState>),
    DrawGraph,
    MouseClick(MouseInput),
    MouseWheel(MouseInput),
    MouseMovement(MouseInput),
    MouseExit,
    ParseComplete(ParseCompleteReturnValue),
    NewData(LineSeriesHolder),
    NewDateRange(DateRange),
}


pub struct MouseInput {
    local_x: f64,
    local_y: f64,
    movement_x: f64,
    movement_y: f64,
    scroll_delta_y: f64,
    left_click: bool,
    _right_click: bool,
    control_held: bool,
    meta_held: bool,
    shift_held: bool,
}

impl Component for Graph {
    type Message = GraphMessage;
    type Properties = GraphProperties;

    fn create(ctx: &Context<Self>) -> Self {
        setup_canvas_events(ctx.props().canvas_id.to_string(), ctx.props().canvas_container_id.to_string());

        // let (graph_state, _context_handle) = 
        //     ctx.link().context::<Rc<GraphState>>(ctx.link().callback(Self::Message::ContextChanged))
        //     .expect("GraphState context must be set for Graph to function.");
        let canvas_id = ctx.props().canvas_id.clone();
        
        Self {
            available_cells: AvailableCells::default(),
            available_controllers: AvailableControllers::default(),
            graph_state: GraphStateRequest::default_with_name(canvas_id.to_string()),
            line_series: LineSeriesHolder::default(),
            canvas_id: canvas_id,
            // _context_handle: _context_handle,
            canvas_node_ref: NodeRef::default(),
            draw_listener: None,
            previous_mouse_input: None,
            previous_x_range: None,
            previous_y_range: None,
            previous_sec_y_range: None,
            markpoints: Vec::new(),
            periodic: None,
            parse_complete_listener: None,
            data_complete_listener: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GraphMessage::DrawGraph => {
                let theme_data = if let Ok(theme_data) = get_theme_data() {
                    theme_data
                } else {
                    return true
                };
                let result = self.draw_graph(theme_data);
                if let Err(e) = result {
                    let error = wasm_bindgen::JsValue::from_str(e.to_string().as_str());
                    web_sys::console::error_1(&error);
                }
            },
            GraphMessage::MouseClick(mouse_input) => {
                // let message = wasm_bindgen::JsValue::from_str(format!("Recieved click event with x={} and y={}.", mouse_input.local_x, mouse_input.local_y).as_str());
                // web_sys::console::info_1(&message);
                let point = self.convert_local_x_y_to_graph_x_y(mouse_input.local_x, mouse_input.local_y);
                // let message = wasm_bindgen::JsValue::from_str(format!("Converted x={:?} and y={:?}", point.0, point.1).as_str());
                // web_sys::console::info_1(&message);
                if let Some(previous_input) = &self.previous_mouse_input {
                    if !previous_input.left_click {
                        if let (Some(graph_x), Some(graph_y)) = point {
                            if let Some(range_x) = self.previous_x_range.clone() {
                                if let Some(range_y) = self.previous_y_range.clone() {
                                    if range_x.contains(&graph_x) && range_y.contains(&graph_y) {
                                        let minimum_difference = (range_x.end - range_x.start) * 0.0025f64; //Markpoints must be more than 0.25% the "viewport" apart
                                        let new_markpoints = match self.get_graph_type() {
                                            GraphType::XAxisLine => {
                                                //If we're in XAxis Line mode, then markpoints are vertical lines.
                                                self.markpoints.iter().cloned().filter(|markpoint| {
                                                    (graph_x - markpoint.0).abs() > minimum_difference
                                                }).collect::<Vec<_>>()
                                            },
                                            GraphType::XYScatter => {
                                                //If we're in XYScatter mode, then markpoints are points.
                                                self.markpoints.iter().cloned().filter(|markpoint| {
                                                    (graph_x - markpoint.0).powi(2) + (graph_y - markpoint.1).powi(2) > minimum_difference.powi(2)
                                                }).collect::<Vec<_>>()
                                            },
                                        };

                                        if new_markpoints.len() < self.markpoints.len() {
                                            self.markpoints = new_markpoints
                                        } else {
                                            self.markpoints.push((graph_x, graph_y))
                                        }
                                    }
                                }
                            }
                        }
                    } 
                }
                // let message = wasm_bindgen::JsValue::from_str(format!("Final Markpoints: {:?}", self.markpoints).as_str());
                // web_sys::console::info_1(&message);
                self.previous_mouse_input = Some(mouse_input);
            },
            GraphMessage::MouseWheel(mouse_input) => {
                if let (Some(x), Some(y)) = self.convert_local_x_y_to_graph_x_y(mouse_input.local_x, mouse_input.local_y){
                    if let (Some(previous_x_range), Some(previous_y_range)) = (&mut self.previous_x_range, &mut self.previous_y_range) {
                        let scroll_amount = mouse_input.scroll_delta_y.clamp(-100f64, 100f64);
                        if (previous_x_range.contains(&x) || mouse_input.control_held || mouse_input.meta_held) && !mouse_input.shift_held {
                            let x_ratio = (x - previous_x_range.start)/(previous_x_range.end - previous_x_range.start);
                            let x_range_difference = previous_x_range.end - previous_x_range.start;
                            let new_x_start = previous_x_range.start - x_range_difference * x_ratio * (scroll_amount/1000f64);
                            let new_x_end = previous_x_range.end + x_range_difference * (1f64 - x_ratio)*(scroll_amount/1000f64);
                            previous_x_range.start = new_x_start;
                            previous_x_range.end = new_x_end;
                        }
                        if (previous_y_range.contains(&y) || mouse_input.shift_held) && !(mouse_input.control_held || mouse_input.meta_held) {
                            let y_ratio = (y - previous_y_range.start)/(previous_y_range.end - previous_y_range.start);
                            let y_range_difference = previous_y_range.end - previous_y_range.start;
                            let new_y_start = previous_y_range.start - y_range_difference * y_ratio * (scroll_amount/1000f64);
                            let new_y_end = previous_y_range.end + y_range_difference * (1f64 - y_ratio)*(scroll_amount/1000f64);
                            previous_y_range.start = new_y_start;
                            previous_y_range.end = new_y_end;
                        }
                    }
                }
                self.previous_mouse_input = Some(mouse_input);
            },
            GraphMessage::MouseMovement(mouse_input) => {
                if let Some(previous_input) = &self.previous_mouse_input {
                    if previous_input.left_click {
                        if let (Some(previous_x), Some(previous_y)) = self.convert_local_x_y_to_graph_x_y(previous_input.local_x, previous_input.local_y) {
                            if let (Some(current_x), Some(current_y)) = self.convert_local_x_y_to_graph_x_y(mouse_input.local_x, mouse_input.local_y) {
                                if let (Some(previous_x_range), Some(previous_y_range)) = (&mut self.previous_x_range, &mut self.previous_y_range) {
                                    if (previous_x_range.contains(&current_x) || mouse_input.control_held || mouse_input.meta_held) && !mouse_input.shift_held {
                                        previous_x_range.start -= current_x - previous_x;
                                        previous_x_range.end -= current_x - previous_x;
                                    }
                                    if (previous_y_range.contains(&current_y) || mouse_input.shift_held) && !(mouse_input.control_held || mouse_input.meta_held) {
                                        previous_y_range.start -= current_y - previous_y;
                                        previous_y_range.end -= current_y - previous_y;
                                    }
                                }
                            }
                        }

                        // let info = wasm_bindgen::JsValue::from_str(format!("Movement X: {}\nMovement Y: {}", mouse_input.movement_x, mouse_input.movement_y).as_str());
                        // web_sys::console::info_1(&info);
                    }
                }
                self.previous_mouse_input = Some(mouse_input);
            },
            GraphMessage::MouseExit => {
                self.previous_mouse_input = None;
            },
            GraphMessage::ParseComplete(payload) => {
                self.available_cells = payload.cell_ids;
                self.available_controllers = payload.controller_ids;
                let message = SimpleMessageProperties { 
                    class: AttrValue::from("notification"), 
                    message:  AttrValue::from(format!("{} parsing complete.", payload.name)),
                };
                if let Err(e) = ctx.props().notification_tx.try_send(message) {
                    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(format!("{:?}", e).as_str()));
                }
                // self.app_state.notification_callback.clone().expect("Notification callback must be set").emit(message);
                bindings::retrieve_solar_data(serde_json::to_string(&self.graph_state).unwrap())
            },
            GraphMessage::NewData(data) => {
                self.line_series = data;
                self.previous_x_range = None;
                self.previous_y_range = None;
                self.previous_sec_y_range = None;
            },
            GraphMessage::NewDateRange(date_range) => {
                todo!()
            },
            
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|event: MouseEvent| {
            Self::Message::MouseClick(
                MouseInput { 
                    local_x: event.offset_x() as f64, //This only works so long as the canvas element has no padding/margins/border 
                    local_y: event.offset_y() as f64, 
                    movement_x: event.movement_x() as f64, 
                    movement_y: event.movement_y() as f64, 
                    scroll_delta_y: 0f64, 
                    left_click: event.buttons() & 1 == 1, 
                    _right_click: event.buttons() & 2 == 2, 
                    control_held: event.ctrl_key(), 
                    meta_held: event.meta_key(), 
                    shift_held: event.shift_key() 
                }
            )
        });

        let onexit = ctx.link().callback(|_event| {
            Self::Message::MouseExit
        });

        let onscroll = ctx.link().callback(|event: WheelEvent| {
            Self::Message::MouseWheel(
                MouseInput { 
                    local_x: event.offset_x() as f64,
                    local_y: event.offset_y() as f64, 
                    movement_x: event.movement_x() as f64, 
                    movement_y: event.movement_y() as f64, 
                    scroll_delta_y: event.delta_y(), 
                    left_click: event.buttons() & 1 == 1, 
                    _right_click: event.buttons() & 2 == 2, 
                    control_held: event.ctrl_key(), 
                    meta_held: event.meta_key(), 
                    shift_held: event.shift_key()
                }
            )
        });

        let onmove = ctx.link().callback(|event: MouseEvent| {
            Self::Message::MouseMovement(
                MouseInput { 
                    local_x: event.offset_x() as f64, //This only works so long as the canvas element has no padding/margins/border 
                    local_y: event.offset_y() as f64, 
                    movement_x: event.movement_x() as f64, 
                    movement_y: event.movement_y() as f64, 
                    scroll_delta_y: 0f64, 
                    left_click: event.buttons() & 1 == 1, 
                    _right_click: event.buttons() & 2 == 2, 
                    control_held: event.ctrl_key(), 
                    meta_held: event.meta_key(), 
                    shift_held: event.shift_key() 
                }
            )
        });

        let onnewdaterange = ctx.link().callback(|date_range: DateRange| {
            Self::Message::NewDateRange(date_range)
        });

        let onnewxaxisrequest = ctx.link().callback(|x_axis_controls_request: XAxisControlsRequest| {
            GraphMessage::DrawGraph //Temporary until I implement more details
        });

        let onnewyaxisrequest = ctx.link().callback(|y_axis_controls_request: YAxisControlsRequest| {
            GraphMessage::DrawGraph //Temporary until I implement more details
        });

        let onnewsecyaxisrequest = ctx.link().callback(|sec_y_axis_controls_request: SecYAxisControlsRequest| {
            GraphMessage::DrawGraph //Temporary until I implement more details
        });

        html!(
            <div class="graph">
                <div id={ctx.props().canvas_container_id.to_string()}>
                    <canvas id={ctx.props().canvas_id.to_string()} ref={self.canvas_node_ref.clone()} onclick={onclick} onmouseleave={onexit} onwheel={onscroll} onmousemove={onmove}  width="1000" height="1000" class="graph-canvas">
                    </canvas>
                </div>
                <div class="graph-controls">
                    <TimeRangeSelector id={format!("{}_litepicker", self.canvas_id)} callback={onnewdaterange}/>
                    <XAxisControls callback={onnewxaxisrequest} available_cells={self.available_cells.clone()} available_controllers={self.available_controllers.clone()} />
                    <YAxisControls callback={onnewyaxisrequest} />
                    <SecYAxisControls callback={onnewsecyaxisrequest} />
                </div>
            </div>
        )
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return
        }
        
        let props = ctx.props();

        if let Some(element) = self.canvas_node_ref.cast::<HtmlElement>() {
            let ondraw = ctx.link().callback(|_e: Event| {
                Self::Message::DrawGraph
            });

            let listener = EventListener::new(
                &element, 
                format!("draw_{}", ctx.props().canvas_id), 
                move |e| ondraw.emit(e.clone())
            );
            
            self.draw_listener = Some(listener);
        }

        resize_canvas(props.canvas_id.to_string(), props.canvas_container_id.to_string());

        let root = bindings::get_root().expect("We should always be able to get the root element");

        let on_new_data_available = ctx.link().callback(|e: Event| {
            // web_sys::console::info_1(&wasm_bindgen::JsValue::from_str("on_new_data_available callback called"));
            let casted_event = e.dyn_ref::<CustomEvent>().unwrap_throw();
            let payload = casted_event.detail();

            let payload = payload.as_string().unwrap_throw();

            let payload = serde_json::from_str::<ParseCompleteReturnValue>(&payload);

            Self::Message::ParseComplete(payload.unwrap_throw())
        });

        let parse_listener = EventListener::new(
            &root, 
            "solar_parse_complete", 
            move |e| on_new_data_available.emit(e.clone())
        );
        
        self.parse_complete_listener = Some(parse_listener);

        let on_new_data = ctx.link().callback(|e: Event| {
            web_sys::console::info_1(&wasm_bindgen::JsValue::from_str("on_new_data callback called"));
            let casted_event = e.dyn_ref::<CustomEvent>().unwrap_throw();
            let payload = casted_event.detail();

            let data = serde_json::from_str::<LineSeriesHolder>(payload.as_string().unwrap().as_ref());

            Self::Message::NewData(data.unwrap_throw())
        });

        let data_listener = EventListener::new(
            &root, 
            format!("data_request_complete_{}", ctx.props().canvas_id), 
            move |e| on_new_data.emit(e.clone())
        );

        self.data_complete_listener = Some(data_listener);
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        teardown_canvas_events(ctx.props().canvas_id.to_string());
    }
}
