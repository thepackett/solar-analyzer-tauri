use yew::prelude::*;

use crate::component::visual::general_props::Props;


#[function_component]
pub fn CloseButton(props: &Props) -> Html {
    html!(
    <svg version="1.1" id="Layer_1" viewBox="0 0 24 24" space="preserve" class={props.class.to_string()}>
        <path d="M7 17L16.8995 7.10051" stroke="#000000" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M7 7.00001L16.8995 16.8995" stroke="#000000" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
    )
}
