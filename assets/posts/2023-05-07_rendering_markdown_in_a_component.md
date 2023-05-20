## Rendering Markdown in a Component

I added Markdown rendering to my personal site and maybe I'll document it
properly here, but for now I need a post with all the markdown tags to be
converted for testing purposes.

### Styling Text

| Style                  | Example            | Output                                 |
| ---------------------- | ------------------ | -------------------------------------- |
| Bold                   | `** **` or `__ __` | **This is bold text**                  |
| Italic                 | `* *` or `_ _`     | _This is italic text_                  |
| Strikethrough          | `~~ ~~`            | ~~This was mistaken text~~             |
| Bold and nested italic | `** **` and `_ _`  | **This text is _extremely_ important** |
| All bold and italic    | `*** ***`          | ***All this text is important***       |
| Subscript              | `<sub></sub>`      | <sub>This is a subscript text</sub>    |
| Superscript            | `<sup></sup>`      | <sup>This is a superscript text</sup>  |

---

### Headings

# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6

---

### Lists

unordered list using `*`:

* item 1
* item 2
* item 3

Unordered list using `-`:

- item 1
- item 2
- item 3

Ordered list using `1. ... 1.`:

1. item 1
1. item 2
1. item 3

Ordered list using `1. ... n.`:

1. item 1
2. item 2
3. item 3

---

### Links

Sample [link](https://github.com/geoffjay/geoffjay.github.io)

---

### Code

#### Inline

This is some `inline code`.

#### Block

```rust
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
```
