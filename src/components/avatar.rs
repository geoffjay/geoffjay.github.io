// use yew::prelude::*;
use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    pub url: String,
}

#[function_component[Avatar]]
pub fn avatar(props: &AvatarProps) -> Html {
    html! {
        <div class="w-24 h-24 relative mb-4">
            <div class="group w-full h-full rounded-full overflow-hidden shadow-inner text-center bg-purple table cursor-pointer">
                <span class="hidden group-hover:table-cell text-white font-bold align-middle">
                    { "GJ" }
                </span>
                <img
                    src={props.url.clone()}
                    alt="avatar"
                    class="object-cover object-center w-full h-full visible group-hover:hidden"
                />
            </div>
        </div>
    }
}
