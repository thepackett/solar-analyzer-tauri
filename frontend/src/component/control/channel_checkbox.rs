use gloo_events::EventListener;
use yew::prelude::*;


//A checkbox that is only clickable so long as no other channel is currently active.

//Whenever it is checked, an enabled event containing the channel as a payload is sent out on its system that other channel checkboxes on the same system will receive.

//If a channel checkbox receives an enable event containing a different channel it increments a counter of active checkboxes from other channels.
//If this checkbox was previously enabled, then it will disable itself until the number of active checkboxes from other channels is zero.

//Whenever it is unchecked, or deconstructed, a disabled event containing the channel as a payload is sent out on its system that other channel checkboxes on the same system will receive.

//If a channel checkbox receives a disable event containing a different channel it decrements a counter of active checkboxes from other channels.
//If this checkbox was previously disabled, then it will enable itself when the number of active checkboxes from other channels is zero.

pub struct ChannelCheckbox {
    listener: Option<EventListener>,
    non_channel_active_checkboxes: usize,
}

#[derive(PartialEq, Properties)]
pub struct ChannelCheckboxProps {
    system: AttrValue,
    channel: AttrValue,
}

pub enum ChannelCheckboxMessage {

}

impl Component for ChannelCheckbox {
    type Message = ChannelCheckboxMessage;
    type Properties = ChannelCheckboxProps;

    fn create(ctx: &Context<Self>) -> Self {
        ChannelCheckbox { 
            listener: None,
            non_channel_active_checkboxes: 0,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(

        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        
    }
}