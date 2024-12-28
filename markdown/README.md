## Comrak

**Comrak** is a Rust library for parsing and rendering Markdown, compliant with the [**CommonMark**](https://commonmark.org/)
specification and [**GFM**](https://github.github.com/gfm/)(GitHub Flavored Markdown). It is a 1:1 Rust port of GitHub's
`cmark-gfm`, ensuring compatibility with upstream changes. Key faetures include:

-   **Markdown to HTML conversion**: Use `comrak::markdown_to_html` for simple conversions
-   **AST manipulation**: Parse Markdown into an Abstract Syntax Tree (AST) for custom transformations
-   **GFM extensions**: Supports tables, strikethrough, autolinks and more
-   **Safe-by-default**: Scrubs raw HTML and dangerous links unless explicitly allowed
-   **Performance**: While slightly slower than some alternatives (e.g., pulldown-cmark), it remains efficient and
    production-ready, used in tools like `docs.rs`

```rust
use comrak::{markdown_to_html, ComrakOPtions};
let html = markdown_to_html("Hello, **world**!", &ComrakOptions::default());
assert_eq!(html, "<p>Hello, <strong>world</strong>!</p>\n")
```
