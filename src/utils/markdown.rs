/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};
use yew::virtual_dom::{VNode, VTag, VText};
use yew::{html, Classes, Html};
// use yew_prism::Prism;

/// Adds a class to the VTag.
/// You can also provide multiple classes separated by ascii whitespaces.
///
/// Note that this has a complexity of O(n),
/// where n is the number of classes already in VTag plus
/// the number of classes to be added.
fn add_class(vtag: &mut VTag, class: impl Into<Classes>) {
    let mut classes: Classes = vtag
        .attributes
        .iter()
        .find(|(k, _)| *k == "class")
        .map(|(_, v)| Classes::from(v.to_owned()))
        .unwrap_or_default();
    classes.push(class);
    vtag.add_attribute("class", classes.to_string());
}

/// Renders a string of Markdown to HTML with the default options (footnotes
/// disabled, tables enabled).
pub fn render_markdown(src: &str) -> Html {
    let mut elems = vec![];
    let mut spine = vec![];
    // let mut codeblock = false;

    macro_rules! add_child {
        ($child:expr) => {{
            let l = spine.len();
            assert_ne!(l, 0);
            spine[l - 1].add_child($child);
        }};
    }

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);

    for ev in Parser::new_ext(src, options) {
        match ev {
            Event::Start(tag) => {
                spine.push(make_tag(tag));
            }
            Event::End(tag) => {
                // TODO: Verify stack end.
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap().clone();
                if let Tag::CodeBlock(_) = tag {
                    // codeblock = true;
                    let top_children = top.children();
                    let vnode = &top_children[0];

                    if let VNode::VText(vtext) = vnode {
                        let text = vtext.text.clone();
                        top.add_attribute("code", text);
                    }

                    let mut pre = VTag::new("pre");
                    pre.add_child(top.into());
                    pre.add_attribute("class", "p-4 m-6 border-2 rounded-sm shadow-md");
                    top = pre;
                } else if let Tag::Table(aligns) = tag {
                    if let Some(top_children) = top.children_mut() {
                        for r in top_children.iter_mut() {
                            if let VNode::VTag(ref mut vtag) = r {
                                if let Some(vtag_children) = vtag.children_mut() {
                                    for (i, c) in vtag_children.iter_mut().enumerate() {
                                        if let VNode::VTag(ref mut vtag) = c {
                                            match aligns[i] {
                                                Alignment::None => {}
                                                Alignment::Left => add_class(vtag, "text-left"),
                                                Alignment::Center => add_class(vtag, "text-center"),
                                                Alignment::Right => add_class(vtag, "text-right"),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if let Tag::TableHead = tag {
                    if let Some(top_children) = top.children_mut() {
                        for c in top_children.iter_mut() {
                            if let VNode::VTag(ref mut vtag) = c {
                                // TODO:
                                // vtag.tag = "th".into();
                                vtag.add_attribute("scope", "col");
                            }
                        }
                    }
                }
                if l == 1 {
                    elems.push(top);
                } else {
                    spine[l - 2].add_child(top.into());
                }
            }
            Event::Text(text) => add_child!(VText::new(text.to_string()).into()),
            Event::Rule => add_child!(VTag::new("hr").into()),
            Event::SoftBreak => add_child!(VText::new("\n").into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),
            _ => println!("Unknown event: {:#?}", ev),
        }
    }

    if elems.len() == 1 {
        VNode::VTag(Box::new(elems.pop().unwrap()))
    // } else if codeblock {
    //     html! {
    //         <div><Prism code="let greeting: &str = \"Hello World\";" language="rust" /></div>
    //     }
    } else {
        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

fn make_tag(t: Tag) -> VTag {
    match t {
        Tag::Paragraph => translate_paragraph(),
        Tag::Heading(n, ..) => translate_heading(n),
        Tag::BlockQuote => translate_blockquote(),
        Tag::CodeBlock(code_block_kind) => {
            let mut el = VTag::new("Prism");

            if let CodeBlockKind::Fenced(lang) = code_block_kind {
                match lang.as_ref() {
                    "c" => el.add_attribute("language", "c"),
                    "go" => el.add_attribute("language", "go"),
                    "html" => el.add_attribute("language", "html"),
                    "ruby" => el.add_attribute("language", "ruby"),
                    "rust" => el.add_attribute("language", "rust"),
                    _ => {},
                };
            }

            el
        },
        Tag::List(None) => {
            let mut el = VTag::new("ul");
            el.add_attribute("class", "list-disc my-4 px-6");
            el
        },
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(ref start)) => {
            let mut el = VTag::new("ol");
            el.add_attribute("start", start.to_string());
            el.add_attribute("class", "list-decimal my-4 px-6");
            el
        },
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => {
            let mut el = VTag::new("table");
            el.add_attribute("class", "table");
            el
        },
        Tag::TableHead => VTag::new("th"),
        Tag::TableRow => VTag::new("tr"),
        Tag::TableCell => VTag::new("td"),
        Tag::Emphasis => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "font-italic");
            el
        },
        Tag::Strong => {
            let mut el = VTag::new("span");
            el.add_attribute("class", "font-weight-bold");
            el
        },
        Tag::Link(_link_type, ref href, ref title) => {
            let mut el = VTag::new("a");
            el.add_attribute("href", href.to_string());
            let title = title.clone().into_string();
            if !title.is_empty() {
                el.add_attribute("title", title);
            }
            el
        },
        Tag::Image(_link_type, ref src, ref title) => {
            let mut el = VTag::new("img");
            el.add_attribute("src", src.to_string());
            let title = title.clone().into_string();
            if !title.is_empty() {
                el.add_attribute("title", title);
            }
            el
        },
        Tag::FootnoteDefinition(ref _footnote_id) => VTag::new("span"),
        Tag::Strikethrough => translate_strikethrough(),
    }
}

fn translate_paragraph() -> VTag {
    let mut el = VTag::new("p");
    el.add_attribute("class", "text-base/7");
    el
}

fn translate_heading(n: HeadingLevel) -> VTag {
    let mut el = VTag::new("p");
    let heading_size = match n {
        HeadingLevel::H1 => "text-6xl",
        HeadingLevel::H2 => "text-5xl",
        HeadingLevel::H3 => "text-4xl",
        HeadingLevel::H4 => "text-3xl",
        HeadingLevel::H5 => "text-2xl",
        HeadingLevel::H6 => "text-xl",
    };
    let classes = format!("py-4 {}", heading_size);
    el.add_attribute("class", classes);
    el
}

fn translate_blockquote() -> VTag {
    // let mut el = VTag::new("blockquote");
    // el.add_attribute("class", "blockquote");
    // el
    let mut el = VTag::new("blockquote");
    el.add_attribute("class", "border-l-4 border-gray-400 italic my-8 pl-8");
    el
}

fn translate_strikethrough() -> VTag {
    let mut el = VTag::new("span");
    el.add_attribute("class", "text-decoration-strikethrough");
    el
}
