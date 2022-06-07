use yew::prelude::*;

// use crate::components::terminal::Terminal;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <div class="text-xl">{"Personal site"}</div>
            <div class="text-lg">{"Built using"}</div>
            <div class="container mx-auto">
                <div class="grid grid-cols-2 gap-4 place-items-center">
                    <div>
                        <a href="https://rust-lang.org" target="_blank">
                            <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/106px-Rust_programming_language_black_logo.svg.png" width="150" />
                        </a>
                    </div>
                    <div>
                        <a href="https://yew.rs" target="_blank">
                            <img src="https://yew.rs/img/logo.png" width="150" />
                        </a>
                    </div>
                    <div>
                        <a href="https://tailwindcss.com" target="_blank">
                            <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Tailwind_CSS_Logo.svg/240px-Tailwind_CSS_Logo.svg.png" width="150" />
                        </a>
                    </div>
                    <div>
                        <a href="https://animxyz.com" target="_blank">
                            <img src="https://raw.githubusercontent.com/ingram-projects/animxyz/master/docs/src/assets/images/animxyz-logo.svg" width="150" />
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
