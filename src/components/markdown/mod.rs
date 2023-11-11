use std::borrow::Borrow;
use yew::{function_component, html, Children, Html, Properties};

pub mod highlight;

use highlight::HighlightCode;

macro_rules! blog_style {
    () => {
        yew::mdx_style!(
            h1: MyH1,
            h2: MyH2,
            h3: MyH3,
            blockquote: MyBlockquote,
            pre: HighlightCode,
            p: MyP,
            li: MyLi,
            ul: MyUl,
            code: MyCode,
        );
    };
}

pub(crate) use blog_style;

#[derive(PartialEq, Properties)]
pub struct ChildProps {
    #[prop_or_default]
    pub children: Children,
}

const HEADER_LINK_LEN: usize = 20;

#[function_component]
pub fn MyH1(c: &ChildProps) -> Html {
    let mut tag = String::new();

    for c in c.children.iter() {
        match c {
            yew::virtual_dom::VNode::VText(t) => {
                tag += &t.text.to_string();
            }
            _ => (),
        };
    }

    tag = tag.replace(" ", "-").to_lowercase();
    tag.truncate(HEADER_LINK_LEN);

    log::info!("tag: {}", tag);

    html! {
      <h1 id={tag.clone()} class="text-4xl pt-10 pb-6">
        <a class="text-inherit" href={format!("#{tag}")}>
          {c.children.clone()}
        </a>
      </h1>
    }
}

#[function_component]
pub fn MyH2(c: &ChildProps) -> Html {
    let mut tag = String::new();

    for c in c.children.iter() {
        match c {
            yew::virtual_dom::VNode::VText(t) => {
                tag += &t.text.to_string();
            }
            _ => (),
        };
    }

    tag = tag.replace(" ", "-").to_lowercase();
    tag.truncate(HEADER_LINK_LEN);

    log::info!("tag: {}", tag);

    html! {
      <h2 id={tag.clone()} class="text-2xl pt-8 pb-4">
        <a class="text-inherit" href={format!("#{tag}")}>
          {c.children.clone()}
        </a>
      </h2>
    }
}

#[function_component]
pub fn MyH3(c: &ChildProps) -> Html {
    let tag = children_to_slug(c.children.iter());

    log::info!("tag: {}", tag);

    html! {
      <h3 id={tag.clone()} class="text-xl pt-6 pb-2">
        <a class="text-inherit" href={format!("#{tag}")}>
          {c.children.clone()}
        </a>
      </h3>
    }
}

fn children_to_slug(c: impl IntoIterator<Item = Html>) -> String {
    let mut out = children_to_string(c);
    out.truncate(HEADER_LINK_LEN);
    out
}

fn children_to_string<H: Borrow<Html>>(c: impl IntoIterator<Item = H>) -> String {
    let mut out = String::new();

    for c in c.into_iter() {
        match c.borrow() {
            yew::virtual_dom::VNode::VText(t) => {
                out += &t.text.to_string();
            }
            _ => (),
        };
    }

    out = out.replace(" ", "-").to_lowercase();

    out
}

#[function_component]
pub fn MyPre(c: &ChildProps) -> Html {
    html! {
      <pre class="overflow-auto m-4 p-6 bg-gray-300/5 rounded">
          <HighlightCode>{c.children.clone()}</HighlightCode>
      </pre>
    }
}

#[function_component]
pub fn MyBlockquote(c: &ChildProps) -> Html {
    html! {
      <blockquote class="text-black/70 dark:text-white/50 border-l-8 px-2 my-2 italic">
        {c.children.clone()}
      </blockquote>
    }
}

#[function_component]
pub fn MyP(c: &ChildProps) -> Html {
    html! {
      <p class="py-2 text-lg">
        {c.children.clone()}
      </p>
    }
}

#[function_component]
pub fn MyUl(c: &ChildProps) -> Html {
    html! {
      <div class="px-4">{c.children.clone()}</div>
    }
}

#[function_component]
pub fn MyLi(c: &ChildProps) -> Html {
    html! {
      <p class="py-1">{" - "}{c.children.clone()}</p>
    }
}

#[function_component]
pub fn MyCode(c: &ChildProps) -> Html {
    html! {
      <code class="bg-gray-300/40 dark:bg-gray-300/20 px-1 rounded">
        {c.children.clone()}
      </code>
    }
}
