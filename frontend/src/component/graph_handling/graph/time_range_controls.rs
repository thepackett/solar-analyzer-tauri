use time::Date;
use yew::prelude::*;

use crate::bindings::{teardown_graph_date_picker, setup_graph_date_picker};

pub struct TimeRangeSelector {}

#[derive(PartialEq, Properties)]
pub struct TimeRangeSelectorProps {
    pub id: AttrValue,
    pub callback: Callback<DateRange>,
}

pub enum TimeRangeSelectorMessage {}

impl Component for TimeRangeSelector {
    type Message = TimeRangeSelectorMessage;
    type Properties = TimeRangeSelectorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        TimeRangeSelector {  }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        //setup logic for detecting date change.
        
        html!(
            <div>
                <p>{"Date range goes here"}</p>
                <input type={"text"} id={props.id.clone()} />
                //Include resolution selector that defaults to auto?
            </div>
        )
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return
        }

        setup_graph_date_picker(ctx.props().id.to_string());
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        teardown_graph_date_picker(ctx.props().id.to_string());
    }
}


pub struct DateRange {
    start: Date,
    end: Date,
}