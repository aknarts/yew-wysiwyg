//! Theme system for customizing widget appearance

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme configuration for the editor and widgets
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Theme name
    pub name: String,
    /// CSS variables to inject
    pub css_variables: HashMap<String, String>,
    /// Global CSS classes
    pub global_classes: Vec<String>,
    /// Custom CSS to inject
    pub custom_css: Option<String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            css_variables: HashMap::new(),
            global_classes: Vec::new(),
            custom_css: None,
        }
    }
}

impl ThemeConfig {
    /// Create a new theme configuration
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Set a CSS variable
    pub fn with_variable(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.css_variables.insert(name.into(), value.into());
        self
    }

    /// Add a global CSS class
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.global_classes.push(class.into());
        self
    }

    /// Set custom CSS
    pub fn with_custom_css(mut self, css: impl Into<String>) -> Self {
        self.custom_css = Some(css.into());
        self
    }
}

/// Trait for theme providers
pub trait Theme: 'static {
    /// Get the theme configuration
    fn config(&self) -> &ThemeConfig;

    /// Get CSS for a specific widget type
    fn widget_css(&self, widget_type: &str) -> Option<String> {
        let _ = widget_type;
        None
    }

    /// Get the theme name
    fn name(&self) -> &str {
        &self.config().name
    }
}

/// Default theme implementation
#[derive(Debug, Clone, Default)]
pub struct DefaultTheme {
    config: ThemeConfig,
}

impl DefaultTheme {
    pub fn new() -> Self {
        Self {
            config: ThemeConfig::new("default")
                .with_variable("--wysiwyg-primary", "#3b82f6")
                .with_variable("--wysiwyg-secondary", "#64748b")
                .with_variable("--wysiwyg-background", "#ffffff")
                .with_variable("--wysiwyg-text", "#1e293b")
                .with_variable("--wysiwyg-border", "#e2e8f0")
                .with_variable("--wysiwyg-border-radius", "4px")
                .with_variable("--wysiwyg-spacing", "8px"),
        }
    }
}

impl Theme for DefaultTheme {
    fn config(&self) -> &ThemeConfig {
        &self.config
    }
}
