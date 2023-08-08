use wasm_bindgen::prelude::*;
use web_sys::File;

use crate::component::visual::theme_data::{ThemeData, Color};

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    //The Error variant of this will be a JsValue that will be converted to a rust error
    #[wasm_bindgen(js_name = readFile, catch)]
    fn read_file_js(file: File) -> Result<(),JsValue>;

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

#[derive(thiserror::Error, Debug, Clone)]
pub enum ReadFileError {
    #[error("Invalid file type for {0}, expected .csv, found .{1}")]
    InvalidFileType(String, String),
    #[error("Failed to read file {0} with JS error {1}.")]
    UnknownJsError(String, String),
    #[error("Failed to read file for unknown reasons.")]
    UnknownError
}

impl ReadFileError {
    fn get_error_from_jsvalue(error: JsValue) -> ReadFileError {
        let error_string = error.as_string().expect("All errors are returned as strings.");
        let split_string = error_string.split('|').collect::<Vec<_>>();
        match *split_string.first().expect("All errors returned by read_file are non-empty") {
            "InvalidFileType" => {
                let name = split_string.get(1).expect("All errors of this type will be split into two parts when split over |").to_string();
                let file_type = name.split('.').skip(1).collect::<String>();
                ReadFileError::InvalidFileType(name, file_type)
            },
            "UnknownJsError" => {
                let name = split_string.get(1).expect("All errors of this type will be split into three parts when split over |").to_string();
                let error = split_string.get(2).expect("All errors of this type will be split into three parts when split over |").to_string();
                ReadFileError::UnknownJsError(name, error)
            },
            _ => {
                //No error matched. This should not be able to happen.
                ReadFileError::UnknownError
            }
        }
    }
}

pub fn read_file(file: File) -> Result<(), ReadFileError> {
    match read_file_js(file) {
        Ok(_) => Ok(()),
        Err(e) => Err(ReadFileError::get_error_from_jsvalue(e))
    }

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
        theme_graph_background: Color::from_hex_code(get_style(":root".to_string(), "--theme-graph-background".to_string()).ok_or("Could not get style")?.as_str())?,
        theme_graph_mesh_light: Color::from_hex_code(get_style(":root".to_string(), "--theme-graph-mesh-light".to_string()).ok_or("Could not get style")?.as_str())?,
        theme_graph_mesh_dark: Color::from_hex_code(get_style(":root".to_string(), "--theme-graph-mesh-dark".to_string()).ok_or("Could not get style")?.as_str())?,
        theme_graph_border: Color::from_hex_code(get_style(":root".to_string(), "--theme-graph-border".to_string()).ok_or("Could not get style")?.as_str())?,
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