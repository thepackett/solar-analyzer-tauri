use std::{rc::Rc};

use web_sys::{HtmlInputElement, FileList};
use yew::prelude::*;

use crate::{bindings, app_state::AppState, component::message_handling::simple_message::SimpleMessageProperties};

pub struct FileSelect {
    app_state: Rc<AppState>,
    context_handle: ContextHandle<Rc<AppState>>,
    files_in_progress: u32,
}

#[derive(Properties, PartialEq)]
pub struct FileSelectProperties {}

pub enum FileSelectMessage {
    ContextChanged(Rc<AppState>),
    NewFilesInProgress(u32),
    FileParseComplete,
}

impl Component for FileSelect {
    type Message = FileSelectMessage;
    type Properties = FileSelectProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let (app_state, _context_handle) = 
            ctx.link().context::<Rc<AppState>>(ctx.link().callback(FileSelectMessage::ContextChanged))
            .expect("AppState context must be set for FileSelect to function.");

        Self { app_state: app_state, context_handle: _context_handle, files_in_progress: 0  }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FileSelectMessage::ContextChanged(app_state) => {
                self.app_state = app_state.clone();
            }
            FileSelectMessage::NewFilesInProgress(n) => {
                self.files_in_progress += n;
                let message = SimpleMessageProperties { 
                    class: AttrValue::from("notification"), 
                    message:  AttrValue::from(format!("{} new files.", n)),
                };
                self.app_state.notification_callback.clone().expect("Notification callback must be set for FileSelect to function").emit(message.clone());
            },
            FileSelectMessage::FileParseComplete => todo!(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <form>
                <label for="myfile">{ "Select files:" }</label>
                <input onchange={ctx.link().callback(|e: Event| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    let file_list: FileList = FileList::from(input.files().unwrap());
                    let count = file_list.length();
                    let test = bindings::read_files(file_list);
                    
                    Self::Message::NewFilesInProgress(count)
                })} type="file" id="myfile" multiple=true/>
            </form>
        )
        //file_data_callbacks.new_file
    }
}
