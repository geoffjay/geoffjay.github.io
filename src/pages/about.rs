use yew::{function_component, html, Html};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="container">
            <div class="text-xl">{"About"}</div>
        </div>
    }
}
