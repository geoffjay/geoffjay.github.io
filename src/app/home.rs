use yew::{function_component, html, Html};

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
                            <img
                                src="https://raw.githubusercontent.com/geoffjay/geoffjay.github.io/docs/assets/images/rust.png"
                                width="150"
                            />
                        </a>
                    </div>
                    <div>
                        <a href="https://yew.rs" target="_blank">
                            <img
                                src="https://raw.githubusercontent.com/geoffjay/geoffjay.github.io/docs/assets/images/yew.png"
                                width="150"
                            />
                        </a>
                    </div>
                    <div>
                        <a href="https://tailwindcss.com" target="_blank">
                            <img
                                src="https://raw.githubusercontent.com/geoffjay/geoffjay.github.io/docs/assets/images/tailwind.png"
                                width="150"
                            />
                        </a>
                    </div>
                    <div>
                        <a href="https://animxyz.com" target="_blank">
                            <img
                                src="https://raw.githubusercontent.com/geoffjay/geoffjay.github.io/docs/assets/images/animxyz.svg"
                                width="150"
                            />
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
