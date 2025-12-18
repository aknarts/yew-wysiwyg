# yew-wysiwyg

A flexible WYSIWYG editor for the [Yew](https://yew.rs) framework with drag-and-drop widgets, serialization support, and extensible theming.

[![CI](https://github.com/aknarts/yew-wysiwyg/workflows/CI/badge.svg)](https://github.com/aknarts/yew-wysiwyg/actions)
[![crates.io](https://img.shields.io/crates/v/yew-wysiwyg.svg)](https://crates.io/crates/yew-wysiwyg)
[![docs.rs](https://docs.rs/yew-wysiwyg/badge.svg)](https://docs.rs/yew-wysiwyg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[Demo](https://aknarts.github.io/yew-wysiwyg/) | [Documentation](https://docs.rs/yew-wysiwyg) | [Examples](./yew-wysiwyg-demo)

## Features

- **Drag-and-Drop Interface**: Intuitive editor for building pages
- **Extensible Widget System**: Create custom widgets via traits
- **JSON Serialization**: Save and load layouts from databases or filesystems
- **Auto-Save**: Automatic persistence to browser localStorage
- **Keyboard Shortcuts**: Undo (Ctrl+Z) and Redo (Ctrl+Y) support
- **Undo/Redo System**: Full history tracking with 50-step memory
- **Theme-Agnostic**: Not locked into any CSS framework
- **Customizable**: Support for custom CSS and styling
- **Type-Safe**: Fully written in Rust with strong type guarantees
- **WASM-Ready**: Runs in the browser via WebAssembly

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
yew-wysiwyg = { version = "0.1", features = ["standard-widgets"] }
yew = "0.21"
```

### Basic Usage

```rust
use yew::prelude::*;
use yew_wysiwyg::Editor;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Editor />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

## Standard Widgets

The library includes standard widgets out of the box:

### Layout Containers
- **Row Container**: Arranges children horizontally
- **Column Container**: Arranges children vertically
- **Grid Container**: Responsive grid layout

### Text Widgets
- **Text**: Rich text with formatting (bold, italic, underline)
- **Heading**: H1-H6 heading elements
- **Paragraph**: Paragraph blocks

## Custom Widgets

Create your own widgets by implementing the `Widget` trait:

```rust
use yew::prelude::*;
use yew_wysiwyg::core::widget::{Widget, WidgetConfig, WidgetProps};

#[derive(Default)]
pub struct MyCustomWidget;

impl Widget for MyCustomWidget {
    fn widget_type(&self) -> &'static str {
        "my.custom.widget"
    }

    fn display_name(&self) -> &'static str {
        "My Custom Widget"
    }

    fn description(&self) -> &'static str {
        "A custom widget for my application"
    }

    fn render(&self, props: &WidgetProps) -> Html {
        html! {
            <div>{ "My custom widget content" }</div>
        }
    }
}
```

Register your widget:

```rust
use yew_wysiwyg::WidgetRegistry;

let mut registry = WidgetRegistry::new();
registry.register(MyCustomWidget::factory())?;
```

## Serialization

Save and load layouts as JSON:

```rust
use yew_wysiwyg::Layout;

// Create a layout
let mut layout = Layout::new();

// ... add widgets ...

// Serialize to JSON
let json = layout.to_json()?;

// Save to database or file
save_to_storage(&json)?;

// Later, load from storage
let loaded_json = load_from_storage()?;
let layout = Layout::from_json(&loaded_json)?;
```

## Theming

Customize the editor appearance:

```rust
use yew_wysiwyg::core::theme::{Theme, ThemeConfig};
use std::rc::Rc;

let theme = ThemeConfig::new("my-theme")
    .with_variable("--wysiwyg-primary", "#ff6b6b")
    .with_variable("--wysiwyg-background", "#f8f9fa")
    .with_custom_css("
        .wysiwyg-canvas {
            font-family: 'Inter', sans-serif;
        }
    ");

html! {
    <Editor theme={Rc::new(theme)} />
}
```

## Advanced Usage

### Custom Configuration UI

Widgets can provide custom configuration interfaces:

```rust
impl Widget for MyWidget {
    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let on_input = {
            let config = config.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config.clone();
                new_config.set_property("value", serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        html! {
            <input oninput={on_input} />
        }
    }
}
```

### Layout Change Callbacks

React to layout changes:

```rust
let on_layout_change = Callback::from(|layout: Layout| {
    // Auto-save to backend
    save_layout(layout);
});

html! {
    <Editor on_layout_change={on_layout_change} />
}
```

## Development

### Prerequisites

- Rust 1.70+
- Trunk (for building the demo)

```bash
cargo install trunk
```

### Building the Library

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running the Demo

```bash
cd yew-wysiwyg-demo
trunk serve
```

Then open http://localhost:8080 in your browser.

### Linting

```bash
cargo clippy --all-features -- -D warnings
cargo fmt --all -- --check
```

## Architecture

The project is organized as follows:

```
yew-wysiwyg/
├── yew-wysiwyg/          # Core library
│   ├── src/
│   │   ├── core/         # Core traits and types
│   │   │   ├── widget.rs    # Widget trait and related types
│   │   │   ├── theme.rs     # Theme system
│   │   │   └── registry.rs  # Widget registry
│   │   ├── editor/       # Editor component
│   │   │   ├── canvas.rs    # Canvas for rendering widgets
│   │   │   ├── palette.rs   # Widget palette
│   │   │   └── toolbar.rs   # Editor toolbar
│   │   ├── widgets/      # Standard widgets
│   │   │   ├── container.rs # Layout containers
│   │   │   └── text.rs      # Text widgets
│   │   ├── serialization.rs # JSON serialization
│   │   └── error.rs      # Error types
│   └── Cargo.toml
└── yew-wysiwyg-demo/     # Demo application
    ├── src/
    ├── index.html
    └── Cargo.toml
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [x] Undo/redo functionality
- [x] Auto-save to localStorage
- [x] Keyboard shortcuts
- [ ] More standard widgets (images, videos, forms)
- [ ] Widget templates and presets
- [ ] Multi-user collaboration support
- [ ] Plugin system for third-party widgets
- [ ] Accessibility improvements
- [ ] Mobile touch support
- [ ] Advanced text editor (CodeMirror/Monaco integration)
- [ ] Media library integration
- [ ] Version history and snapshots

## Acknowledgments

Built with:
- [Yew](https://yew.rs) - A modern Rust framework for creating web apps
- [serde](https://serde.rs) - Serialization framework
- [uuid](https://github.com/uuid-rs/uuid) - UUID generation

## Support

If you encounter any issues or have questions:
- Open an issue on [GitHub](https://github.com/aknarts/yew-wysiwyg/issues)
- Check the [documentation](https://docs.rs/yew-wysiwyg)
- See the [examples](./yew-wysiwyg-demo)
