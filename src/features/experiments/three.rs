use js_sys::*;
use yew::{function_component, html, use_callback, use_effect_with, use_state, use_state_eq, Html};
use yew_hooks::use_raf;

use crate::bindings::{init, render};

#[function_component(Three)]
pub fn three() -> Html {
    let initialized = use_state_eq(|| false);
    let elapsed = use_raf(5000, 100);
    let then = use_state(|| Date::now());
    let fps = use_state(|| 1000.0 / elapsed);

    let handle_init = use_callback(initialized.setter(), |_input, initialized_setter| {
        init();
        initialized_setter.set(true);
    });

    use_effect_with(elapsed, move |_| {
        if *initialized {
            let fps_interval = 1000.0 / 60.0;
            let now = Date::now();
            let dt = now - *then;

            // log::info!("elapsed: {}", elapsed);
            // log::info!("then: {}", *then.clone());
            // log::info!("now: {}", now);
            // log::info!("dt: {}", dt);
            // log::info!("fps_interval: {}", fps_interval);
            // log::info!("fps: {}", *fps.clone());

            if dt as f64 > fps_interval {
                render();
                then.set(now);
                fps.set(1000.0 / dt);
            }
        }
    });

    html! {
        <div class="container">
            <div class="text-xl">{"three.js"}</div>
            <button onclick={handle_init}>{"init"}</button>
            <div id="canvas" class="w-[800px] h-[600px]"></div>
            // <p class="text-xl">{format!("FPS: {}", *fps.clone())}</p>
        </div>
    }
}
