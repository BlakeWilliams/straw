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

let element = Element::new("div", Some(vec![("id".to_owned(), "main".to_owned())]), vec![
  Element::new("h1", None, "Hello".to_owned()),
  Element::new("h1", None, "World".to_owned()),
]);

element.render();
``
