use web_sys::{HtmlInputElement, FileList};
use yew::prelude::*;

use crate::{bindings, component::message_handling::simple_message::SimpleMessageProperties, component_channel::ComponentChannelTx};

pub struct FileSelect {
    // app_state: Rc<AppState>,
    // _context_handle: ContextHandle<Rc<AppState>>,
}

#[derive(Properties, PartialEq)]
pub struct FileSelectProperties {
    pub notification_tx: ComponentChannelTx<SimpleMessageProperties>,
}

pub enum FileSelectMessage {
    // ContextChanged(Rc<AppState>),
    FileHandlingComplete((Vec<String>, Vec<bindings::ReadFileError>)),
}

impl Component for FileSelect {
    type Message = FileSelectMessage;
    type Properties = FileSelectProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FileSelectMessage::FileHandlingComplete((good, failed)) => {
                if good.len() > 0 {
                    let message = SimpleMessageProperties { 
                        class: AttrValue::from("notification"), 
                        message: AttrValue::from(format!("Parsing {} new file{}.", good.len(), if good.len() == 1 {""} else {"s"})), 
                    };
                    if let Err(e) = ctx.props().notification_tx.try_send(message) {
                        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(e.to_string().as_str()));
                    };
                }
                failed.into_iter().for_each(|failure| {
                    let error = SimpleMessageProperties { 
                        class: AttrValue::from("error"), 
                        message: AttrValue::from(failure.to_string()), 
                    };
                    if let Err(e) = ctx.props().notification_tx.try_send(error) {
                        web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(e.to_string().as_str()));
                    };
                });
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_new_file = ctx.link().callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let file_list: FileList = FileList::from(input.files().unwrap());
            let results = (0..file_list.length()).into_iter().map(|i| {
                let file = match file_list.item(i) {
                    Some(file) => {
                        file
                    },
                    None => {
                        //This should never happen, but in case it does, just return an error.
                        return Err(bindings::ReadFileError::UnknownError);
                    }
                };
                match bindings::read_file(file.clone()) {
                    Ok(_) => {
                        Ok(file.name())
                    },
                    Err(e) => {
                        Err(e)
                    }
                }
            }).collect::<Vec<_>>();

            let sorted_results = results.into_iter().fold((Vec::new(), Vec::new()), |mut acc, x| {
                match x {
                    Ok(name) => acc.0.push(name),
                    Err(e) => acc.1.push(e),
                }
                acc
            });

            Self::Message::FileHandlingComplete(sorted_results)
        });



        html!(
            <form>
                <label for="myfile">{ "Select files:" }</label>
                <input onchange={on_new_file} type="file" id="myfile" multiple=true/>
            </form>
        )
    }
}
