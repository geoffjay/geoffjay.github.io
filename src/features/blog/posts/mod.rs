use yew::{include_mdx, Html};

use crate::components::markdown::*;
use crate::components::markdown::highlight::HighlightCode;
use crate::features::blog::Metadata;

blog_style!();

pub fn post_1(_: &Metadata) -> Html {
    include_mdx!("src/features/blog/posts/2023-05-07_rendering_markdown_in_a_component.mdx")
}
