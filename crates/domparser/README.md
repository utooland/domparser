# domparser

A super fast html parser and manipulator written in rust.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
domparser = "0.0.5"
```

## Usage

```rust
use domparser::parse;

fn main() {
    let html = r#"<div id="foo" class="bar">hello <span>world</span></div>"#;
    let root = parse(html.to_string());

    let div = root.select("div".to_string()).unwrap();
    println!("{}", div.get_attribute("id".to_string()).unwrap()); // "foo"
    println!("{}", div.text()); // "hello world"
    
    div.set_attribute("title".to_string(), "my-title".to_string());
    println!("{}", div.outer_html()); 
    // <div id="foo" class="bar" title="my-title">hello <span>world</span></div>
}
```

## Features

- Parse HTML string to DOM
- Select nodes with CSS selectors
- Manipulate attributes and text
- Serialize DOM back to HTML
