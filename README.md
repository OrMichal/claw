# Claw-lib
Claw-lib is a lightweight Rust crate providing **structs representing HTML elements**, designed specifically for **WASM (WebAssembly) applications**. It allows developers to define and manipulate HTML elements in Rust, making DOM interactions more type-safe and ergonomic when compiling Rust to WebAssembly.
If you are interested, you should also check out [ClawRS](https://github.com/berbenzuel/claw) frontend/fullstack framework which uses this library as its core.

## Features
- Struct representations of common HTML elements (e.g., Div, Span, Button, etc.).
- Type-safe manipulation of element attributes, children, and events.
- Designed for seamless integration with WASM projects using wasm-bindgen or web-sys.
- Minimal and dependency-light, focusing purely on HTML structure modeling.

## Installation
Add claw-lib to your Cargo.toml
```toml
[dependencies]
claw-lib = { git = "https://github.com/OrMichal/claw", branch = "main" }
```

## Usage
Here is a simple example of using claw-lib
```rust
use claw::prelude::*;

pub struct Page;
impl Page {
    pub fn new() -> Self {
        Self { }
    }
}

impl Component for Page {
    fn name(&self) -> String {
       "index-page".to_string() 
    }

    fn render(&self) {
        
    }

    fn template(&self) -> Html {
        div([
            p("Rust index page")
        ])
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::new()
    }
}

```

