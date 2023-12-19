use yew::{function_component, html, use_callback, use_state_eq, Html};

use crate::bindings::spline_editor::init;

#[function_component(SplineEditor)]
pub fn spline_editor() -> Html {
    let initialized = use_state_eq(|| false);

    let handle_init = use_callback(initialized.setter(), |_input, initialized_setter| {
        init();
        initialized_setter.set(true);
    });

    html! {
        <div class="container">
            <div class="text-xl">{"Spline Editor"}</div>
            <button onclick={handle_init}>{"init"}</button>
            <div id="canvas" class="w-[800px] h-[600px]"></div>
        </div>
    }
}
