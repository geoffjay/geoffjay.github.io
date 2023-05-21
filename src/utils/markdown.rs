/// Original author of this code is [Nathan Ringo](https://github.com/remexre)
/// Source: https://github.com/acmumn/mentoring/blob/master/web-client/src/view/markdown.rs
use pulldown_cmark::{Alignment, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};
use yew::virtual_dom::{VNode, VTag, VText};
use yew::{html, Classes, Html};
// use log::info;

/// Adds a class to the VTag.
/// You can also provide multiple classes separated by ascii whitespaces.
///
/// Note that this has a complexity of O(n), where n is the number of classes already in VTag plus
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
            Event::Start(tag) => spine.push(start_tag(tag)),
            Event::End(tag) => {
                // TODO: Verify stack end.
                let l = spine.len();
                assert!(l >= 1);
                let mut top = spine.pop().unwrap().clone();

                if let Tag::CodeBlock(_) = tag {
                    let top_children = top.children_mut().unwrap();
                    let vnode = &mut top_children[0].clone();
                    let vtext = &top_children[1].clone();
                    top_children.truncate(0);

                    if let VNode::VTag(ref mut code) = vnode {
                        code.add_child(vtext.clone().into());
                    }

                    top_children.push(vnode.clone());
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
                                vtag.add_child(VTag::new("th").into());
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
            Event::Code(text) => {
                let mut vtag = VTag::new("code");
                vtag.add_child(VText::new(text.to_string()).into());
                vtag.add_attribute("class", "border border-gray-500 rounded-sm bg-gray-300 text-gray-800 text-xs p-1");
                add_child!(vtag.into());
            }
            Event::Html(text) => add_child!(VText::new(text.to_string()).into()),
            // Event::Rule => add_child!(VTag::new("hr").into()),
            Event::SoftBreak => add_child!(VText::new("\n").into()),
            Event::HardBreak => add_child!(VTag::new("br").into()),
            _ => println!("Unknown event: {:#?}", ev),
        }
    }

    if elems.len() == 1 {
        VNode::VTag(Box::new(elems.pop().unwrap()))
    } else {
        html! {
            <div>{ for elems.into_iter() }</div>
        }
    }
}

fn start_tag(tag: Tag) -> VTag {
    match tag {
        Tag::Paragraph => translate_paragraph(),
        Tag::Heading(n, ..) => translate_heading(n),
        Tag::BlockQuote => translate_blockquote(),
        Tag::CodeBlock(info) => translate_codeblock(info),
        Tag::List(None) => translate_unordered_list(),
        Tag::List(Some(1)) => VTag::new("ol"),
        Tag::List(Some(ref start)) => translate_ordered_list(start),
        Tag::Item => VTag::new("li"),
        Tag::Table(_) => translate_table(),
        Tag::TableHead => translate_table_head(),
        Tag::TableRow => translate_table_row(),
        Tag::TableCell => translate_table_cell(),
        Tag::Emphasis => translate_emphasis(),
        Tag::Strong => translate_strong(),
        Tag::Link(_link_type, ref href, ref title) => translate_link(href, title),
        Tag::Image(_link_type, ref src, ref title) => translate_image(src, title),
        Tag::FootnoteDefinition(ref footnote_id) => translate_footnote_definition(footnote_id),
        Tag::Strikethrough => translate_strikethrough(),
    }
}

// fn end_tag(tag: Tag) -> VTag {
//     match tag {
//     }
// }

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
    let mut el = VTag::new("blockquote");
    el.add_attribute("class", "border-l-4 border-red italic m-4 pl-4");
    el
}

fn translate_codeblock(info: CodeBlockKind) -> VTag {
    let mut el = VTag::new("pre");
    el.add_attribute("class", "p-4 m-6 border-2 rounded-sm shadow-md");

    match info {
        CodeBlockKind::Fenced(info) => {
            let lang = info.split(' ').next().unwrap();
            if lang.is_empty() {
                el.add_child(VTag::new("code").into());
            } else {
                let mut code = VTag::new("code");
                let class = format!("language-{}", lang);
                code.add_attribute("class", class);
                el.add_child(code.into());
            }
        }
        pulldown_cmark::CodeBlockKind::Indented => el.add_child(VTag::new("code").into()),
    }

    el
}

fn translate_unordered_list() -> VTag {
    let mut el = VTag::new("ul");
    el.add_attribute("class", "list-disc my-4 px-6");
    el
}

fn translate_ordered_list(start: &u64) -> VTag {
    let mut el = VTag::new("ol");
    el.add_attribute("start", start.to_string());
    el.add_attribute("class", "list-decimal my-4 px-6");
    el
}

fn translate_table() -> VTag {
    let mut el = VTag::new("table");
    el.add_attribute("class", "min-w-full divide-y divide-gray-200 border border-collapse border-gray-500");
    el
}

fn translate_table_head() -> VTag {
    let mut el = VTag::new("thead");
    el.add_attribute("scope", "col");
    el.add_attribute("class", "px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase tracking-wider border border-gray-500");
    el
}

fn translate_table_row() -> VTag {
    let mut el = VTag::new("tr");
    el.add_attribute("class", "bg-gray-200");
    el
}

fn translate_table_cell() -> VTag {
    let mut el = VTag::new("td");
    el.add_attribute("class", "px-4 py-2 whitespace-nowrap text-sm text-gray-500 border border-gray-500");
    el
}

fn translate_emphasis() -> VTag {
    let mut el = VTag::new("span");
    el.add_attribute("class", "italic");
    el
}

fn translate_strong() -> VTag {
    let mut el = VTag::new("span");
    el.add_attribute("class", "font-bold");
    el
}

fn translate_strikethrough() -> VTag {
    let mut el = VTag::new("span");
    el.add_attribute("class", "line-through");
    el
}

fn translate_link(href: &str, title: &str) -> VTag {
    let mut el = VTag::new("a");
    el.add_attribute("href", href.to_string());
    el.add_attribute("class", "text-blue hover:text-lightblue");
    if !title.is_empty() {
        el.add_attribute("title", title.to_string());
    }
    el
}

fn translate_image(src: &str, title: &str) -> VTag {
    let mut el = VTag::new("img");
    el.add_attribute("src", src.to_string());
    if !title.is_empty() {
        el.add_attribute("title", title.to_string());
    }
    el
}

fn translate_footnote_definition(footnote_id: &str) -> VTag {
    let mut el = VTag::new("span");
    el.add_attribute("id", footnote_id.to_string());
    el
}
