use yew::{function_component, html, Html};
use yew_router::prelude::*;

use crate::app::ExperimentsRoute;

mod three;

pub use three::Three;

#[function_component(Experiments)]
pub fn experiments() -> Html {
    html! {
        <div class="container">
            <div class="text-xl">{"Experiments"}</div>
            <Link<ExperimentsRoute> to={ExperimentsRoute::Three}>
                {"three.js"}
            </Link<ExperimentsRoute>>
        </div>
    }
}
