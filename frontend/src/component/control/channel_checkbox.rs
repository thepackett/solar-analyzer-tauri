use std::{rc::Rc, collections::HashMap, hash::Hash};

use yew::prelude::*;

use super::channel::ChannelState;


//A checkbox that is only clickable so long as no other channel is currently active.

//Whenever it is clicked, a callback to a channel state manager containing an activation request is sent. If valid, the next context update will allow the box to be checked.

//Whenever it is unchecked, or deconstructed, a callback to a channel state manager containing a deactivation request is sent.


pub struct ChannelCheckbox<S,C,I> 
    where
    S: PartialEq + Eq + Hash + Clone + 'static,
    C: PartialEq + Eq + Hash + Clone + 'static,
    I: PartialEq + Eq + Clone + 'static,
{
    // listener: Option<EventListener>,
    // non_channel_active_checkboxes: usize,
    channel_state: Rc<ChannelState<S,C,I>>,
    _context_handle: ContextHandle<Rc<ChannelState<S,C,I>>>,
}

#[derive(PartialEq, Properties, Clone)]
pub struct ChannelCheckboxProps<S,C,I> 
    where
    S: PartialEq + Eq + Hash + Clone + 'static,
    C: PartialEq + Eq + Hash + Clone + 'static,
    I: PartialEq + Eq + Clone + 'static,
{
    pub system: S,
    pub channel: C,
    pub id: I,
    pub init_checked: bool,
}

pub enum ChannelCheckboxMessage<S,C,I> 
    where
    S: PartialEq + Eq + Hash + Clone + 'static,
    C: PartialEq + Eq + Hash + Clone + 'static,
    I: PartialEq + Eq + Clone + 'static,
{
    ContextChanged(Rc<ChannelState<S,C,I>>),
    Activate,
    Deactivate,
}

impl<S,C,I> Component for ChannelCheckbox<S,C,I> 
    where
    S: PartialEq + Eq + Hash + Clone + 'static,
    C: PartialEq + Eq + Hash + Clone + 'static,
    I: PartialEq + Eq + Clone + 'static,
{
    type Message = ChannelCheckboxMessage<S,C,I>;
    type Properties = ChannelCheckboxProps<S,C,I>;

    fn create(ctx: &Context<Self>) -> Self {
        let (channel_state, _context_handle) = 
            ctx.link().context::<Rc<ChannelState<S,C,I>>>(ctx.link().callback(Self::Message::ContextChanged))
            .expect("ChannelState context must be set for ChannelCheckbox to function.");

        ChannelCheckbox {
            channel_state,
            _context_handle,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_state = self.get_state(ctx, &self.channel_state.map);
        html!(
            if current_state == CheckboxState::Active {
                <input type="checkbox" onclick={ctx.link().callback(|_| Self::Message::Deactivate)} checked=true readonly=false/>
            } else if current_state == CheckboxState::Inactive {
                <input type="checkbox" onclick={ctx.link().callback(|_| Self::Message::Activate)} readonly=false/>
            } else if current_state == CheckboxState::Unactivatable{
                <input type="checkbox" readonly=true disabled=true/>
            }
        )
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let props = ctx.props();
        match msg {
            ChannelCheckboxMessage::ContextChanged(new_state) => {
                // web_sys::console::info_1(&wasm_bindgen::JsValue::from_str(format!("Context Changed").as_str()));
                let new_checkbox_state = self.get_state(ctx, &new_state.map);
                let old_checkbox_state = self.get_state(ctx, &self.channel_state.map);
                self.channel_state = new_state;
                old_checkbox_state != new_checkbox_state
            },
            ChannelCheckboxMessage::Activate => {
                self.channel_state.callback.emit(super::channel::ChannelMessage::ActivateChannelComponent((props.system.clone(), props.channel.clone(), props.id.clone())));
                false
            },
            ChannelCheckboxMessage::Deactivate => {
                self.channel_state.callback.emit(super::channel::ChannelMessage::DeactivateChannelComponent((props.system.clone(), props.channel.clone(), props.id.clone())));
                false
            },
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let props = ctx.props();

        if props.init_checked {
            self.channel_state.callback.emit(super::channel::ChannelMessage::ActivateChannelComponent((props.system.clone(), props.channel.clone(), props.id.clone())));
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        let props = ctx.props();
        self.channel_state.callback.emit(super::channel::ChannelMessage::DeactivateChannelComponent((props.system.clone(), props.channel.clone(), props.id.clone())));
    }
}

impl<S,C,I> ChannelCheckbox<S,C,I> 
    where
    S: PartialEq + Eq + Hash + Clone + 'static,
    C: PartialEq + Eq + Hash + Clone + 'static,
    I: PartialEq + Eq + Clone + 'static,
{
    fn get_state(&self, ctx: &Context<Self>, map: &HashMap<S, HashMap<C, Vec<I>>>) -> CheckboxState {
        let props = ctx.props();
        match map.get(&props.system) {
            Some(channel_map) => {
                for (channel, active_elements) in channel_map {
                    if *channel == props.channel {
                        if active_elements.contains(&props.id) {
                            return CheckboxState::Active
                        }
                    } else {
                        if !active_elements.is_empty() {
                            return CheckboxState::Unactivatable
                        }
                    }
                }
                CheckboxState::Inactive //We iterated and found nothing that said this element was either activated of inactivatable, thus it must be inactive.
            },
            None => {
                CheckboxState::Inactive //There are no channels even registered for this system, thus the state must be Inactive
            },
        }
    }
}

#[derive(PartialEq)]
enum CheckboxState {
    Active,
    Inactive,
    Unactivatable,
}