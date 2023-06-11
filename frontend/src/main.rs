mod component;
mod bindings;
mod app_state;

use component::message_handling::message_box::MessageBoxProperties;
use tracing::{event, Level};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use web_sys::HtmlInputElement;
use yew::prelude::*;


use crate::{component::{file_handling::file_select::FileSelect, message_handling::message_box::MessageBox, visual::{sidebar::Sidebar, sidemenu::Sidemenu, svg::{cog::Cog, file_upload::FileUpload}}, control::{switch::{Switch, SwitchProperties}, button::{Button, ButtonProperties}}}, app_state::AppStateHolder, bindings::{Theme, remove_classes, add_classes, toggle_classes}};

fn main() {
    bindings::set_detected_theme();
    tracing_wasm::set_as_global_default();
    event!(Level::INFO, "Main function run");
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "World".to_string());

    {
        let welcome = welcome.clone();
        use_effect_with_deps(
            move |name| {
                update_welcome_message(welcome, name.clone());
                || ()
            }
            , (*name).clone());
    }

    let message = (*welcome).clone();

    let message_box_props = MessageBoxProperties{
        class: AttrValue::from("MessageBoxClassName"),
        queue_capacity: 30,
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
        <AppStateHolder>
            <div class="main-layout">
                <div class="main-contents">
                    <h2>{message}</h2>
                    <p>{"Paragraph"}</p>
                    <p>{"Paragraph with a whole lot of text. Wow, that's a lot of text! how much text you ask? Well, envision the most text you've ever seen in one place before, and then add a bit more text to even that. It just keeps going! No one knows why, it just is, at this point. Something that we've accepted as an irretractable part of nature, of our very existence."}</p>
                    
                    <MessageBox ..message_box_props/>
                </div>
                <Sidebar>
                    <Button ..settings_button_props>
                        <Cog/>
                    </Button>
                    <Sidemenu class="settings-menu">
                        <Switch ..theme_switch_props/>
                    </Sidemenu>
                    <Button ..file_upload_button_props>
                        <FileUpload/>
                    </Button>
                    <Sidemenu class="file-upload-menu">
                        <p>{"Side menu 2!"}</p>
                        <FileSelect/>
                    </Sidemenu>
                </Sidebar>
            </div>
        </AppStateHolder>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String){
    wasm_bindgen_futures::spawn_local(async move {
        // This will call our glue code all the way through to the tauri
        // back-end command and return the `Result<String, String>` as
        // `Result<JsValue, JsValue>`.
        match hello(name).await {
            Ok(message) => {
                welcome.set(message.as_string().unwrap());
            }
            Err(e) => {
                let window = web_sys::window().unwrap();
                window
                    .alert_with_message(&format!("Error: {:?}", e))
                    .unwrap();
            }
        }
    })
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
  #[wasm_bindgen(js_name = invokeHello, catch)]
  pub async fn hello(name: String) -> Result<JsValue, JsValue>;
}