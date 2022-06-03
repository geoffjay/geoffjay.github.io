use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="container">
            <div class="notification is-primary">
                <h1 class="title">{"Boxo"}</h1>
            </div>
            <section class="section">
                <p>{"Boxo Website "}
                    <a href="https://github.com/geoffjay/boxo/blob/master/LICENSE">
                        {"MIT licensed"}
                    </a>
                </p>
            </section>
        </div>
    }
}
