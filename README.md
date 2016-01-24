# Straw

Work in progress Experimental Rust -> HTML renderer.

## Installation

Add straw to `Cargo.toml`.

```
[dependencies]
straw = "*"
```

## Usage

Import `Element` and `Renderable` and start creating `Element`s.

```
extern crate straw;

use straw::element::{Element, Renderable}

let element = Element::new("div", Some(vec![("id", "main")]), vec![
  Element::new("h1", None, "Hello"),
  Element::new("h1", None, "World"),
]);

element.render();
```
