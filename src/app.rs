use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{footer::Footer, nav::Nav};
use crate::pages::{about::About, home::Home, resume::Resume};

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/resume")]
    Resume,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="md:flex flex-col md:flex-row md:min-h-screen w-full">
                <Nav />
                <div class="flex flex-col w-full text-gray-700 bg-white dark-mode:text-gray-200 dark-mode:bg-gray-800 flex-shrink-0">
                    <main>
                        <Switch<Route> render={Switch::render(switch)} />
                    </main>
                    <Footer />
                </div>
            </div>
        </BrowserRouter>
    }
}

fn switch(route: &Route) -> Html {
    match route.clone() {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Resume => html! { <Resume /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
