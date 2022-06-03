use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <div class="notification is-primary">
                <h1 class="title">{"Portfolio"}</h1>
            </div>
            <div class="notification is-secondary">
                <h2 class="subtitle">{"Built using"}</h2>
            </div>
            <section class="section">
                <ul>
                    <li>
                        <a href="https://rust-lang.org" target="_blank">
                            {"rust.org"}
                        </a>
                        {" : Rust language"}
                    </li>
                    <li>
                        <a href="https://yew.rs" target="_blank">
                            {"yew.rs"}
                        </a>
                        {" : Rust WASM frontend framework"}
                    </li>
                    <li>
                        <a href="https://tailwindcss.com" target="_blank">
                            {"tailwindcss.com"}
                        </a>
                        {" : Tailwind CSS framework"}
                    </li>
                </ul>
            </section>
        </div>
    }
}
