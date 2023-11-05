use yew::{function_component, html, Html};

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    html! {
        <div class="container">
            <div class="text-xl">{"Portfolio"}</div>
        </div>
    }
}
