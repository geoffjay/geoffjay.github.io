use time::macros::date;
use yew::{function_component, include_mdx, html, Html, Properties};
use yew_router::components::Redirect;

use crate::app::Route;
use crate::components::markdown::*;
use crate::components::markdown::highlight::HighlightCode;

blog_style!();

pub fn post_1(_: &Metadata) -> Html {
    include_mdx!("src/features/blog/posts/2023-05-07_rendering_markdown_in_a_component.mdx")
}

pub struct Metadata {
    title: &'static str,
    date: time::Date,
    slug: &'static str,
    subtitle: &'static str,
    published: bool,
}

const POSTS: &[(Metadata, &dyn Fn(&Metadata) -> Html)] = &[
    (
        Metadata {
            title: "Rendering Markdown in a Component",
            date: date!(2023 - 5 - 7),
            slug: "rendering-markdown-in-a-component",
            subtitle: "A post demonstrating all supported markdown features",
            published: true,
        },
        &post_1,
    ),
];

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub slug: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let slug = props.slug.clone();
    let post_content = POSTS
        .iter()
        .find(|(meta, _)| &slug == &meta.slug)
        .map(|(meta, post)| post(meta))
        .unwrap_or(html! { <Redirect<Route> to={Route::NotFound} /> });
    html! {
      <div class="w-full md:max-w-4xl p-2">
        {post_content}
      </div>
    }
}
