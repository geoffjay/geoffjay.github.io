use yew::{function_component, html, Html};
use yew_router::prelude::Link;

use crate::app::Route;
use crate::features::blog::POSTS;

#[function_component(Blog)]
pub fn blog() -> Html {
    let fmt = time::macros::format_description!("[month repr:short] [day], [year]");

    POSTS
        .iter()
        .filter(|(md, _)| option_env!("SHOW_UNPUBLISHED").is_some() || md.published)
        .map(|(metadata, _)| {
            html! {
              <div class="container">
                <div class="text-3xl">{"Blog"}</div>
                <Link<Route> classes="text-inherit" to={Route::Post { slug: metadata.slug.into() }}>
                  <section class="section">
                    <h1 class="text-2xl font-display">
                      {&metadata.title}
                    </h1>
                    <div> {&metadata.subtitle} </div>
                    <div class="text-xl"> {&metadata.date.clone().format(&fmt).unwrap_or_default()} </div>
                  </section>
                </Link<Route>>
              </div>
            }
        })
        .collect()
}
