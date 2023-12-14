mod component;
mod bindings;
mod app_state;
mod component_channel;

use std::rc::Rc;

use component::message_handling::{message_box::MessageBoxProperties, simple_message::SimpleMessageProperties};
use tracing::{event, Level};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use web_sys::HtmlInputElement;
use yew::prelude::*;


use crate::{component::{file_handling::file_select::{FileSelect, FileSelectProperties}, message_handling::message_box::MessageBox, visual::{sidebar::Sidebar, sidemenu::Sidemenu, svg::{cog::Cog, file_upload::FileUpload}}, control::{switch::{Switch, SwitchProperties}, button::{Button, ButtonProperties}, shared_data_context::SharedDataContext}, graph_handling::graph::{Graph, graph_coordination::SharableGraphData}}, bindings::{Theme, remove_classes, add_classes, toggle_classes}, component_channel::ComponentChannel};

fn main() {
    bindings::set_detected_theme();
    // tracing_wasm::set_as_global_default();
    // event!(Level::INFO, "Main function run");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let (notification_tx, notification_rx) 
        = ComponentChannel::get(crossbeam::channel::bounded::<SimpleMessageProperties>(30));

    let message_box_props = MessageBoxProperties {
        class: AttrValue::from("MessageBoxClassName"),
        queue_capacity: 30,
        message_rx: notification_rx.clone(),
    };

    let file_select_props = FileSelectProperties {
        notification_tx: notification_tx.clone()
    };

    let theme_switch_props = SwitchProperties{
        class: AttrValue::from("theme-switch"),
        initial: match bindings::get_theme() {
            bindings::Theme::Dark => true,
            bindings::Theme::Light => false,
        },
        onclick: Callback::from(|e: MouseEvent| {
            let checkbox: HtmlInputElement = e.target_unchecked_into();
            //Checked is dark mode
            if checkbox.checked() {
                bindings::set_theme(Theme::Dark);
            } else {
                bindings::set_theme(Theme::Light);
            }   
        }),
    };

    let settings_button_props = ButtonProperties {
        class: AttrValue::from("settings-button"),
        onclick: Callback::from(|_e| {
            remove_classes(".sidebar .button:not(.settings-button)".to_owned(), "menu-visible".to_owned());
            toggle_classes(".sidebar .settings-button".to_owned(), "menu-visible".to_owned());

            remove_classes(".sidemenu:not(.settings-menu)".to_owned(), "visible".to_owned());
            toggle_classes(".sidemenu.settings-menu".to_owned(), "visible".to_owned());
        }),
        children: Children::default(),
    };

    let file_upload_button_props = ButtonProperties {
        class: AttrValue::from("file-upload-button"),
        onclick: Callback::from(|_e| {
            remove_classes(".sidebar .button:not(.file-upload-button)".to_owned(), "menu-visible".to_owned());
            toggle_classes(".sidebar .file-upload-button".to_owned(), "menu-visible".to_owned());

            remove_classes(".sidemenu:not(.file-upload-menu)".to_owned(), "visible".to_owned());
            toggle_classes(".sidemenu.file-upload-menu".to_owned(), "visible".to_owned());
        }),
        children: Children::default(),
    };

    html! {
        // <AppStateHolder>
        // <GraphStateHolder>
            <div class="main-layout">
                <div class="main-content">
                    <SharedDataContext<Option<SharableGraphData>> init={Rc::from(None)}>
                        <Graph canvas_id={AttrValue::from("test")} canvas_container_id={AttrValue::from("test-container")} notification_tx={notification_tx.clone()}/>
                        <Graph canvas_id={AttrValue::from("test2")} canvas_container_id={AttrValue::from("test-container2")} notification_tx={notification_tx.clone()}/>
                        <MessageBox ..message_box_props/>
                    </SharedDataContext<Option<SharableGraphData>>>
                </div>
                <Sidebar>
                    <Button ..settings_button_props>
                        <Cog class="sidebar-icon svg" />
                    </Button>
                    <Sidemenu class="settings-menu">
                        <Switch ..theme_switch_props/>
                    </Sidemenu>
                    <Button ..file_upload_button_props>
                        <FileUpload class="sidebar-icon svg" />
                    </Button>
                    <Sidemenu class="file-upload-menu">
                        <p>{"Side menu 2!"}</p>
                        <FileSelect ..file_select_props/>
                    </Sidemenu>
                </Sidebar>
            </div>
        // </GraphStateHolder>
        // </AppStateHolder>
    }
}
