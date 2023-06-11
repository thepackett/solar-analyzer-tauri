const appWindow = window.__TAURI__.window.appWindow
const invoke = window.__TAURI__.invoke

// invoke('my_custom_command_name')
// invoke('my_custom_command_name_with_argumants', { invoke_argument1: 'Hello!'})
// Commands return a promise, so you can do this
// invoke('my_custom_command_name_that_returns_a_string').then((message) => console.log(message))
// Commands can also handle result types, async, access state, and a ton more
// More details here:
// https://tauri.app/v1/guides/features/command/

export async function invokeHello(name) {
    return await invoke("Hello", {name: name});
}


export function readFiles(file_list) {
    //console.log(file_list);
    //Possibly an unnecessary check since it is checked in rust.
    if(!(file_list instanceof FileList)) {
        console.log("Passed in object is not an instance of a FileList");
        return;
    }
    let data_storage_array = new Array();
    for (let i = 0; i < file_list.length; i++) {
        let file = file_list.item(i);
        if(!(file.type == "text/csv")){
            console.log("File type is not text/csv, skipping...");
            continue;
        }
        file.text()
            .then(
                //Accepted case
                (contents) => {
                    //invoke function which passes data to backend
                    invoke('parse_solar_data', {data: contents})
                        .then((data_storage) => {
                            console.log("invoked function returned ok");
                            console.log(data_storage);
                            data_storage_array.push(data_storage);
                        })
                        .catch((err) => {
                            console.log("invoked function returned error");
                        });
                },
                //Rejected case
                (err) => {
                    console.log("Could not get contents of file. Error: " + err);
                }
            )
    }
    return data_storage_array;
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
    console.log("clicked");
    let class_array = classes.split(' ')
        .filter(text => text.length > 0)
        .map(s => s.trim());
    let elements = document.querySelectorAll(css_selector);
    for(let i=0; i < elements.length; i++){
        for(let j=0; j< class_array.length; j++){
            console.log("Removing class " + class_array[j] + " from element " + elements[i]);
            elements[i].classList.remove(class_array[j]);
        }
    }
}

export function addClasses(css_selector, classes) {
    console.log("clicked");
    let class_array = classes.split(' ')
        .filter(text => text.length > 0)
        .map(s => s.trim());
    let elements = document.querySelectorAll(css_selector);
    for(let i=0; i < elements.length; i++){
        for(let j=0; j< class_array.length; j++){
            console.log("Adding class " + class_array[j] + " to element " + elements[i]);
            elements[i].classList.add(class_array[j]);
        }
    }
}

export function toggleClasses(css_selector, classes) {
    console.log("clicked");
    let class_array = classes.split(' ')
        .filter(text => text.length > 0)
        .map(s => s.trim());
    let elements = document.querySelectorAll(css_selector);
    for(let i=0; i < elements.length; i++){
        for(let j=0; j< class_array.length; j++){
            console.log("Toggling class " + class_array[j] + " in element " + elements[i]);
            elements[i].classList.toggle(class_array[j]);
        }
    }
}