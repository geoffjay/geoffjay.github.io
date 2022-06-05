use yew::prelude::*;

// use crate::components::terminal::Terminal;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <div class="notification is-primary">
                <h1 class="title">{"[WIP] Personal site"}</h1>
            </div>
            <div class="notification is-secondary">
                <h3 class="subtitle">{"Built using"}</h3>
            </div>
            <section class="section">
                // <Terminal />
                <ul>
                    <li>
                        <a href="https://rust-lang.org" target="_blank">
                            <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/106px-Rust_programming_language_black_logo.svg.png" width="150" />
                        </a>
                    </li>
                    <li>
                        <a href="https://yew.rs" target="_blank">
                            <img src="https://yew.rs/img/logo.png" width="150" />
                        </a>
                    </li>
                    <li>
                        <a href="https://tailwindcss.com" target="_blank">
                            <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Tailwind_CSS_Logo.svg/240px-Tailwind_CSS_Logo.svg.png" width="150" />
                        </a>
                    </li>
                </ul>
            </section>
        </div>
    }
}
