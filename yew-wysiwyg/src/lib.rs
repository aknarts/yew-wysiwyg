//! # yew-wysiwyg
//!
//! A flexible WYSIWYG editor for the Yew framework with drag-and-drop widgets,
//! serialization support, and extensible theming.
//!
//! ## Features
//!
//! - Drag-and-drop interface for building pages
//! - Extensible widget system via traits
//! - JSON serialization/deserialization for layouts
//! - Theme-agnostic design
//! - Support for custom CSS and JavaScript
//!
//! ## Example
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use yew_wysiwyg::Editor;
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Editor />
//!     }
//! }
//! ```

pub mod core;
pub mod editor;
pub mod error;
pub mod serialization;

#[cfg(feature = "standard-widgets")]
pub mod widgets;

// Re-exports
pub use crate::core::{
    registry::WidgetRegistry,
    theme::{Theme, ThemeConfig},
    widget::{Widget, WidgetConfig, WidgetFactory, WidgetProps},
};
pub use crate::editor::Editor;
pub use crate::error::{Error, Result};
pub use crate::serialization::{Layout, LayoutNode, SerializedLayout};

#[cfg(feature = "standard-widgets")]
pub use crate::widgets::{container, text};
