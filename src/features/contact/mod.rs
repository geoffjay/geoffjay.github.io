use yew::{function_component, html, Html};

use crate::components::terminal::Terminal;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div class="grow flex flex-col h-full">
            <div class="text-xl">{"Contact"}</div>
            <div class="grow mx-auto flex items-stretch">
                <div class="self-center min-w-[512px] max-w-[1024px]">
                    <Terminal command="cat TODO">
                        <p class="text-md text-gray-300 py-2">
                            {r#"
                                This page is a work in progress...
                            "#}
                        </p>
                    </Terminal>
                </div>
            </div>
        </div>
    }
}
