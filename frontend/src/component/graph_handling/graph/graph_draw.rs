use std::ops::Range;
use plotters::{prelude::*, coord::{ranged1d::{ValueFormatter, NoDefaultFormatting, KeyPointHint}, types::RangedCoordf64}};
use plotters_canvas::CanvasBackend;
use shared::graph::{graph_type::GraphType, graph_axis::{AxisDataType, AxisDataOption}};
use time::OffsetDateTime;
use yew::Context;

use crate::component::visual::theme_data::ThemeData;

use super::{Graph, graph_draw_utils::{other_axis_label_formatter, time_axis_label_formatter}};

pub const CHART_MARGIN_SIZE: u32 = 20;
pub const CHART_LABEL_SIZE: u32 = 30;

struct GraphDataRange {
    pub range: RangedCoordf64,
    pub data_type: AxisDataType,
}

impl Ranged for GraphDataRange {
    type FormatOption = NoDefaultFormatting;
    type ValueType = f64;

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        self.range.map(value, limit)
    }

    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<Self::ValueType> {
        match self.data_type {
            AxisDataType::Time | AxisDataType::PeriodicTime => {
                let start = self.range.range().start;
                let end = self.range.range().end;
                //We want even divisions of:
                //1 Year,
                //1 Month,
                //1 Week,
                //1 Day,
                //6 Hour,
                //1 Hour,
                //15 Minutes,
                //5 Minutes,
                //All of these line up as nice subdivisions of the larger unit, with the exception of weeks
                //There are a fractional number of weeks per month, and furthermore not all months have the same number of days.
                //We want to show two of these divisions at all times, in both bold and regular lines. For example, if years are bold, then months are normal
                //If Months are bold, then weeks are normal.
                //If 15 minutes are bold, then 5 minutes are normal. Etc.
                //Transition from one unit to a smaller unit if less than 1 whole unit is visible.
                //Transition from one unit to a larger unit if at least 1 whole larger unit is visible.
                let key_points: Vec<f64> = Vec::new();
                
                match end - start {
                    x if x >= (60*60*24*365) as f64 => { //If we contain at least a whole year* (365 days)
                        let start_date_time = OffsetDateTime::from_unix_timestamp(end as i64).unwrap_or(OffsetDateTime::UNIX_EPOCH);
                    },
                    x if x >= (60*60*24*30) as f64 => { //If we contain at least a whole month* (30 days)

                    },
                    x if x >= (60*60*24*7) as f64 => { //If we contain at least a whole week

                    },
                    x if x >= (60*60*24) as f64 => { //If we contain at least a whole day

                    },
                    x if x >= (60*60*6) as f64 => { //If we contain at least 6 hours

                    },
                    x if x >= (60*60) as f64 => { //If we contain at least 1 hour

                    },
                    _ => { //Assume we contain at least 15 minutes, since there are no smaller subdivision pairs left

                    }
                }
                vec![0f64, 1f64*10000f64, 2f64*10000f64, 3f64*10000f64, 4f64*10000f64, 5f64*10000f64, 6f64*10000f64, 7f64*10000f64, 8f64*10000f64, 9f64*10000f64]
            },
            _ => self.range.key_points(hint),
        }
    }

    fn range(&self) -> std::ops::Range<Self::ValueType> {
        self.range.range()
    }
}

impl ValueFormatter<f64> for GraphDataRange {
    fn format(_value: &f64) -> String {
        RangedCoordf64::format(_value)
    }

    fn format_ext(&self, value: &f64) -> String {
        Self::format(value)
    }
}

impl Graph {
    pub fn draw_graph(&mut self, ctx: &Context<Graph>, theme: ThemeData) -> Result<(), Box<dyn std::error::Error>>  {
        //Thinking about the broader picture, I really need to divide behavior based on graph type.
        //Currently 2 graph types are planned.
        //1. X axis time line series
        //2. X axis periodic time line series
        //3. X-Y scatter plot
        //Most behavior can be shared between these plot types (graph mesh / captions, data ranges, styles, zoom controls, etc)
        //But some behavior can't. For example, X-Y scatters must be drawn without a line. "Vertical line" markpoints only make sense for the x axis line series.



        let canvas_id = ctx.props().canvas_id.as_str();
        let line_series = &self.line_series;
        let backend = CanvasBackend::new(canvas_id).expect("cannot find canvas");
        let root = backend.into_drawing_area();
        let caption_font = FontDesc::new(FontFamily::from("sans-serif"), 20.0, FontStyle::Normal);
        let label_font = FontDesc::new(FontFamily::from("sans-serif"), 12.0, FontStyle::Normal);
    
        root.fill(&RGBColor::from(&theme.theme_graph_background))?;
    
        let x_axis_range = if let Some(x_range) = self.previous_x_range.clone() {
            x_range
        } else { 
            let mut x_range = line_series.series.iter().map(|series| {
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
            x_range.end = x_range.end + 0.000001f64;
            self.previous_x_range = Some(x_range.clone());
            x_range
        };
    
        let y_axis_range = if let Some(y_range) = self.previous_y_range.clone() {
            y_range
        } else { 
            let mut y_range = line_series.series.iter().map(|series| {
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
            y_range.end = y_range.end + 0.000001f64;
            self.previous_y_range = Some(y_range.clone());
            y_range
        };
    
        let secondary_y_axis_range = if let Some(sec_y_range) = self.previous_sec_y_range.clone() {
            sec_y_range
        } else { 
            let mut sec_y_range = line_series.secondary_series.iter().map(|series| {
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
            sec_y_range.end = sec_y_range.end + 0.000001f64;
            self.previous_sec_y_range = Some(sec_y_range.clone());
            sec_y_range
        };
    
        let mut chart = ChartBuilder::on(&root)
            .margin(CHART_MARGIN_SIZE)
            .caption(format!("temp caption"), caption_font.clone().with_color(RGBColor::from(&theme.theme_text)))
            .x_label_area_size(CHART_LABEL_SIZE)
            .y_label_area_size(CHART_LABEL_SIZE)
            .build_cartesian_2d(GraphDataRange {
                range: RangedCoordf64::from(x_axis_range.clone()),
                data_type: self.graph_state.x_axis.requests.first().unwrap_or(&(AxisDataType::Time, AxisDataOption::Average)).0.clone(),
            }, GraphDataRange{
                range: RangedCoordf64::from(y_axis_range.clone()),
                data_type: self.graph_state.y_axis.0.requests.first().unwrap_or(&(AxisDataType::BatteryVoltage, AxisDataOption::Average)).0.clone(),
            })?;

        
        //chart.set_secondary_coord(x_axis_range, secondary_y_axis_range).configure_secondary_axes();
        


        //Do all plotting based on graph type.
        match self.get_graph_type() {
            GraphType::XAxisLine => {
                //Draw the data series
                line_series.series.iter().enumerate().for_each(|series| {
                    let name = series.1.name.clone();
                    //If drawn as is, the line series "line" for data outside the range of the graph is drawn such that the data is clamped to the graph bounds.
                    //This causes it to display an incorrect slope, and thus incorrect values.
                    //Here we will modify the data for display such that any data going from in bounds to out of bounds, or from out of bounds to in bounds
                    // hits the y (or x) axis at the right location by adding points on the line from one point to the next.
                    let data = series.1.data_points.clone().windows(2).map(|point| {
                        let current_point = point[0];
                        let next_point = point[1];
                        let slope = (next_point.1 - current_point.1) / (next_point.0 - current_point.0);
                        let mut valid_points = vec![current_point];

                        //Since all x values are sorted in ascending x, we only care about points that are of higher x value than the current point
                        //Furthermore, since we are only interested in lines of shorter length, and all lines share the same slope, any line with
                        // an x value greater than the x value of the next point is longer, and thus invalid

                        //Solve for the y_max intercept line
                        let y_max_intercept_x = (y_axis_range.end - current_point.1) / slope + current_point.0;
                        if y_max_intercept_x > current_point.0 && y_max_intercept_x < next_point.0 {
                            //We don't care about the Ok case since there's no point in inserting identical points.
                            if let Err(index) = valid_points.binary_search_by(|element| element.0.total_cmp(&y_max_intercept_x)) {
                                valid_points.insert(index, (y_max_intercept_x, y_axis_range.end));
                            }
                        } 

                        //Solve for the y_min intercept line
                        let y_min_intercept_x = (y_axis_range.start - current_point.1) / slope + current_point.0;
                        if y_min_intercept_x > current_point.0 && y_min_intercept_x < next_point.0 {
                            if let Err(index) = valid_points.binary_search_by(|element| element.0.total_cmp(&y_min_intercept_x)) {
                                valid_points.insert(index, (y_min_intercept_x, y_axis_range.start));
                            }
                        } 

                        //Solve for the x_max intercept line
                        if x_axis_range.end > current_point.0 && x_axis_range.end < next_point.0 {
                            if let Err(index) = valid_points.binary_search_by(|element| element.0.total_cmp(&x_axis_range.end)) {
                                valid_points.insert(index, (x_axis_range.end, slope * (x_axis_range.end - current_point.0) + current_point.1));
                            }
                        }

                        //Solve for the x_min intercept line
                        if x_axis_range.start > current_point.0 && x_axis_range.start < next_point.0 {
                            if let Err(index) = valid_points.binary_search_by(|element| element.0.total_cmp(&x_axis_range.start)) {
                                valid_points.insert(index, (x_axis_range.start, slope * (x_axis_range.start - current_point.0) + current_point.1));
                            }
                        }

                        valid_points
                    }).flatten().collect::<Vec<_>>();
                    match chart.draw_series(LineSeries::new(data, Palette99::pick(series.0))) {
                        Ok(line_series) => {
                            //Configure labels and legend here
                            line_series
                                .label(name)
                                .legend(move |(x,y)| {PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(series.0))});
                        },
                        Err(_) => {},
                    }
                }); 

                line_series.series.iter().chain(line_series.secondary_series.iter()).enumerate().for_each(|series| {
                    let mut points = Vec::new();
                    self.markpoints.iter().for_each(|markpoint| {
                        match series.1.data_points.binary_search_by(|e| {
                            e.0.total_cmp(&markpoint.0)
                        }) {
                            Ok(index) => {
                                let point = series.1.data_points[index];
                                //Only do anything if the markpoint fits inside the limits of our data
                                if x_axis_range.contains(&point.0) && y_axis_range.contains(&point.1) {
                                    points.push(point);
                                }
                            },
                            Err(index) => {
                                //Only do anything if the markpoint fits inside the limits of our data
                                if index > 0 && index < series.1.data_points.len() {
                                    let final_point = series.1.data_points[index];
                                    let initial_point = series.1.data_points[index-1];
                                    let interpolated_point = (
                                        markpoint.0,
                                        (final_point.1 - initial_point.1)/(final_point.0 - initial_point.0)*(markpoint.0 - initial_point.0) + initial_point.1
                                    );
                                    if x_axis_range.contains(&interpolated_point.0) && y_axis_range.contains(&interpolated_point.1) {
                                        points.push(interpolated_point);
                                    } 
                                }
                            },
                        }
                        //Draw a vertical line
                        let _result = chart.draw_series(
                            LineSeries::new([(markpoint.0.clone(), y_axis_range.start.clone()), (markpoint.0.clone(), y_axis_range.end.clone())], &BLACK)
                        );
                    });

                    //Draw the markpoint circles
                    if !points.is_empty() {
                        let _result = chart.draw_series(PointSeries::of_element(points, 5, &Palette99::pick(series.0), &|coord, size, style| {
                            EmptyElement::at(coord)
                                + Circle::new((0,0), size, style)
                                + Text::new(format!("({}, {:.2})", time_axis_label_formatter(&coord.0), coord.1), (0,15), ("sans-serif", 15).into_font().color(&RGBColor::from(&theme.theme_text)))
                        }));
                    }
                });
            },
            GraphType::XYScatter => todo!(),
        }


        
        

        let x_axis_formatter = match self.graph_state.x_axis.requests.first() {
            Some((data_type, data_option)) => {
                match data_type {
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
        let y_axis_formatter = match self.graph_state.y_axis.0.requests.first() {
            Some((data_type, data_option)) => {
                match data_type {
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
        let secondary_y_axis_formatter = match self.graph_state.y_axis.1.requests.first() {
            Some((data_type, _data_option)) => {
                match data_type {
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

        let x_axis_description = match self.graph_state.x_axis.requests.first() {
            Some((data_type, _data_option)) => {
                data_type.get_unit().get_name()
            },
            None => "",
        };
        let y_axis_description = match self.graph_state.y_axis.0.requests.first() {
            Some((data_type, _data_option)) => {
                data_type.get_unit().get_name()
            },
            None => "",
        };
        let secondary_y_axis_description = match self.graph_state.y_axis.1.requests.first() {
            Some((data_type, _data_option)) => {
                data_type.get_unit().get_name()
            },
            None => "",
        };


        chart.configure_mesh()
            .light_line_style(&RGBColor::from(&theme.theme_graph_mesh_light))
            .bold_line_style(&RGBColor::from(&theme.theme_graph_mesh_dark))
            .axis_style(&RGBColor::from(&theme.theme_graph_border))
            .x_desc(x_axis_description)
            .x_label_style(label_font.clone())
            .x_labels(3)
            .x_label_formatter(&x_axis_formatter)
            .y_desc(y_axis_description)
            .y_label_style(label_font.clone())
            .y_labels(3)
            .y_label_formatter(&y_axis_formatter)
            .label_style(&RGBColor::from(&theme.theme_text))
            .draw()?;

        chart.configure_series_labels()
            .label_font(label_font.clone().color(&RGBColor::from(&theme.theme_text)))
            .background_style(&RGBColor::from(&theme.theme_graph_background))
            .border_style(&RGBColor::from(&theme.theme_graph_border))
            .position(SeriesLabelPosition::UpperRight)
            .draw()?;

        //Draw and configure secondary axis, if it is present
        match self.graph_state.y_axis.1.requests.first() {
            Some((data_type, data_option)) => {
                chart.set_secondary_coord(x_axis_range, secondary_y_axis_range)
                    .configure_secondary_axes()
                    .y_desc(secondary_y_axis_description)
                    .y_label_formatter(&secondary_y_axis_formatter)
                    .x_labels(3);
                    
            },
            None => {},
        };
    
    
        root.present()?;
        Ok(())
    }

    pub fn get_graph_type(&self) -> GraphType {
        match self.graph_state.x_axis.requests.first() {
            Some((data_type, _data_option)) => {
                match data_type {
                    AxisDataType::Time => GraphType::XAxisLine,
                    _ => GraphType::XYScatter,
                }
            },
            None => {
                GraphType::XAxisLine //In the case where there is no data to graph, an example line chart will be plotted
            }
        }
    }
}
