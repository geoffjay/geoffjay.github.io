use web_sys::Element;
use yew::{prelude::*, virtual_dom::VNode};

// Credit to https://github.com/kcking/implfuture.dev for this code.

#[function_component]
pub fn HighlightCode(c: &super::ChildProps) -> Html {
    let code_ref = use_state_eq(|| NodeRef::default());
    let mut code_tag = c.children.iter().next().unwrap().clone();
    match &mut code_tag {
        VNode::VTag(t) => t.node_ref = (*code_ref).clone(),
        _ => {}
    };

    use_effect_with(c.children.clone(), move |_| {
        let element = code_ref.cast::<Element>().unwrap();
        prism::highlightElement(element.clone());
        move || {
            element
                .closest(".codecontainer")
                .ok()
                .flatten()
                .map(|e| e.remove());
        }
    });

    html! {
        <div class="codecontainer">
            <pre class="overflow-auto m-4 p-6 bg-gray-300/5 rounded">
                {code_tag}
            </pre>
        </div>
    }
}

mod prism {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = Prism)]
        pub fn highlightElement(element: web_sys::Element);
    }
}
