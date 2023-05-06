use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="container">
            <div class="text-xl">{"About"}</div>
            <section class="section">
                <p>
                    <a href="https://github.com/geoffjay">
                        {"Geoff Johnson"}
                    </a>
                </p>
            </section>
        </div>
    }
}
