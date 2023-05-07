# Rendering Markdown in a Component

I added Markdown rendering to my personal site and maybe I'll document it
properly here, but for now I need a post with all the markdown tags to be
converted for testing purposes.

**headings**

# Heading 1

## Heading 2

### Heading 3

#### Heading 4

##### Heading 5

###### Heading 6

**lists**

* item 1
* item 2
* item 3

- item 1
- item 2
- item 3

1. item 1
1. item 2
1. item 3

1. item 1
2. item 2
3. item 3

**links**

**code**

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
