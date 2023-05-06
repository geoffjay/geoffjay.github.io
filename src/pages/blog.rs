use yew::{function_component, html};

#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <div class="container">
            <div class="text-xl">{"Blog"}</div>
        </div>
    }
}
