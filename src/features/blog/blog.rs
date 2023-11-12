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
              <div class="container mb-32">
                <div class="text-xl pb-6">{"Blog"}</div>
                <Link<Route> classes="text-inherit" to={Route::Post { slug: metadata.slug.into() }}>
                  <section class="section bg-gray-200 mx-4 border-2 hover:border-red rounded-lg p-4 shadow-md hover:shadow-lg">
                    <h1 class="text-xl text-gray-700 font-display">
                      {&metadata.title}
                    </h1>
                    <div class="pl-2">
                        <div class="text-md text-gray-500 italic">{&metadata.subtitle}</div>
                        <div class="text-md text-gray-500 italic">
                            {"created on "}{&metadata.date.clone().format(&fmt).unwrap_or_default()}
                        </div>
                    </div>
                  </section>
                </Link<Route>>
              </div>
            }
        })
        .collect()
}
