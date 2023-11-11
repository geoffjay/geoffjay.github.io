use time::macros::date;
use yew::Html;

mod blog;
mod post;
mod posts;

pub use blog::Blog;
pub use post::Post;

use posts::post_1;

pub struct Metadata {
    title: &'static str,
    date: time::Date,
    slug: &'static str,
    subtitle: &'static str,
    published: bool,
}

pub const POSTS: &[(Metadata, &dyn Fn(&Metadata) -> Html)] = &[
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
