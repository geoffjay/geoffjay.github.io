use time::macros::date;
use yew::Html;

mod blog;
mod post;
mod posts;

pub use blog::Blog;
pub use post::Post;

use posts::{post_1, post_2};

pub struct Metadata {
    title: &'static str,
    date: time::Date,
    slug: &'static str,
    subtitle: &'static str,
    published: bool,
}

pub const POSTS: &[(Metadata, &dyn Fn(&Metadata) -> Html)] = &[(
    Metadata {
        title: "Rendering Markdown in a Component",
        date: date!(2023 - 5 - 7),
        slug: "rendering-markdown-in-a-component",
        subtitle: "A post demonstrating rendering all supported markdown features in a Yew component using pulldown_cmark",
        published: true,
    },
    &post_1,
), (
    Metadata {
        title: "Using three.js in Yew",
        date: date!(2023 - 12 - 15),
        slug: "using-threejs-in-yew",
        subtitle: "A post demonstrating using three.js in a Yew component",
        published: true,
    },
    &post_2,
)];
