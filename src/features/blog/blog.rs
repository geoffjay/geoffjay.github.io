use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::{function_component, html, use_state, use_effect_with, Html};

use crate::utils::markdown::render_markdown;

const MARKDOWN_URL: &str = "https://raw.githubusercontent.com/geoffjay/geoffjay.github.io/main/src/features/blog/posts/2023-05-07_rendering_markdown_in_a_component.mdx";

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

/// Fetches markdown for `url`.
async fn fetch_markdown(url: &'static str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();

    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let text = JsFuture::from(resp.text()?).await?;

    Ok(text.as_string().unwrap())
}

#[function_component(BlogMarkdown)]
fn blog_markdown() -> Html {
    let markdown = use_state(|| "".to_string());
    {
        let markdown = markdown.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match fetch_markdown(MARKDOWN_URL).await {
                    Ok(md) => markdown.set(md),
                    Err(err) => markdown.set(err.to_string()),
                }
            });
            || ()
        });
    }

    html! {
        <div>
            { render_markdown(&markdown) }
        </div>
    }
}

#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <div class="container">
            <div class="text-xl">{"Blog"}</div>
            <div class="text-lg">{"Testing loading markdown..."}</div>
            <section class="section">
                <BlogMarkdown />
            </section>
        </div>
    }
}
