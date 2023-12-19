use yew::prelude::*;
use yew_router::prelude::*;

use crate::bindings;
use crate::components::{footer::Footer, nav::Nav};
use crate::features::{
    about::About,
    blog::{Blog, Post},
    contact::Contact,
    experiments::{Experiments, Three, SplineEditor},
    home::Home,
    resume::Resume,
};

#[derive(Clone, Routable, PartialEq)]
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
    #[at("/experiments")]
    ExperimentsIndex,
    #[at("/experiments/*")]
    Experiments,
    #[at("/resume")]
    Resume,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum ExperimentsRoute {
    #[at("/experiments")]
    Index,
    #[at("/experiments/three")]
    Three,
    #[at("/experiments/spline-editor")]
    SplineEditor,
    #[not_found]
    #[at("/experiments/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes.clone() {
        Route::Home => html! {<Home />},
        Route::About => html! {<About />},
        Route::Blog => html! {<Blog />},
        Route::Post { slug } => html! {<Post slug={slug} />},
        Route::Contact => html! {<Contact />},
        Route::ExperimentsIndex | Route::Experiments => {
            html! {<Switch<ExperimentsRoute> render={switch_experiments} />}
        }
        Route::Resume => html! {<Resume />},
        Route::NotFound => html! {<h1>{"404"}</h1>},
    }
}

/// Experiment route switch.
///
/// Could probably do this with an :id param and single ExperimentShow route
/// and corresponding page that handles the value.
fn switch_experiments(route: ExperimentsRoute) -> Html {
    match route {
        ExperimentsRoute::Index => html! {<Experiments />},
        ExperimentsRoute::Three => html! {<Three />},
        ExperimentsRoute::SplineEditor => html! {<SplineEditor />},
        ExperimentsRoute::NotFound => html! {<Redirect<Route> to={Route::NotFound}/>},
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
        <BrowserRouter>
            <div class="flex flex-col md:flex-row min-h-screen w-full bg-gray-100">
                <Nav />
                <div class={classes!(main_classes)}>
                    <main class="h-full grow flex flex-col">
                        <Switch<Route> render={switch} />
                    </main>
                    <Footer />
                </div>
            </div>
        </BrowserRouter>
    }
}
