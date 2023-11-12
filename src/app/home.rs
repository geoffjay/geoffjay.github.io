use yew::{function_component, html, Html};

// use crate::components::terminal::Terminal;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container grow flex flex-col h-full">
            <div class="grow mx-auto flex items-stretch">
                <p class="text-xl self-center max-w-[512px]">
                    {r#"
                        👋 Hello, I'm Geoff! I like making things with rust, go, c and sometimes python and ruby.
                        I've dabbled in home automation, image processing, and many other things. For a long
                        time I built measurement and control systems on everything from an 8-bit microcontroller
                        to full scale distributed control systems interfacing with PLCs using a variety of
                        protocols and technologies. Now I work on web applications as a full-stack developer.
                    "#}
                </p>
            </div>
            <div class="container flex-none mx-auto">
                <div class="grid grid-cols-2 md:grid-cols-4 gap-4 place-items-center">
                    <div>
                        <a href="https://rust-lang.org" target="_blank">
                            <img
                                src="https://raw.githubusercontent.com/geoffjay/geoffjay.github.io/docs/assets/images/rust.png"
                                width="64"
                            />
                        </a>
                    </div>
                    <div>
                        <a href="https://yew.rs" target="_blank">
                            <img
                                src="https://raw.githubusercontent.com/geoffjay/geoffjay.github.io/docs/assets/images/yew.png"
                                width="64"
                            />
                        </a>
                    </div>
                    <div>
                        <a href="https://tailwindcss.com" target="_blank">
                            <img
                                src="https://raw.githubusercontent.com/geoffjay/geoffjay.github.io/docs/assets/images/tailwind.png"
                                width="64"
                            />
                        </a>
                    </div>
                    <div>
                        <a href="https://animxyz.com" target="_blank">
                            <img
                                src="https://raw.githubusercontent.com/geoffjay/geoffjay.github.io/docs/assets/images/animxyz.svg"
                                width="64"
                            />
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
