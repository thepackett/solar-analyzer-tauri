//Component responsible for containing messages

use std::rc::Rc;
use shared::types::fixed_size_queue::FixedSizeQueue;
use yew::prelude::*;
use crate::{app_state::AppState, component::message_handling::simple_message::SimpleMessage};

use super::simple_message::SimpleMessageProperties;

pub struct MessageBox {
    app_state: Rc<AppState>,
    _context_handle: ContextHandle<Rc<AppState>>,
    messages: FixedSizeQueue<(u32, SimpleMessageProperties)>,
}

pub enum MessageBoxMessage {
    NewMessage(SimpleMessageProperties),
    RemoveMessage(u32),
    ContextChanged(Rc<AppState>),
}

#[derive(PartialEq, Properties)]
pub struct MessageBoxProperties {
    pub class: AttrValue,
    pub queue_capacity: usize,

}

impl Component for MessageBox {
    type Message = MessageBoxMessage;
    type Properties = MessageBoxProperties;

    fn create(ctx: &yew::Context<Self>) -> Self {
        let props = ctx.props();
        let (previous_app_state, _context_handle) = 
            ctx.link().context::<Rc<AppState>>(ctx.link().callback(MessageBoxMessage::ContextChanged))
            .expect("AppState context must be set for MessageBox to function.");
        
        let new_app_state = Rc::from(
            AppState {
                update_app_state_callback: previous_app_state.update_app_state_callback.clone(),
                notification_callback: Some(ctx.link().callback(MessageBoxMessage::NewMessage)),
            }   
        );
        previous_app_state.update_app_state_callback.emit(new_app_state.clone());

        let mut component = MessageBox {
            app_state: new_app_state,
            _context_handle,
            messages: FixedSizeQueue::new(props.queue_capacity),
            
        };
        
        component.messages.push((0, SimpleMessageProperties {class: AttrValue::from("message TestClass"), message: AttrValue::from("TestMessage")}));
        component.messages.push((1, SimpleMessageProperties {class: AttrValue::from("error TestClass2"), message: AttrValue::from("TestMessage2")}));
        component.messages.push((2, SimpleMessageProperties {class: AttrValue::from("error message TestClass3"), message: AttrValue::from("TestMessage3")}));

        component
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MessageBoxMessage::NewMessage(msg_props) => {
                let last_element = self.messages.iter().last();
                match last_element {
                    Some(e) => {
                        let preceding_id = e.0;
                        let current_id = preceding_id.wrapping_add(1);
                        self.messages.push((current_id, msg_props));
                    },
                    None => {
                        self.messages.push((0, msg_props));
                    },
                }
                true
            },
            MessageBoxMessage::RemoveMessage(remove_id) => {
                self.messages.retain(|e| {
                    let msg_id = e.0;
                    msg_id != remove_id
                });
                true
            },
            MessageBoxMessage::ContextChanged(app_state) => {
                self.app_state = app_state;
                true
            },
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        let messages = &self.messages;

        html!{
            <div>
                <ol class={ctx.props().class.to_string()}>
                    {
                    messages.iter().map(|message| 
                        { 
                        html!(
                            <li key={format!("msg-id-{}", message.0)} onanimationend={generate_animation_end_callback(message.0, ctx)}>
                                <SimpleMessage ..message.1.clone()/>
                            </li> 
                        ) 
                        }).collect::<Html>() 
                    }
                </ol>
            </div>
        }
    }
}

fn generate_animation_end_callback(id: u32, ctx: &yew::Context<MessageBox>) -> Callback<AnimationEvent>{
    let remove_message = ctx.link().callback(|id| {
        MessageBoxMessage::RemoveMessage(id)
    });
    Callback::from(move |e: AnimationEvent| {
        remove_message.emit(id);
    })
}