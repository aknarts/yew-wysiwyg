//! Widget registry for managing available widget types

use indexmap::IndexMap;
use std::rc::Rc;

use crate::core::widget::{Widget, WidgetFactory};
use crate::error::{Error, Result};

/// Registry for managing available widget types
#[derive(Clone, Default)]
pub struct WidgetRegistry {
    factories: IndexMap<String, Rc<dyn WidgetFactory>>,
}

impl PartialEq for WidgetRegistry {
    fn eq(&self, other: &Self) -> bool {
        if self.factories.len() != other.factories.len() {
            return false;
        }
        self.factories
            .keys()
            .all(|k| other.factories.contains_key(k))
    }
}

impl WidgetRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            factories: IndexMap::new(),
        }
    }

    /// Register a widget factory
    pub fn register<F: WidgetFactory>(&mut self, factory: F) -> Result<()> {
        let widget_type = factory.widget_type().to_string();
        if self.factories.contains_key(&widget_type) {
            return Err(Error::InvalidOperation(format!(
                "Widget type '{}' is already registered",
                widget_type
            )));
        }
        self.factories.insert(widget_type, Rc::new(factory));
        Ok(())
    }

    /// Create a widget instance by type
    pub fn create_widget(&self, widget_type: &str) -> Result<Box<dyn Widget>> {
        self.factories
            .get(widget_type)
            .map(|factory| factory.create())
            .ok_or_else(|| Error::WidgetNotFound(widget_type.to_string()))
    }

    /// Get all registered widget types
    pub fn widget_types(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }

    /// Check if a widget type is registered
    pub fn has_widget(&self, widget_type: &str) -> bool {
        self.factories.contains_key(widget_type)
    }

    /// Get the number of registered widgets
    pub fn len(&self) -> usize {
        self.factories.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.factories.is_empty()
    }

    /// Create a registry with standard widgets
    #[cfg(feature = "standard-widgets")]
    pub fn with_standard_widgets() -> Self {
        use crate::widgets::{basic, container, text};

        let mut registry = Self::new();

        // Register layout/container widgets (in order)
        registry.register(container::RowContainer::factory()).ok();
        registry
            .register(container::ColumnContainer::factory())
            .ok();
        registry.register(container::GridContainer::factory()).ok();
        registry.register(container::Card::factory()).ok();
        registry.register(basic::Spacer::factory()).ok();

        // Register text widgets (in order)
        registry.register(text::HeadingWidget::factory()).ok();
        registry.register(text::ParagraphWidget::factory()).ok();
        registry.register(text::TextWidget::factory()).ok();

        // Register interactive widgets (in order)
        registry.register(basic::Button::factory()).ok();
        registry.register(basic::Link::factory()).ok();
        registry.register(basic::Image::factory()).ok();

        // Register form widgets (in order)
        registry.register(basic::TextInput::factory()).ok();
        registry.register(basic::TextArea::factory()).ok();
        registry.register(basic::Checkbox::factory()).ok();

        // Register other widgets (in order)
        registry.register(basic::Divider::factory()).ok();

        registry
    }
}
