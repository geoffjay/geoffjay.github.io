use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let navbar_active = use_state_eq(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };

    let active_class = if !*navbar_active { "is-active" } else { "" };

    html! {
        <nav class="navbar has-shadow" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <h1 class="navbar-item is-size-3">{ "Boxo" }</h1>
                <button
                    class={classes!("navbar-burger", "burger", active_class)}
                    aria-label="menu" aria-expanded="false"
                    onclick={toggle_navbar}
                >
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </button>
            </div>
            <div class={classes!("navbar-menu", active_class)}>
                <div class="navbar-start">
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                        { "Home" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::About}>
                        { "About" }
                    </Link<Route>>

                    // <div class="navbar-item has-dropdown is-hoverable">
                    //     <div class="navbar-link">
                    //         { "More" }
                    //     </div>
                    //     <div class="navbar-dropdown">
                    //         <Link<Route> classes={classes!("navbar-item")} to={Route::Assets}>
                    //             { "Meet the authors" }
                    //         </Link<Route>>
                    //     </div>
                    // </div>
                </div>
            </div>
        </nav>
    }
}