# Straw

Work in progress Experimental Rust -> HTML renderer.

## Installation

Add straw to `Cargo.toml`.

```toml
[dependencies]
straw = "*"
```

## Usage

Import `Element` and `Renderable` and start creating `Element`s.

```rust
extern crate straw;

use straw::element::{Element, Renderable}
use straw::attribute::Attr;

let element = Element::new("div", vec![Attr::id("main")], vec![
  Element::new("h1", vec![], "Hello"),
  Element::new("input", vec![Attr::disabled(true)], ""),
]);

element.render(); // <div id="main"><h1>Hello</h1><input disabled></input></div>
```
