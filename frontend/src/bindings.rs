use wasm_bindgen::prelude::*;
use web_sys::FileList;

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
}

pub fn get_theme() -> Theme {
    let theme = get_theme_js();
    if theme == "theme-light" {
        return Theme::Light
    } else {
        return Theme::Dark
    }
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