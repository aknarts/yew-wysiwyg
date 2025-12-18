# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

yew-wysiwyg is a WYSIWYG editor library for the Yew WebAssembly framework. It provides a drag-and-drop interface for building pages with customizable widgets, JSON serialization for persistence, and an extensible theming system. The project is designed to be CSS framework agnostic.

## Commands

### Building and Testing
```bash
# Build the entire workspace
cargo build --workspace --all-features

# Run all tests (including library and doctests)
cargo test --workspace --all-features

# Run specific test
cargo test --package yew-wysiwyg test_layout_serialization

# Format code
cargo fmt --all

# Lint code
cargo clippy --workspace --all-features -- -D warnings
```

### Demo Development
```bash
# IMPORTANT: DO NOT run trunk serve during development
# The user runs trunk serve separately to see changes in real-time
# Use trunk build to verify compilation only

# Verify demo builds correctly (use this for testing)
cd yew-wysiwyg-demo
trunk build

# Build demo for production
trunk build --release

# Note: trunk serve runs at http://localhost:8080 (user manages this separately)
```

### WASM Target
```bash
# Add WASM target if not already installed
rustup target add wasm32-unknown-unknown
```

## Architecture

### Core Design Principles

1. **Trait-Based Extensibility**: The system is built around three core traits that enable plugin-style architecture:
   - `Widget` trait for creating custom widgets
   - `Theme` trait for custom theming
   - `WidgetFactory` trait for widget instantiation

2. **Separation of State and Rendering**:
   - `Layout` and `SerializedLayout` handle state and persistence
   - `Editor` component orchestrates UI rendering
   - Widget implementations are stateless and receive props

3. **Type-Safe Serialization**: UUIDs identify widget instances, configurations stored as `HashMap<String, serde_json::Value>` for flexibility while maintaining type safety at boundaries

### Key Components

**Core Layer** (`yew-wysiwyg/src/core/`):
- `widget.rs`: Defines `Widget`, `WidgetProps`, `WidgetConfig`, and `WidgetFactory` traits. WidgetConfig stores widget-specific properties, CSS classes, and inline styles.
- `registry.rs`: `WidgetRegistry` manages available widget types and creates instances. Uses `IndexMap` to maintain insertion order for consistent widget palette display. Custom `PartialEq` implementation compares registries by widget type keys only (not factory closures).
- `theme.rs`: `Theme` trait and `ThemeConfig` for CSS variables, global classes, and custom CSS injection.

**Serialization Layer** (`yew-wysiwyg/src/serialization.rs`):
- `SerializedLayout`: JSON-serializable representation with version, root nodes, node HashMap, and metadata.
- `LayoutNode`: Individual node with WidgetConfig, children IDs, parent ID, and metadata.
- `Layout`: In-memory wrapper around SerializedLayout with validation and tree manipulation methods including:
  - `add_root_widget()`, `add_child_widget()` - Add widgets to layout
  - `remove_widget()` - Remove widget and all descendants
  - `move_widget_up()`, `move_widget_down()` - Reorder widgets within their parent's children or root list
- All layouts are validated on deserialization to ensure referential integrity.

**Editor Layer** (`yew-wysiwyg/src/editor/`):
- `mod.rs`: Main `Editor` component that accepts optional registry, theme, initial layout, and callbacks. Uses `use_memo` for registry to prevent recreation on every render. Orchestrates palette (left), canvas (center), and config panel (right). Implements:
  - **Undo/Redo System**: Tracks up to 50 history states with keyboard shortcuts (Ctrl+Z/Cmd+Z for undo, Ctrl+Y/Cmd+Y/Ctrl+Shift+Z for redo)
  - **Auto-Save**: Automatically saves layout to browser localStorage on every change using key "yew-wysiwyg-autosave"
  - **Auto-Load**: Loads saved layout from localStorage on initialization if no initial_layout prop provided
  - **Edit/Preview Modes**: Toggle between editing and preview modes
- `canvas.rs`: Renders the editable layout recursively. Handles widget selection, deletion, move up/down, and config changes. Each widget wrapper includes control buttons (up, down, delete) when selected.
- `palette.rs`: Left sidebar displaying available widgets from registry. Shows widget icons/descriptions, emits widget configs on click to add to canvas.
- `config_panel.rs`: Right sidebar showing selected widget's configuration UI. Displays widget name, description, properties editor (via `render_config_ui()`), and widget info (type, ID). Shows "No Widget Selected" placeholder when nothing is selected.
- `toolbar.rs`: Top bar with import/export modal and action buttons. Includes:
  - Undo/Redo buttons with disabled states
  - Import/Export modal for JSON manipulation
  - Clear button with confirmation modal to reset editor and localStorage
  - Edit/Preview mode toggle

**Standard Widgets** (`yew-wysiwyg/src/widgets/`):
- Container widgets (Row, Column, Grid, Card) support children via `can_have_children()` returning true.
- Text widgets (Heading, Paragraph, Text) store content and formatting options in WidgetConfig properties.
- Form widgets (TextInput, TextArea, Checkbox) provide configurable input fields with labels and validation support.
- Interactive widgets (Button, Link, Image) support user interactions and content display.
- Layout widgets (Spacer, Divider) provide visual spacing and separation.
- Each widget provides `render_config_ui()` for property editing in the editor.
- Helper functions `build_style()` and `build_class()` convert WidgetConfig to HTML attributes.

### Adding New Widgets

Standard widget pattern:
1. Create struct implementing `Default`
2. Implement `Widget` trait with required methods
3. Provide `factory()` static method returning `SimpleWidgetFactory<Self>`
4. Register in `WidgetRegistry::with_standard_widgets()` if it's a standard widget
5. Implement `render_config_ui()` if the widget has configurable properties

Widget type naming convention: Use dot notation for categorization (e.g., "container.row", "text.heading").

### Editor Component Flow

1. `Editor` initializes with optional props (registry, theme, initial layout)
2. If no registry provided, creates default with standard widgets (when feature enabled)
3. Layout stored in `use_state`, selected widget ID in separate state
4. Callbacks propagate changes up: `on_add_widget`, `on_config_change`, `on_widget_delete`
5. Canvas recursively renders widgets using `render_widget_node()`
6. Widget selection managed via callbacks, shows delete button and config UI when selected

### Serialization Strategy

- UUIDs (v4) uniquely identify widget instances
- Parent-child relationships stored bidirectionally (children Vec in parent, parent Option in child)
- Root widgets stored separately in `root_nodes` Vec
- Validation ensures no orphaned references before allowing deserialization
- JSON format includes version field for future migration support

### Theme System

- CSS variables prefixed with `--wysiwyg-*` for isolation
- Default theme provides standard color palette
- Theme config injected as inline styles on editor root div
- Widgets can access theme through CSS variables, no direct theme prop needed
- Custom CSS can be injected via `ThemeConfig.custom_css`

## Web-sys Features

When adding new HTML interactions, remember to add web-sys features to `yew-wysiwyg/Cargo.toml`. Currently enabled:
- HtmlElement, DragEvent, DataTransfer, Element, MouseEvent, Window, Document
- HtmlSelectElement, HtmlTextAreaElement (for form controls)
- CssStyleDeclaration, DomTokenList (for style/class manipulation)

## Testing Patterns

- Unit tests in same file as implementation (e.g., `serialization.rs` has layout tests)
- Integration tests would go in `yew-wysiwyg/tests/`
- Demo app in `yew-wysiwyg-demo` serves as manual integration test
- Doctests in lib.rs demonstrate public API usage

## Common Pitfalls

1. **PartialEq Requirements**: Yew Properties must implement PartialEq. For trait objects like `Theme`, implement manually or store in non-compared fields.

2. **Callback Cloning**: When using callbacks in closures that will be called multiple times, clone before moving into the closure to avoid borrow checker issues.

3. **Registry Memoization**: `use_memo` requires deps argument first (changed in Yew 0.21), then closure.

4. **Widget Lifecycle**: Widgets are stateless - all state lives in `WidgetConfig` which must be serializable.

5. **CSS String Building**: Use `join(" ")` for CSS classes rather than `Classes::from_iter()` to avoid lifetime issues with borrowed data.

6. **Rendering HTML from Strings**: DO NOT use `dangerously_set_inner_html` attribute - it doesn't work in Yew. Instead, use `Html::from_html_unchecked()` to create Html from raw HTML strings. Example:
   ```rust
   let html_string = markdown_to_html(content);
   let inner_html = Html::from_html_unchecked(html_string.into());
   html! { <div>{ inner_html }</div> }
   ```

## Feature Flags

- `standard-widgets`: Includes built-in container and text widgets. Enabled by default. When disabled, users must provide all widgets via registry.

## CI/CD

- `.github/workflows/ci.yml`: Runs tests, clippy, and formatting checks
- `.github/workflows/deploy.yml`: Builds and deploys demo to GitHub Pages
- `.github/workflows/release.yml`: Publishes to crates.io on version tags
- `.github/workflows/wasm-size.yml`: Tracks WASM bundle size on PRs
