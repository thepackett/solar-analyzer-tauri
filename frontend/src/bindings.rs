use std::num::ParseIntError;

use wasm_bindgen::prelude::*;
use web_sys::FileList;

use crate::component::visual::theme_data::{ThemeData, Color};

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = readFiles, catch)]
    //The Error variant of this will be a JsValue that was gotten from a rust enum, and can be converted back for completeness
    pub fn read_files(file_list: FileList) -> Result<(),JsValue>;

    #[wasm_bindgen(js_name = setTheme)]
    pub fn set_theme_js(theme: String);

    #[wasm_bindgen(js_name = setDetectedTheme)]
    pub fn set_detected_theme();

    #[wasm_bindgen(js_name = getTheme)]
    fn get_theme_js() -> String;

    #[wasm_bindgen(js_name = setToggles)]
    pub fn set_toggles(css_selector: String, state: bool);

    #[wasm_bindgen(js_name = removeClasses)]
    pub fn remove_classes(css_selector: String, classes: String);

    #[wasm_bindgen(js_name = addClasses)]
    pub fn add_classes(css_selector: String, classes: String);

    #[wasm_bindgen(js_name = toggleClasses)]
    pub fn toggle_classes(css_selector: String, classes: String);

    #[wasm_bindgen(js_name = getStyle)]
    pub fn get_style(css_selector: String, style: String) -> Option<String>;

    #[wasm_bindgen(js_name = getElementOffsetHeight)]
    pub fn get_element_offset_height(css_selector: String) -> Option<i32>;

    #[wasm_bindgen(js_name = getElementOffsetWidth)]
    pub fn get_element_offset_width(css_selector: String) -> Option<i32>;

    #[wasm_bindgen(js_name = setCanvasSize)]
    pub fn set_canvas_size(canvas_id: String, width_px: i32, height_px: i32);

    #[wasm_bindgen (js_name = resizeCanvas)]
    pub fn resize_canvas(canvas_id: String, container_id: String);

    #[wasm_bindgen (js_name = setupCanvasEvents)]
    pub fn setup_canvas_events(canvas_id: String, container_id: String);

    #[wasm_bindgen (js_name = teardownCanvasEvents)]
    pub fn teardown_canvas_events(canvas_id: String);

    #[wasm_bindgen (js_name = retrieveSolarData)]
    pub fn retrieve_solar_data(json_string: String);
}

pub fn get_theme() -> Theme {
    let theme = get_theme_js();
    if theme == "theme-light" {
        return Theme::Light
    } else {
        return Theme::Dark
    }
}

pub fn get_theme_data() -> Result<ThemeData, Box<dyn std::error::Error>> {
    Ok(ThemeData {
        theme_primary: Color::from_hex_code(get_style(":root".to_string(), "--theme-primary".to_string()).ok_or("Could not get style")?.as_str())?,
        theme_secondary: Color::from_hex_code(get_style(":root".to_string(), "--theme-secondary".to_string()).ok_or("Could not get style")?.as_str())?,
        theme_background_primary: Color::from_hex_code(get_style(":root".to_string(), "--theme-background-primary".to_string()).ok_or("Could not get style")?.as_str())?,
        theme_background_secondary: Color::from_hex_code(get_style(":root".to_string(), "--theme-background-secondary".to_string()).ok_or("Could not get style")?.as_str())?,
        theme_background_tertiary: Color::from_hex_code(get_style(":root".to_string(), "--theme-background-tertiary".to_string()).ok_or("Could not get style")?.as_str())?,
        theme_text: Color::from_hex_code(get_style(":root".to_string(), "--theme-text".to_string()).ok_or("Could not get style")?.as_str())?,
    })
}

pub fn set_theme(theme: Theme) {
    set_theme_js(theme.to_string());
}

pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn to_string(&self) -> String {
        match self {
            Theme::Dark => "dark".to_owned(),
            Theme::Light => "light".to_owned(),
        }
    }
}