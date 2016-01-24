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

let element = Element::new("div", vec![
  Element::new("h1", "Hello".to_owned()),
  Element::new("h1", "World".to_owned()),
]);

element.render();
``
