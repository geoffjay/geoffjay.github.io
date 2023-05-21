use yew::prelude::*;
use yew_router::prelude::*;

use crate::bindings;
use crate::components::{footer::Footer, nav::Nav};
use crate::pages::{
    about::About,
    blog::Blog,
    contact::Contact,
    home::Home,
    portfolio::Portfolio,
    resume::Resume,
};

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/blog")]
    Blog,
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

fn switch(route: &Route) -> Html {
    match route.clone() {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Blog => html! { <Blog /> },
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
        "p-8",
        "w-full",
        "text-gray-700",
        "bg-gray-100",
        "dark-mode:text-gray-200",
        "dark-mode:bg-gray-700",
    ];

    bindings::highlight();

    html! {
        <>
            <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/prism.min.js"></script>
            <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-c.min.js"></script>
            <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-go.min.js"></script>
            <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-ruby.min.js"></script>
            <script src="https://cdn.jsdelivr.net/npm/prismjs@1.29.0/components/prism-rust.min.js"></script>
            <BrowserRouter>
                <div class="md:flex flex-col md:flex-row md:min-h-screen w-full">
                    <Nav />
                    <div class={classes!(main_classes)}>
                        <main>
                            <Switch<Route> render={Switch::render(switch)} />
                        </main>
                        <Footer />
                    </div>
                </div>
            </BrowserRouter>
        </>
    }
}
