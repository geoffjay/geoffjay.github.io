use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="container">
            <div class="notification is-primary">
                <h1 class="title">{"[WIP] Portfolio"}</h1>
            </div>
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
