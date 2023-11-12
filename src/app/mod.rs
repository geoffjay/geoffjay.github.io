use yew::prelude::*;
use yew_router::prelude::*;

mod home;
use home::Home;

use crate::bindings;
use crate::components::{footer::Footer, nav::Nav};
use crate::features::{
    about::About,
    blog::{Blog, Post},
    contact::Contact,
    portfolio::Portfolio,
    resume::Resume,
};

#[derive(Clone, Debug, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/blog")]
    Blog,
    #[at("/blog/:slug")]
    Post { slug: String },
    #[at("/contact")]
    Contact,
    #[at("/portfolio")]
    Portfolio,
    #[at("/resume")]
    Resume,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes.clone() {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Blog => html! { <Blog /> },
        Route::Post { slug } => html! { <Post slug={slug} /> },
        Route::Contact => html! { <Contact /> },
        Route::Portfolio => html! { <Portfolio /> },
        Route::Resume => html! { <Resume /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let main_classes = vec![
        "flex",
        "flex-col",
        "flex-0",
        "grow",
        "p-8",
        "w-full",
        "text-gray-700",
        "bg-gray-100",
        "dark-mode:text-gray-200",
        "dark-mode:bg-gray-700",
        "justify-between",
    ];

    bindings::highlight();

    html! {
        <HashRouter>
            <div class="flex flex-col md:flex-row min-h-screen w-full bg-gray-100">
                <Nav />
                <div class={classes!(main_classes)}>
                    <main class="h-full grow flex flex-col">
                        <Switch<Route> render={switch} />
                    </main>
                    <Footer />
                </div>
            </div>
        </HashRouter>
    }
}
