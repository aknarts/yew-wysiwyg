//! Serialization and deserialization for layouts

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::widget::{WidgetConfig, WidgetId};
use crate::error::{Error, Result};

/// Serialized representation of a layout
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializedLayout {
    /// Version of the serialization format
    pub version: String,
    /// Root node IDs
    pub root_nodes: Vec<WidgetId>,
    /// All nodes in the layout
    pub nodes: HashMap<WidgetId, LayoutNode>,
    /// Metadata about the layout
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for SerializedLayout {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            root_nodes: Vec::new(),
            nodes: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
}

impl SerializedLayout {
    /// Create a new empty layout
    pub fn new() -> Self {
        Self::default()
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    /// Serialize to pretty JSON string
    pub fn to_json_pretty(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| Error::DeserializationError(e.to_string()))
    }

    /// Add a node to the layout
    pub fn add_node(&mut self, id: WidgetId, node: LayoutNode) {
        self.nodes.insert(id, node);
    }

    /// Remove a node from the layout
    pub fn remove_node(&mut self, id: &WidgetId) -> Option<LayoutNode> {
        self.nodes.remove(id)
    }

    /// Get a node by ID
    pub fn get_node(&self, id: &WidgetId) -> Option<&LayoutNode> {
        self.nodes.get(id)
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, id: &WidgetId) -> Option<&mut LayoutNode> {
        self.nodes.get_mut(id)
    }

    /// Add metadata
    pub fn set_metadata(&mut self, key: String, value: serde_json::Value) {
        self.metadata.insert(key, value);
    }

    /// Validate the layout structure
    pub fn validate(&self) -> Result<()> {
        // Check that all root nodes exist
        for root_id in &self.root_nodes {
            if !self.nodes.contains_key(root_id) {
                return Err(Error::InvalidOperation(format!(
                    "Root node {} not found in nodes",
                    root_id
                )));
            }
        }

        // Check that all child references are valid
        for (id, node) in &self.nodes {
            for child_id in &node.children {
                if !self.nodes.contains_key(child_id) {
                    return Err(Error::InvalidOperation(format!(
                        "Node {} references non-existent child {}",
                        id, child_id
                    )));
                }
            }
        }

        Ok(())
    }
}

/// A node in the layout tree
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutNode {
    /// Widget configuration
    pub config: WidgetConfig,
    /// Child widget IDs (for containers)
    pub children: Vec<WidgetId>,
    /// Parent widget ID (if any)
    pub parent: Option<WidgetId>,
    /// Custom metadata for this node
    pub metadata: HashMap<String, serde_json::Value>,
}

impl LayoutNode {
    /// Create a new layout node
    pub fn new(config: WidgetConfig) -> Self {
        Self {
            config,
            children: Vec::new(),
            parent: None,
            metadata: HashMap::new(),
        }
    }

    /// Add a child to this node
    pub fn add_child(&mut self, child_id: WidgetId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /// Remove a child from this node
    pub fn remove_child(&mut self, child_id: &WidgetId) -> bool {
        if let Some(pos) = self.children.iter().position(|id| id == child_id) {
            self.children.remove(pos);
            true
        } else {
            false
        }
    }
}

/// In-memory representation of a layout
#[derive(Clone, PartialEq)]
pub struct Layout {
    serialized: SerializedLayout,
}

impl Layout {
    /// Create a new empty layout
    pub fn new() -> Self {
        Self {
            serialized: SerializedLayout::new(),
        }
    }

    /// Create from serialized layout
    pub fn from_serialized(serialized: SerializedLayout) -> Result<Self> {
        serialized.validate()?;
        Ok(Self { serialized })
    }

    /// Get the serialized representation
    pub fn to_serialized(&self) -> &SerializedLayout {
        &self.serialized
    }

    /// Get a mutable reference to the serialized representation
    pub fn to_serialized_mut(&mut self) -> &mut SerializedLayout {
        &mut self.serialized
    }

    /// Add a root widget
    pub fn add_root_widget(&mut self, id: WidgetId, config: WidgetConfig) {
        self.serialized.root_nodes.push(id);
        self.serialized.add_node(id, LayoutNode::new(config));
    }

    /// Add a root widget at a specific position
    pub fn insert_root_widget(&mut self, id: WidgetId, config: WidgetConfig, position: usize) {
        let pos = position.min(self.serialized.root_nodes.len());
        self.serialized.root_nodes.insert(pos, id);
        self.serialized.add_node(id, LayoutNode::new(config));
    }

    /// Add a child widget to a parent
    pub fn add_child_widget(
        &mut self,
        parent_id: WidgetId,
        child_id: WidgetId,
        config: WidgetConfig,
    ) -> Result<()> {
        let parent = self
            .serialized
            .get_node_mut(&parent_id)
            .ok_or_else(|| Error::WidgetNotFound(parent_id.to_string()))?;

        parent.add_child(child_id);

        let mut child_node = LayoutNode::new(config);
        child_node.parent = Some(parent_id);
        self.serialized.add_node(child_id, child_node);

        Ok(())
    }

    /// Add a child widget to a parent at a specific position
    pub fn insert_child_widget(
        &mut self,
        parent_id: WidgetId,
        child_id: WidgetId,
        config: WidgetConfig,
        position: usize,
    ) -> Result<()> {
        let parent = self
            .serialized
            .get_node_mut(&parent_id)
            .ok_or_else(|| Error::WidgetNotFound(parent_id.to_string()))?;

        let pos = position.min(parent.children.len());
        parent.children.insert(pos, child_id);

        let mut child_node = LayoutNode::new(config);
        child_node.parent = Some(parent_id);
        self.serialized.add_node(child_id, child_node);

        Ok(())
    }

    /// Remove a widget and its children
    pub fn remove_widget(&mut self, id: &WidgetId) -> Result<()> {
        let node = self
            .serialized
            .get_node(id)
            .ok_or_else(|| Error::WidgetNotFound(id.to_string()))?
            .clone();

        // Remove from parent or root
        if let Some(parent_id) = node.parent {
            if let Some(parent) = self.serialized.get_node_mut(&parent_id) {
                parent.remove_child(id);
            }
        } else {
            self.serialized.root_nodes.retain(|root_id| root_id != id);
        }

        // Recursively remove children
        for child_id in &node.children {
            self.remove_widget(child_id)?;
        }

        // Remove the node itself
        self.serialized.remove_node(id);

        Ok(())
    }

    /// Move a widget up in its parent's children list (or root list)
    pub fn move_widget_up(&mut self, id: &WidgetId) -> Result<()> {
        let node = self
            .serialized
            .get_node(id)
            .ok_or_else(|| Error::WidgetNotFound(id.to_string()))?
            .clone();

        // Determine if this is a root widget or has a parent
        if let Some(parent_id) = node.parent {
            // Move within parent's children
            let parent = self
                .serialized
                .get_node_mut(&parent_id)
                .ok_or_else(|| Error::WidgetNotFound(parent_id.to_string()))?;

            let pos = parent
                .children
                .iter()
                .position(|child_id| child_id == id)
                .ok_or_else(|| Error::InvalidOperation("Widget not found in parent".to_string()))?;

            if pos > 0 {
                parent.children.swap(pos - 1, pos);
            }
        } else {
            // Move within root nodes
            let pos = self
                .serialized
                .root_nodes
                .iter()
                .position(|root_id| root_id == id)
                .ok_or_else(|| Error::InvalidOperation("Widget not found in roots".to_string()))?;

            if pos > 0 {
                self.serialized.root_nodes.swap(pos - 1, pos);
            }
        }

        Ok(())
    }

    /// Move a widget down in its parent's children list (or root list)
    pub fn move_widget_down(&mut self, id: &WidgetId) -> Result<()> {
        let node = self
            .serialized
            .get_node(id)
            .ok_or_else(|| Error::WidgetNotFound(id.to_string()))?
            .clone();

        // Determine if this is a root widget or has a parent
        if let Some(parent_id) = node.parent {
            // Move within parent's children
            let parent = self
                .serialized
                .get_node_mut(&parent_id)
                .ok_or_else(|| Error::WidgetNotFound(parent_id.to_string()))?;

            let pos = parent
                .children
                .iter()
                .position(|child_id| child_id == id)
                .ok_or_else(|| Error::InvalidOperation("Widget not found in parent".to_string()))?;

            if pos < parent.children.len() - 1 {
                parent.children.swap(pos, pos + 1);
            }
        } else {
            // Move within root nodes
            let pos = self
                .serialized
                .root_nodes
                .iter()
                .position(|root_id| root_id == id)
                .ok_or_else(|| Error::InvalidOperation("Widget not found in roots".to_string()))?;

            if pos < self.serialized.root_nodes.len() - 1 {
                self.serialized.root_nodes.swap(pos, pos + 1);
            }
        }

        Ok(())
    }

    /// Get root widget IDs
    pub fn root_widgets(&self) -> &[WidgetId] {
        &self.serialized.root_nodes
    }

    /// Get a widget node
    pub fn get_widget(&self, id: &WidgetId) -> Option<&LayoutNode> {
        self.serialized.get_node(id)
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String> {
        self.serialized.to_json()
    }

    /// Serialize to pretty JSON
    pub fn to_json_pretty(&self) -> Result<String> {
        self.serialized.to_json_pretty()
    }

    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        let serialized = SerializedLayout::from_json(json)?;
        Self::from_serialized(serialized)
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_serialization() {
        let mut layout = Layout::new();
        let id = WidgetId::new_v4();
        let config = WidgetConfig::new("test");

        layout.add_root_widget(id, config);

        let json = layout.to_json().unwrap();
        let deserialized = Layout::from_json(&json).unwrap();

        assert_eq!(deserialized.root_widgets().len(), 1);
    }
}
