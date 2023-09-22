use std::{rc::Rc, collections::HashMap, hash::Hash};

use yew::prelude::*;



pub struct Channel<S,C,I> 
    where
    S: PartialEq + Eq + Hash + Clone + 'static,
    C: PartialEq + Eq + Hash + Clone + 'static,
    I: PartialEq + Eq + Clone + 'static,
{
    state: Rc<ChannelState<S,C,I>>,
}

#[derive(PartialEq, Properties)]
pub struct ChannelProps<S,C,I> 
    where
    S: PartialEq + Eq + Hash + Clone + 'static,
    C: PartialEq + Eq + Hash + Clone + 'static,
    I: PartialEq + Eq + Clone + 'static,
{
    #[prop_or_default]
    pub children: Children,
    pub on_destroy_callback: Callback<HashMap<S, HashMap<C, Vec<I>>>>,
}

pub enum ChannelMessage<S,C,I> {
    ActivateChannelComponent((S, C, I)),
    DeactivateChannelComponent((S, C, I)),
}

impl<S,C,I> Component for Channel<S,C,I> 
    where
    S: PartialEq + Eq + Hash + Clone + 'static,
    C: PartialEq + Eq + Hash + Clone + 'static,
    I: PartialEq + Eq + Clone + 'static,
{
    type Message = ChannelMessage<S,C,I>;
    type Properties = ChannelProps<S,C,I>;

    fn create(ctx: &Context<Self>) -> Self {
        Channel {
            state: Rc::from(ChannelState::new(ctx.link().callback(|msg| {
                msg
            }))),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <ContextProvider<Rc<ChannelState<S,C,I>>> context={self.state.clone()}>
                {for ctx.props().children.iter()}
            </ContextProvider<Rc<ChannelState<S,C,I>>>>
        )
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChannelMessage::ActivateChannelComponent((system, channel, id)) => {
                // web_sys::console::info_1(&wasm_bindgen::JsValue::from_str(format!("Channel state: {:?}", self.state.map).as_str()));
                // web_sys::console::info_1(&wasm_bindgen::JsValue::from_str(format!("Activating system: {}, channel: {}, id:{}", system, channel, id).as_str()));
                let mut new_map = self.state.map.clone();
                let channel_map = new_map.entry(system).or_insert(HashMap::new());
                

                //Check to see if this operation if valid
                let valid_operation = channel_map.iter().fold(true, |acc, (iter_channel, id_vec)| {
                    if *iter_channel != channel && id_vec.len() > 0 {
                        false
                    } else {
                        acc
                    }
                });
                //If valid:
                if valid_operation {
                    let id_vec = channel_map.entry(channel.clone()).or_insert(Vec::new()); 
                    if !id_vec.contains(&id) {
                        id_vec.push(id);
                        self.state = Rc::from(
                            ChannelState {
                                callback: self.state.callback.clone(),
                                map: new_map,
                            }
                        )
                    }
                }
            },
            ChannelMessage::DeactivateChannelComponent((system, channel, id)) => {
                // web_sys::console::info_1(&wasm_bindgen::JsValue::from_str(format!("Deactivating system: {}, channel: {}, id:{}", system, channel, id).as_str()));
                let mut new_map = self.state.map.clone();
                let channel_map = new_map.entry(system).or_insert(HashMap::new());
                let id_vec = channel_map.entry(channel).or_insert(Vec::new());

                *id_vec = id_vec.iter().cloned().filter(|stored_id| {
                    *stored_id != id
                }).collect();
                
                self.state = Rc::from(
                    ChannelState {
                        callback: self.state.callback.clone(),
                        map: new_map,
                    }
                )
            },
        }
        false
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        ctx.props().on_destroy_callback.emit(self.state.map.clone());
    }
}

#[derive(PartialEq, Default)]
pub struct ChannelState<S,C,I>
    where
    S: PartialEq + Eq + Hash + Clone + 'static,
    C: PartialEq + Eq + Hash + Clone + 'static,
    I: PartialEq + Eq + Clone + 'static,
{
    pub callback: Callback<ChannelMessage<S,C,I>>,
    pub map: HashMap<S, HashMap<C, Vec<I>>>
}

impl<S,C,I> ChannelState<S,C,I> 
    where
    S: PartialEq + Eq + Hash + Clone + 'static,
    C: PartialEq + Eq + Hash + Clone + 'static,
    I: PartialEq + Eq + Clone + 'static,
{
    fn new(callback: Callback<ChannelMessage<S,C,I>>) -> ChannelState<S,C,I> {
        ChannelState { 
            callback: callback,
            map: HashMap::new(),
        }
    }
}