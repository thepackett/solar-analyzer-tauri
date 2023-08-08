//import { emit, listen } from '@tauri-apps/api/event'

const appWindow = window.__TAURI__.window.appWindow
const invoke = window.__TAURI__.invoke
const emit = window.__TAURI__.event.emit
const listen = window.__TAURI__.event.listen
const handlersMap = new Map()
const intervalMap = new Map()


// invoke('my_custom_command_name')
// invoke('my_custom_command_name_with_argumants', { invoke_argument1: 'Hello!'})
// Commands return a promise, so you can do this
// invoke('my_custom_command_name_that_returns_a_string').then((message) => console.log(message))
// Commands can also handle result types, async, access state, and a ton more
// More details here:
// https://tauri.app/v1/guides/features/command/

//Set up listeners that will receive tauri events and emit DOM events
const unlisten = await listen('solar_parse_complete', (event) => {
    console.log("recieved solar_parse_complete event");
    document.getElementById("graph_state_holder").dispatchEvent(new CustomEvent("solar_parse_complete", {detail: event.payload}));
});

const unlisten2 = await listen("data_request_complete", (event) => {
    console.log("recieved data_request_complete event");
    document.getElementById("graph_state_holder").dispatchEvent(new CustomEvent("data_request_complete", {detail: event.payload}));
});


export function retrieveSolarData(json_string) {
    invoke('retrieve_solar_data', {graphStateRequest: json_string})
}

export function readFile(file) {
    if(!(file.type == "text/csv")){
        console.log("File type is not text/csv, skipping...");
        throw "InvalidFileType|" + file.name;
    }
    file.text()
        .then(
            //Accepted case
            (contents) => {
                //invoke function which passes data to backend
                invoke('parse_solar_data', {name: file.name, data: contents});
            },
            //Rejected case
            (err) => {
                throw "UnknownJsError|" + file.name + "|" + err;
            }
        )
}

export function setTheme(theme) {
    if(theme == "dark"){
        setToggles(".theme-switch input", true);
        document.documentElement.classList.remove("theme-dark");
        document.documentElement.classList.remove("theme-light");
        document.documentElement.classList.add("theme-dark");
    }else if(theme == "light"){
        setToggles(".theme-switch input", false);
        document.documentElement.classList.remove("theme-dark");
        document.documentElement.classList.remove("theme-light");
        document.documentElement.classList.add("theme-light");
    }
}

export function setDetectedTheme() {
    appWindow.theme().then((detected_theme) => {
        if(detected_theme == "dark") {
            setTheme("dark");
        }else if (detected_theme == "light") {
            setTheme("light");
        }
    });
}

export function setToggles(css_selector, bool){
    let elements = document.querySelectorAll(css_selector);
    for(let i=0; i < elements.length; i++){
        elements[i].checked = bool;
    }
}

export function getTheme() {
    if (document.documentElement.classList.contains("theme-light")) {
        return "theme-light";
    } else if (document.documentElement.classList.contains("theme-dark")) {
        return "theme-dark";
    }
    console.log("Error: tried to get theme but no theme was set. This should not happen. Make sure the theme is set on page load.");
    return "theme-light";
}

export function removeClasses(css_selector, classes) {
    let class_array = classes.split(' ')
        .filter(text => text.length > 0)
        .map(s => s.trim());
    let elements = document.querySelectorAll(css_selector);
    for(let i=0; i < elements.length; i++){
        for(let j=0; j< class_array.length; j++){
            elements[i].classList.remove(class_array[j]);
        }
    }
}

export function addClasses(css_selector, classes) {
    let class_array = classes.split(' ')
        .filter(text => text.length > 0)
        .map(s => s.trim());
    let elements = document.querySelectorAll(css_selector);
    for(let i=0; i < elements.length; i++){
        for(let j=0; j< class_array.length; j++){
            elements[i].classList.add(class_array[j]);
        }
    }
}

export function toggleClasses(css_selector, classes) {
    let class_array = classes.split(' ')
        .filter(text => text.length > 0)
        .map(s => s.trim());
    let elements = document.querySelectorAll(css_selector);
    for(let i=0; i < elements.length; i++){
        for(let j=0; j< class_array.length; j++){
            elements[i].classList.toggle(class_array[j]);
        }
    }
}

export function getStyle(css_selector, style) {
    let element = document.querySelector(css_selector);
    let styles = window.getComputedStyle(element);
    let result = styles.getPropertyValue(style);
    if (result.length == 0) {
        return;
    } else {
        return result;
    }
}

export function getElementOffsetHeight(css_selector) {
    let element = document.querySelector(css_selector);
    if (element == null) {
        return
    } else {
        return element.offsetHeight;
    }
}

export function getElementOffsetWidth(css_selector) {
    let element = document.querySelector(css_selector);
    if (element == null) {
        return
    } else {
        console.log(element.offsetWidth);
        return element.offsetWidth;
    }
}

export function setCanvasSize(canvasid, width, height) {
    let canvascontext = document.getElementById(canvasid);
    if(canvascontext == null) {
        console.log("Attempted to set canvas size of a canvas that does not exist.");
        return;
    }
    canvascontext.width = width;
    canvascontext.height = height;
    canvascontext.dispatchEvent(new Event("draw_" + canvasid));
}


export function resizeCanvas(canvasid, containerid) {
    let container = document.getElementById(containerid);
    if(container == null){
        return;
    }
    setCanvasSize(canvasid, container.offsetWidth, container.offsetHeight);
}

export function setupCanvasEvents(canvasid, containerid) {
    //Resize event handling
    handlersMap.set(canvasid, function(e) {
        console.log("resize event called");
        let container = document.getElementById(containerid);
        if(container == null){
            return;
        }
        setCanvasSize(canvasid, container.offsetWidth, container.offsetHeight);
    });
    window.addEventListener("resize", handlersMap.get(canvasid));

    //Redraw interval
    intervalMap.set(canvasid, window.setInterval(function () {
        let canvas = document.getElementById(canvasid);
        if(canvas == null) {
            return;
        }
        canvas.dispatchEvent(new Event("draw_" + canvasid));
    }, 33));

}

export function teardownCanvasEvents(canvasid) {
    window.removeEventListener("resize", handlersMap.get(canvasid));
    handlersMap.delete(canvasid);
    window.clearInterval(intervalMap.get(canvasid));
    intervalMap.delete(canvasid);
}