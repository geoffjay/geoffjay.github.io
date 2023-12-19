use yew::{function_component, html, Html};
use yew_router::prelude::*;

use crate::app::ExperimentsRoute;

mod three;
mod spline_editor;

pub use three::Three;
pub use spline_editor::SplineEditor;

#[function_component(Experiments)]
pub fn experiments() -> Html {
    html! {
        <div class="container">
            <div class="text-xl">{"Experiments"}</div>
            <Link<ExperimentsRoute> to={ExperimentsRoute::Three}>
                {"three.js"}
            </Link<ExperimentsRoute>>
            <Link<ExperimentsRoute> to={ExperimentsRoute::SplineEditor}>
                {"Spline Editor"}
            </Link<ExperimentsRoute>>
        </div>
    }
}
