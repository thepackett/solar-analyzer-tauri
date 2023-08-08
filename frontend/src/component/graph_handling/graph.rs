use std::{rc::Rc, ops::Range};

use gloo_events::EventListener;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use shared::graph::graph_axis::{AxisData, AxisZoom};
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::{bindings::{setup_canvas_events, teardown_canvas_events, resize_canvas, get_theme_data}, component::visual::theme_data::ThemeData};

use super::graph_state::GraphState;

pub struct Graph {
    graph_state: Rc<GraphState>,
    _context_handle: ContextHandle<Rc<GraphState>>,
    x_axis_zoom: AxisZoom,
    y_axis_zoom: AxisZoom,
    periodic: Option<i64>,
    previous_x_range: Range<f64>,
    previous_y_range: Range<f64>,
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
            _context_handle: _context_handle,
            canvas_node_ref: NodeRef::default(),
            draw_listener: None,
            x_axis_zoom: AxisZoom::Auto,
            y_axis_zoom: AxisZoom::Auto,
            previous_x_range: 0f64..1f64,
            previous_y_range: 0f64..1f64,
            periodic: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GraphMessage::DrawGraph => {
                let canvas_id = ctx.props().canvas_id.to_string();
                let theme_data = if let Ok(theme_data) = get_theme_data() {
                    theme_data
                } else {
                    return true
                };
                let result = draw_graph(canvas_id.as_ref(), self.graph_state.clone(), theme_data);
                if let Err(e) = result {
                    let error = wasm_bindgen::JsValue::from_str(e.to_string().as_str());
                    web_sys::console::error_1(&error);
                }
            },
            GraphMessage::ContextChanged(graph_state) => {
                self.graph_state = graph_state;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        

        html!(
            <div class="graph">
                <div class="graph-y-axis">
                    <p>{"y-axis label"}</p>
                    <button class="graph-y-axis graph-y-axis-button" type="button">{"y-axis-button"}</button>
                </div>
                <div id={ctx.props().canvas_container_id.to_string()}>
                    <canvas id={ctx.props().canvas_id.to_string()} ref={self.canvas_node_ref.clone()} width="1000" height="1000" class="graph-canvas">
                    </canvas>
                </div>
                <div class="graph-timescale">
                    <p>{"timescale"}</p>
                    <button class="graph-timescale-button" type="button">{"timescale-button"}</button>
                </div>
                <div class="graph-x-axis">
                    <p>{"x-axis label"}</p>
                    <button class="graph-x-axis-button" type="button">{"x-axis-button"}</button>
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


//Graph implementation plans:
//Firstly, the below code correctly draws a graph, so use it as an example.
//Secondly, I need to hook up graph colors to theme colors. Whenever the theme changes, the graph needs to redraw. Update: Graph is being redrawn 30 times per second.
//Thirdly, I need to hook up the graph data into the appstate (best solution for a "global variable" in yew). Whenever the app state updates, the graph should redraw. Update: use graph state.
//TBD: Controls and whatnot are still uncertain. Should I work on those first?
//Also need to include mouse location somewhere

pub fn draw_graph(canvas_id: &str, graph_state: Rc<GraphState>, theme: ThemeData) -> Result<(), Box<dyn std::error::Error>>  {
    let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
    let root = backend.into_drawing_area();
    //let font: FontDesc = ("sans-serif", 20.0).with_color(&RGBColor::from(theme.theme_text)).into();
    let font = FontDesc::new(FontFamily::from("sans-serif"), 20.0, FontStyle::Normal);

    root.fill(&RGBColor::from(theme.theme_graph_background))?;

    let x_axis_range = graph_state.line_series.series.iter().map(|series| {
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



    let y_axis_range = graph_state.line_series.series.iter().map(|series| {
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

    // let y_axis_range = graph_state.line_series.series.iter().fold(0f64..0f64, |accumulator, series| {
    //     let max_point = series.data_points.iter().fold((f64::MIN, f64::MIN), |accumulator, e| {
    //         if accumulator.1 < e.1 {
    //             *e
    //         } else {
    //             accumulator
    //         }
    //     });
    //     let min_point = series.data_points.iter().fold((f64::MAX, f64::MAX), |accumulator, e| {
    //         if accumulator.1 > e.1 {
    //             *e
    //         } else {
    //             accumulator
    //         }
    //     });

    //     let mut range = accumulator.clone();
    //     if range.start > min_point.1 {
    //         range.start = min_point.1;
    //     }
    //     if range.end < max_point.1 {
    //         range.end = max_point.1;
    //     }
    //     range
    // });

    let secondary_y_axis_range = graph_state.line_series.secondary_series.iter().fold(0f64..1f64, |accumulator, series| {
        let max_point = series.data_points.iter().fold((0f64, f64::MIN), |accumulator, e| {
            if accumulator.1 < e.1 {
                *e
            } else {
                accumulator
            }
        });
        let min_point = series.data_points.iter().fold((0f64, f64::MAX), |accumulator, e| {
            if accumulator.1 > e.1 {
                *e
            } else {
                accumulator
            }
        });
        let mut range = accumulator.clone();
        if range.start > min_point.1 {
            range.start = min_point.1;
        }
        if range.end < max_point.1 {
            range.end = min_point.1;
        }
        range
    });

    let mut chart = ChartBuilder::on(&root)
        .margin(20u32)
        .caption(format!("temp caption"), font.clone().with_color(RGBColor::from(theme.theme_text.clone())))
        .x_label_area_size(30u32)
        .y_label_area_size(30u32)
        .build_cartesian_2d(x_axis_range, y_axis_range)?;
    

    graph_state.line_series.series.iter().enumerate().for_each(|series| {
        let _result = chart.draw_series(LineSeries::new(series.1.data_points.clone(), Palette99::pick(series.0)));
    });


        chart.configure_mesh()
            .light_line_style(&RGBColor::from(theme.theme_graph_mesh_light))
            .bold_line_style(&RGBColor::from(theme.theme_graph_mesh_dark))
            .axis_style(&RGBColor::from(theme.theme_graph_border))
            .x_desc("X axis description")
            .x_labels(3)
            .y_desc("Y Axis description")
            .y_labels(3)
            .label_style(&RGBColor::from(theme.theme_text.clone()))
            .draw()?;
        chart.configure_series_labels()
            .label_font(font.clone().color(&RGBColor::from(theme.theme_text)));


        //chart.set_secondary_coord(x_axis_range, secondary_y_axis_range).configure_secondary_axes();


    root.present()?;
    Ok(())
}


// pub fn draw_graph(canvas_id: &str, x_axis: GraphableData, y_axis: GraphableData, start_time: SystemTime, duration: Duration) -> DrawResult<impl Fn((i32, i32)) -> Option<(f32, f32)>> {
    
// }

