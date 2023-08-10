use std::{rc::Rc, ops::Range};

use gloo_events::EventListener;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use shared::graph::graph_axis::AxisDataType;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{bindings::{setup_canvas_events, teardown_canvas_events, resize_canvas, get_theme_data, get_canvas_width, get_canvas_height}, component::visual::theme_data::ThemeData};

use super::graph_state::GraphState;

const CHART_MARGIN_SIZE: u32 = 20;
const CHART_LABEL_SIZE: u32 = 30;

pub struct Graph {
    graph_state: Rc<GraphState>,
    canvas_id: AttrValue,
    _context_handle: ContextHandle<Rc<GraphState>>,
    previous_mouse_input: Option<MouseInput>,
    previous_x_range: Option<Range<f64>>,
    previous_y_range: Option<Range<f64>>,
    previous_sec_y_range: Option<Range<f64>>,
    markpoints: Vec<f64>,
    periodic: Option<i64>,
    pub canvas_node_ref: NodeRef,
    pub draw_listener: Option<EventListener>,
}


#[derive(Properties, PartialEq)]
pub struct GraphProperties {
    pub canvas_id: AttrValue,
    pub canvas_container_id: AttrValue,
}

pub enum GraphMessage {
    ContextChanged(Rc<GraphState>),
    DrawGraph,
    MouseClick(MouseInput),
    MouseClickDown(MouseInput),
    MouseWheel(MouseInput),
    MouseMovement(MouseInput),
    MouseExit,
}


pub struct MouseInput {
    local_x: f64,
    local_y: f64,
    movement_x: f64,
    movement_y: f64,
    scroll_delta_x: f64,
    left_click: bool,
    right_click: bool,
    control_held: bool,
    meta_held: bool,
    alt_held: bool,
}

impl Graph {
    fn convert_local_x_y_to_graph_x_y(&self, x: f64, y: f64) -> (Option<f64>, Option<f64>) {
        let x = if x.is_finite() {
            match self.previous_x_range.clone() {
                Some(x_range) => {
                    Some(
                        (x - CHART_MARGIN_SIZE as f64 - CHART_LABEL_SIZE as f64) 
                        * ((x_range.end - x_range.start) / ((get_canvas_width(self.canvas_id.to_string()) as f64) - (CHART_MARGIN_SIZE * 2 + CHART_LABEL_SIZE) as f64))
                        + x_range.start
                    )
                },
                None => None,
            }
        } else { None };
        let y = if y.is_finite() {
            match self.previous_y_range.clone() {
                Some(y_range) => {
                    Some(
                        y_range.end - ((y - CHART_MARGIN_SIZE as f64 - CHART_LABEL_SIZE as f64) 
                        * ((y_range.end - y_range.start) / ((get_canvas_height(self.canvas_id.to_string()) as f64) - (CHART_MARGIN_SIZE * 2 + CHART_LABEL_SIZE * 2) as f64))
                        + y_range.start)
                    )
                },
                None => None,
            }
        } else { None };
        (x,y)
    }

    pub fn draw_graph(&mut self, theme: ThemeData) -> Result<(), Box<dyn std::error::Error>>  {
        //Thinking about the broader picture, I really need to divide behavior based on graph type.
        //Currently 2 graph types are planned.
        //1. X axis time line series
        //2. X-Y scatter plot
        //Most behavior can be shared between these plot types (graph mesh / captions, data ranges, styles, zoom controls, etc)
        //But some behavior can't. For example, X-Y scatters must be drawn without a line. "Vertical line" markpoints only make sense for the x axis time line series.



        let canvas_id = self.canvas_id.as_str();
        let graph_state = self.graph_state.clone();
        let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
        let root = backend.into_drawing_area();
        let caption_font = FontDesc::new(FontFamily::from("sans-serif"), 20.0, FontStyle::Normal);
        let label_font = FontDesc::new(FontFamily::from("sans-serif"), 12.0, FontStyle::Normal);
    
        root.fill(&RGBColor::from(&theme.theme_graph_background))?;
    
        let x_axis_range = if let Some(x_range) = self.previous_x_range.clone() {
            x_range
        } else { 
            let x_range = graph_state.line_series.series.iter().map(|series| {
            let max_point = series.data_points.iter().reduce(|accumulator, e| {
                if accumulator.0 < e.0 {
                    e
                } else {
                    accumulator
                }
            }).unwrap_or(&(0f64,0f64));
            let min_point = series.data_points.iter().reduce(|accumulator, e| {
                if accumulator.0 > e.0 {
                    e
                } else {
                    accumulator
                }
            }).unwrap_or(&(0f64,0f64));
    
            min_point.0 .. max_point.0
            }).reduce(|accumulator, series_range| {
                let mut range = accumulator.clone();
                if range.start > series_range.start {
                    range.start = series_range.start;
                }
                if range.end < series_range.end {
                    range.end = series_range.end;
                }
                range
            }).unwrap_or(0f64..0f64);
            self.previous_x_range = Some(x_range.clone());
            x_range
        };
    
        let y_axis_range = if let Some(y_range) = self.previous_y_range.clone() {
            y_range
        } else { 
            let y_range = graph_state.line_series.series.iter().map(|series| {
                let max_point = series.data_points.iter().reduce(|accumulator, e| {
                    if accumulator.1 < e.1 {
                        e
                    } else {
                        accumulator
                    }
                }).unwrap_or(&(0f64,0f64));
                let min_point = series.data_points.iter().reduce(|accumulator, e| {
                    if accumulator.1 > e.1 {
                        e
                    } else {
                        accumulator
                    }
                }).unwrap_or(&(0f64,0f64));
        
                min_point.1 .. max_point.1
            }).reduce(|accumulator, series_range| {
                let mut range = accumulator.clone();
                if range.start > series_range.start {
                    range.start = series_range.start;
                }
                if range.end < series_range.end {
                    range.end = series_range.end;
                }
                range
            }).unwrap_or(0f64..0f64);
            self.previous_y_range = Some(y_range.clone());
            y_range
        };
    
        let secondary_y_axis_range = if let Some(sec_y_range) = self.previous_sec_y_range.clone() {
            sec_y_range
        } else { 
            let sec_y_range = graph_state.line_series.secondary_series.iter().map(|series| {
                let max_point = series.data_points.iter().reduce(|accumulator, e| {
                    if accumulator.1 < e.1 {
                        e
                    } else {
                        accumulator
                    }
                }).unwrap_or(&(0f64,0f64));
                let min_point = series.data_points.iter().reduce(|accumulator, e| {
                    if accumulator.1 > e.1 {
                        e
                    } else {
                        accumulator
                    }
                }).unwrap_or(&(0f64,0f64));
        
                min_point.1 .. max_point.1
            }).reduce(|accumulator, series_range| {
                let mut range = accumulator.clone();
                if range.start > series_range.start {
                    range.start = series_range.start;
                }
                if range.end < series_range.end {
                    range.end = series_range.end;
                }
                range
            }).unwrap_or(0f64..0f64);
            self.previous_sec_y_range = Some(sec_y_range.clone());
            sec_y_range
        };
    
        let mut chart = ChartBuilder::on(&root)
            .margin(CHART_MARGIN_SIZE)
            .caption(format!("temp caption"), caption_font.clone().with_color(RGBColor::from(&theme.theme_text)))
            .x_label_area_size(CHART_LABEL_SIZE)
            .y_label_area_size(CHART_LABEL_SIZE)
            .build_cartesian_2d(x_axis_range.clone(), y_axis_range.clone())?;
        

        self.markpoints.iter().for_each(|markpoint| {
            let _result = chart.draw_series(
                LineSeries::new([(markpoint.clone(), y_axis_range.start.clone()), (markpoint.clone(), y_axis_range.end.clone())], &BLACK)
            );
        });
    
        graph_state.line_series.series.iter().enumerate().for_each(|series| {
            let name = series.1.name.clone();
            match chart.draw_series(LineSeries::new(series.1.data_points.clone(), Palette99::pick(series.0))) {
                Ok(line_series) => {
                    //Configure labels and legend here
                    line_series
                        .label(name)
                        .legend(move |(x,y)| {PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(series.0))});
                },
                Err(_) => {},
            }
        }); 

        let x_axis_formatter = match self.graph_state.details.x_axis.first() {
            Some(axis_data) => {
                match axis_data.data_type {
                    AxisDataType::Time => {
                        time_axis_label_formatter
                    },
                    _ => {
                        other_axis_label_formatter
                    }
                }
            },
            None => other_axis_label_formatter,
        };
        let y_axis_formatter = match self.graph_state.details.y_axis.0.first() {
            Some(axis_data) => {
                match axis_data.data_type {
                    AxisDataType::Time => {
                        time_axis_label_formatter
                    },
                    _ => {
                        other_axis_label_formatter
                    }
                }
            },
            None => other_axis_label_formatter,
        };
        let secondary_y_axis_formatter = match self.graph_state.details.y_axis.1.first() {
            Some(axis_data) => {
                match axis_data.data_type {
                    AxisDataType::Time => {
                        time_axis_label_formatter
                    },
                    _ => {
                        other_axis_label_formatter
                    }
                }
            },
            None => other_axis_label_formatter,
        };

    
        chart.configure_mesh()
            .light_line_style(&RGBColor::from(&theme.theme_graph_mesh_light))
            .bold_line_style(&RGBColor::from(&theme.theme_graph_mesh_dark))
            .axis_style(&RGBColor::from(&theme.theme_graph_border))
            .x_desc("X axis description")
            .x_label_style(label_font.clone())
            .x_labels(3)
            .x_label_formatter(&x_axis_formatter)
            .y_desc("Y Axis description")
            .y_label_style(label_font.clone())
            .y_labels(3)
            .y_label_formatter(&y_axis_formatter)
            .label_style(&RGBColor::from(&theme.theme_text))
            .draw()?;
        chart.configure_series_labels()
            .label_font(label_font.clone().color(&RGBColor::from(&theme.theme_text)))
            .background_style(&RGBColor::from(&theme.theme_graph_background))
            .border_style(&RGBColor::from(&theme.theme_graph_border))
            .draw()?;

    
            //chart.set_secondary_coord(x_axis_range, secondary_y_axis_range).configure_secondary_axes();
    
        root.present()?;
        Ok(())
    }
}

impl Component for Graph {
    type Message = GraphMessage;
    type Properties = GraphProperties;

    fn create(ctx: &Context<Self>) -> Self {
        setup_canvas_events(ctx.props().canvas_id.to_string(), ctx.props().canvas_container_id.to_string());

        let (graph_state, _context_handle) = 
            ctx.link().context::<Rc<GraphState>>(ctx.link().callback(Self::Message::ContextChanged))
            .expect("GraphState context must be set for Graph to function.");

        Self {
            graph_state: graph_state,
            canvas_id: ctx.props().canvas_id.clone(),
            _context_handle: _context_handle,
            canvas_node_ref: NodeRef::default(),
            draw_listener: None,
            previous_mouse_input: None,
            previous_x_range: None,
            previous_y_range: None,
            previous_sec_y_range: None,
            markpoints: Vec::new(),
            periodic: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
            GraphMessage::ContextChanged(graph_state) => {
                self.graph_state = graph_state;
                self.previous_x_range = None;
                self.previous_y_range = None;
                self.previous_sec_y_range = None;
            },
            GraphMessage::MouseClick(mouse_input) => {
                let message = wasm_bindgen::JsValue::from_str(format!("Recieved click event with x={} and y={}.", mouse_input.local_x, mouse_input.local_y).as_str());
                web_sys::console::info_1(&message);
                let point = self.convert_local_x_y_to_graph_x_y(mouse_input.local_x, mouse_input.local_y);
                let message = wasm_bindgen::JsValue::from_str(format!("Converted x={:?} and y={:?}", point.0, point.1).as_str());
                web_sys::console::info_1(&message);
                if let (Some(graph_x), Some(graph_y)) = point {
                    if let Some(range_x) = self.previous_x_range.clone() {
                        if let Some(range_y) = self.previous_y_range.clone() {
                            if range_x.contains(&graph_x) && range_y.contains(&graph_y) {
                                let minimum_difference = (range_x.end - range_x.start) * 0.0025f64; //Markpoints must be more than 0.25% the "viewport" apart
                                let new_markpoints = self.markpoints.iter().cloned().filter(|markpoint| {
                                    (graph_x - markpoint).abs() > minimum_difference
                                }).collect::<Vec<_>>();
                                if new_markpoints.len() < self.markpoints.len() {
                                    self.markpoints = new_markpoints
                                } else {
                                    self.markpoints.push(graph_x)
                                }
                            }
                        }
                    }
                } 
                let message = wasm_bindgen::JsValue::from_str(format!("Final Markpoints: {:?}", self.markpoints).as_str());
                web_sys::console::info_1(&message);
                self.previous_mouse_input = Some(mouse_input);
            },
            GraphMessage::MouseClickDown(mouse_input) => {
                self.previous_mouse_input = Some(mouse_input);
            },
            GraphMessage::MouseWheel(mouse_input) => todo!(),
            GraphMessage::MouseMovement(mouse_input) => todo!(),
            GraphMessage::MouseExit => {
                self.previous_mouse_input = None;
            },
            
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|event: MouseEvent| {
            Self::Message::MouseClick(
                MouseInput { 
                    local_x: event.client_x() as f64, //This only works so long as the canvas element has no padding/margins/border 
                    local_y: event.client_y() as f64, 
                    movement_x: event.movement_x() as f64, 
                    movement_y: event.movement_y() as f64, 
                    scroll_delta_x: 0f64, 
                    left_click: event.buttons() & 1 == 1, 
                    right_click: event.buttons() & 2 == 2, 
                    control_held: event.ctrl_key(), 
                    meta_held: event.meta_key(), 
                    alt_held: event.alt_key() 
                }
            )
        });

        let onexit = ctx.link().callback(|_event| {
            Self::Message::MouseExit
        });

        html!(
            <div class="graph">
                // <div class="graph-y-axis">
                //     <p>{"y-axis label"}</p>
                //     <button class="graph-y-axis graph-y-axis-button" type="button">{"y-axis-button"}</button>
                // </div>
                <div id={ctx.props().canvas_container_id.to_string()}>
                    <canvas id={ctx.props().canvas_id.to_string()} ref={self.canvas_node_ref.clone()} onclick={onclick} onmouseleave={onexit}  width="1000" height="1000" class="graph-canvas">
                    </canvas>
                </div>
                // <div class="graph-timescale">
                //     <p>{"timescale"}</p>
                //     <button class="graph-timescale-button" type="button">{"timescale-button"}</button>
                // </div>
                // <div class="graph-x-axis">
                //     <p>{"x-axis label"}</p>
                //     <button class="graph-x-axis-button" type="button">{"x-axis-button"}</button>
                // </div>
                <div class="graph-controls">
                    <p>{"Controls go here"}</p>
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
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        teardown_canvas_events(ctx.props().canvas_id.to_string());
    }
}

fn time_axis_label_formatter(unix_time: &f64) -> String {
    time::OffsetDateTime::from_unix_timestamp(unix_time.clone() as i64)
        .expect("All stored timestamps are valid")
        .format(time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]"))
        .expect("Given format is verified during compilation")
}

fn other_axis_label_formatter(data: &f64) -> String {
    data.to_string()
}