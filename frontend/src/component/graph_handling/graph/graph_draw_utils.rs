use yew::Context;

use crate::bindings::{get_canvas_width, get_canvas_height};

use super::{Graph, graph_draw::{CHART_MARGIN_SIZE, CHART_LABEL_SIZE}};


pub fn time_axis_label_formatter(unix_time: &f64) -> String {
    time::OffsetDateTime::from_unix_timestamp(unix_time.clone() as i64)
        .expect("All stored timestamps are valid")
        .format(time::macros::format_description!("[year]-[month]-[day] [hour]:[minute]"))
        .expect("Given format is verified during compilation")
}

pub fn other_axis_label_formatter(data: &f64) -> String {
    format!("{:.0}", data)
}

impl Graph {
    pub fn convert_local_x_y_to_graph_x_y(&self, ctx: &Context<Self>, x: f64, y: f64) -> (Option<f64>, Option<f64>) {
        let x = if x.is_finite() {
            match self.previous_x_range.clone() {
                Some(x_range) => {
                    Some(
                        (x - CHART_MARGIN_SIZE as f64 - CHART_LABEL_SIZE as f64) 
                        * ((x_range.end - x_range.start) / ((get_canvas_width(ctx.props().canvas_id.to_string()) as f64) - (CHART_MARGIN_SIZE * 2 + CHART_LABEL_SIZE) as f64))
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
                        (-y +CHART_MARGIN_SIZE as f64 + CHART_LABEL_SIZE as f64) 
                        * ((y_range.end - y_range.start) / ((get_canvas_height(ctx.props().canvas_id.to_string()) as f64) - (CHART_MARGIN_SIZE * 2 + CHART_LABEL_SIZE * 2) as f64))
                        + y_range.end
                    )
                },
                None => None,
            }
        } else { None };
        (x,y)
    }

    pub fn convert_local_x_y_to_graph_x_sec_y(&self, ctx: &Context<Self>, x: f64, y: f64) -> (Option<f64>, Option<f64>) {
        let x = if x.is_finite() {
            match self.previous_x_range.clone() {
                Some(x_range) => {
                    Some(
                        (x - CHART_MARGIN_SIZE as f64 - CHART_LABEL_SIZE as f64) 
                        * ((x_range.end - x_range.start) / ((get_canvas_width(ctx.props().canvas_id.to_string()) as f64) - (CHART_MARGIN_SIZE * 2 + CHART_LABEL_SIZE) as f64))
                        + x_range.start
                    )
                },
                None => None,
            }
        } else { None };
        let sec_y = if y.is_finite() {
            match self.previous_sec_y_range.clone() {
                Some(sec_y_range) => {
                    Some(
                        (-y +CHART_MARGIN_SIZE as f64 + CHART_LABEL_SIZE as f64) 
                        * ((sec_y_range.end - sec_y_range.start) / ((get_canvas_height(ctx.props().canvas_id.to_string()) as f64) - (CHART_MARGIN_SIZE * 2 + CHART_LABEL_SIZE * 2) as f64))
                        + sec_y_range.end
                    )
                },
                None => None,
            }
        } else { None };
        (x,sec_y)
    }
}