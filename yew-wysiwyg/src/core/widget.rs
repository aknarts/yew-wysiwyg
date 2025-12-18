//! Core widget trait and related types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use yew::prelude::*;

use crate::error::Result;

/// Unique identifier for a widget instance
pub type WidgetId = Uuid;

/// Properties passed to every widget
#[derive(Clone, PartialEq)]
pub struct WidgetProps {
    /// Unique identifier for this widget instance
    pub id: WidgetId,
    /// Whether the widget is in edit mode (can be modified)
    pub edit_mode: bool,
    /// Widget-specific configuration
    pub config: WidgetConfig,
    /// Child widgets (for container widgets)
    pub children: Vec<WidgetId>,
    /// Callback when widget configuration changes
    pub on_config_change: Callback<WidgetConfig>,
    /// Callback when widget requests deletion
    pub on_delete: Callback<WidgetId>,
}

/// Widget configuration data (serializable)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WidgetConfig {
    /// Widget type identifier
    pub widget_type: String,
    /// Custom properties specific to this widget type
    pub properties: HashMap<String, serde_json::Value>,
    /// Custom CSS classes
    pub css_classes: Vec<String>,
    /// Custom inline styles
    pub inline_styles: HashMap<String, String>,
}

impl WidgetConfig {
    /// Create a new widget configuration
    pub fn new(widget_type: impl Into<String>) -> Self {
        Self {
            widget_type: widget_type.into(),
            properties: HashMap::new(),
            css_classes: Vec::new(),
            inline_styles: HashMap::new(),
        }
    }

    /// Set a property value
    pub fn with_property(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    /// Get a property value
    pub fn get_property(&self, key: &str) -> Option<&serde_json::Value> {
        self.properties.get(key)
    }

    /// Set a property value (mutable)
    pub fn set_property(&mut self, key: impl Into<String>, value: serde_json::Value) {
        self.properties.insert(key.into(), value);
    }

    /// Add a CSS class
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.css_classes.push(class.into());
        self
    }

    /// Add an inline style
    pub fn with_style(mut self, property: impl Into<String>, value: impl Into<String>) -> Self {
        self.inline_styles.insert(property.into(), value.into());
        self
    }
}

/// Trait that all widgets must implement
pub trait Widget: 'static {
    /// Return the unique type identifier for this widget
    fn widget_type(&self) -> &'static str;

    /// Render the widget
    fn render(&self, props: &WidgetProps) -> Html;

    /// Validate widget configuration
    fn validate_config(&self, config: &WidgetConfig) -> Result<()> {
        // Default implementation accepts any config
        let _ = config;
        Ok(())
    }

    /// Provide a default configuration for this widget
    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
    }

    /// Return whether this widget can contain children
    fn can_have_children(&self) -> bool {
        false
    }

    /// Render the widget's configuration UI (for the editor)
    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let _ = (config, on_change);
        html! { <div>{ "No configuration available" }</div> }
    }

    /// Get a human-readable name for this widget
    fn display_name(&self) -> &'static str {
        self.widget_type()
    }

    /// Get a description for this widget
    fn description(&self) -> &'static str {
        ""
    }

    /// Get an icon or preview for this widget (HTML or CSS class name)
    fn icon(&self) -> Html {
        html! { <span>{ "📦" }</span> }
    }
}

/// Factory for creating widget instances
pub trait WidgetFactory: 'static {
    /// Create a new widget instance
    fn create(&self) -> Box<dyn Widget>;

    /// Get the widget type this factory creates
    fn widget_type(&self) -> &'static str;
}

/// Simple widget factory implementation
pub struct SimpleWidgetFactory<W: Widget + Default> {
    _phantom: std::marker::PhantomData<W>,
}

impl<W: Widget + Default> SimpleWidgetFactory<W> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<W: Widget + Default> Default for SimpleWidgetFactory<W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<W: Widget + Default> WidgetFactory for SimpleWidgetFactory<W> {
    fn create(&self) -> Box<dyn Widget> {
        Box::new(W::default())
    }

    fn widget_type(&self) -> &'static str {
        W::default().widget_type()
    }
}
