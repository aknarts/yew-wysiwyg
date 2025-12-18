//! Editor component and related utilities

mod canvas;
mod config_panel;
mod palette;
mod toolbar;

use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::core::registry::WidgetRegistry;
use crate::core::theme::{DefaultTheme, Theme};
use crate::core::widget::{WidgetConfig, WidgetId};
use crate::serialization::Layout;

pub use canvas::Canvas;
pub use config_panel::ConfigPanel;
pub use palette::WidgetPalette;
pub use toolbar::Toolbar;

/// Local storage key for auto-saving layouts
const AUTOSAVE_KEY: &str = "yew-wysiwyg-autosave";

/// Load layout from local storage
fn load_from_storage() -> Option<Layout> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok()??;
    let json = storage.get_item(AUTOSAVE_KEY).ok()??;
    Layout::from_json(&json).ok()
}

/// Save layout to local storage
fn save_to_storage(layout: &Layout) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(json) = layout.to_json() {
                let _ = storage.set_item(AUTOSAVE_KEY, &json);
            }
        }
    }
}

/// Clear layout from local storage
fn clear_storage() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(AUTOSAVE_KEY);
        }
    }
}

/// Properties for the Editor component
#[derive(Properties)]
pub struct EditorProps {
    /// Initial layout (optional)
    #[prop_or_default]
    pub initial_layout: Option<Layout>,

    /// Widget registry (optional, uses standard widgets if not provided)
    #[prop_or_default]
    pub registry: Option<WidgetRegistry>,

    /// Theme (optional, uses default theme if not provided)
    #[prop_or_default]
    pub theme: Option<Rc<dyn Theme>>,

    /// Callback when layout changes
    #[prop_or_default]
    pub on_layout_change: Option<Callback<Layout>>,

    /// Whether to show the widget palette
    #[prop_or(true)]
    pub show_palette: bool,

    /// Whether to show the toolbar
    #[prop_or(true)]
    pub show_toolbar: bool,

    /// Whether to show the configuration panel
    #[prop_or(true)]
    pub show_config_panel: bool,
}

impl PartialEq for EditorProps {
    fn eq(&self, other: &Self) -> bool {
        self.initial_layout == other.initial_layout
            && self.registry == other.registry
            && self.show_palette == other.show_palette
            && self.show_toolbar == other.show_toolbar
            && self.show_config_panel == other.show_config_panel
        // Note: We skip comparing theme and callbacks as they can't be compared easily
    }
}

/// Main editor component
#[function_component(Editor)]
pub fn editor(props: &EditorProps) -> Html {
    // Initialize state - try loading from localStorage if no initial layout provided
    let layout = use_state(|| {
        props
            .initial_layout
            .clone()
            .or_else(load_from_storage)
            .unwrap_or_default()
    });

    let registry = use_memo(props.registry.clone(), |registry_prop| {
        registry_prop.clone().unwrap_or_else(|| {
            #[cfg(feature = "standard-widgets")]
            {
                WidgetRegistry::with_standard_widgets()
            }
            #[cfg(not(feature = "standard-widgets"))]
            {
                WidgetRegistry::new()
            }
        })
    });

    let theme = props
        .theme
        .clone()
        .unwrap_or_else(|| Rc::new(DefaultTheme::new()) as Rc<dyn Theme>);

    let selected_widget = use_state(|| None::<WidgetId>);
    let edit_mode = use_state(|| true);

    // History management for undo/redo
    let history = use_state(|| vec![props.initial_layout.clone().unwrap_or_default()]);
    let history_index = use_state(|| 0usize);

    // Helper function to add a layout to history
    let push_to_history = {
        let history = history.clone();
        let history_index = history_index.clone();
        let layout = layout.clone();
        move |new_layout: Layout| {
            let mut hist = (*history).clone();
            let idx = *history_index;

            // Remove any future history if we're not at the end
            hist.truncate(idx + 1);

            // Add the new layout
            hist.push(new_layout.clone());

            // Limit history to 50 entries
            if hist.len() > 50 {
                hist.remove(0);
            } else {
                history_index.set(idx + 1);
            }

            history.set(hist);
            layout.set(new_layout);
        }
    };

    // Undo/Redo callbacks
    let on_undo = {
        let history = history.clone();
        let history_index = history_index.clone();
        let layout = layout.clone();
        Callback::from(move |_| {
            let idx = *history_index;
            if idx > 0 {
                let new_idx = idx - 1;
                history_index.set(new_idx);
                if let Some(prev_layout) = (*history).get(new_idx) {
                    layout.set(prev_layout.clone());
                }
            }
        })
    };

    let on_redo = {
        let history = history.clone();
        let history_index = history_index.clone();
        let layout = layout.clone();
        Callback::from(move |_| {
            let idx = *history_index;
            let hist = (*history).clone();
            if idx < hist.len() - 1 {
                let new_idx = idx + 1;
                history_index.set(new_idx);
                if let Some(next_layout) = hist.get(new_idx) {
                    layout.set(next_layout.clone());
                }
            }
        })
    };

    let can_undo = *history_index > 0;
    let can_redo = *history_index < (*history).len() - 1;

    // Callbacks
    let on_add_widget = {
        let push_to_history = push_to_history.clone();
        let layout = layout.clone();
        let selected_widget = selected_widget.clone();
        let registry = registry.clone();
        let on_layout_change = props.on_layout_change.clone();
        Callback::from(move |(_widget_type, config): (String, WidgetConfig)| {
            let mut new_layout = (*layout).clone();
            let id = WidgetId::new_v4();

            // Check if a container widget is selected
            let add_as_child = if let Some(parent_id) = *selected_widget {
                // Check if parent can have children
                if let Some(parent_node) = new_layout.get_widget(&parent_id) {
                    if let Ok(parent_widget) =
                        registry.create_widget(&parent_node.config.widget_type)
                    {
                        parent_widget.can_have_children()
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if add_as_child {
                // Add as child of selected container
                if let Some(parent_id) = *selected_widget {
                    if new_layout.add_child_widget(parent_id, id, config).is_ok() {
                        push_to_history(new_layout.clone());
                        if let Some(callback) = &on_layout_change {
                            callback.emit(new_layout);
                        }
                    }
                }
            } else {
                // Add as root widget
                new_layout.add_root_widget(id, config);
                push_to_history(new_layout.clone());
                if let Some(callback) = &on_layout_change {
                    callback.emit(new_layout);
                }
            }
        })
    };

    let on_widget_select = {
        let selected_widget = selected_widget.clone();
        Callback::from(move |id: Option<WidgetId>| {
            selected_widget.set(id);
        })
    };

    let on_widget_delete = {
        let push_to_history = push_to_history.clone();
        let layout = layout.clone();
        let selected_widget = selected_widget.clone();
        let on_layout_change = props.on_layout_change.clone();
        Callback::from(move |id: WidgetId| {
            let mut new_layout = (*layout).clone();
            if new_layout.remove_widget(&id).is_ok() {
                push_to_history(new_layout.clone());
                selected_widget.set(None);

                if let Some(callback) = &on_layout_change {
                    callback.emit(new_layout);
                }
            }
        })
    };

    let on_config_change = {
        let push_to_history = push_to_history.clone();
        let layout = layout.clone();
        let on_layout_change = props.on_layout_change.clone();
        Callback::from(move |(id, config): (WidgetId, WidgetConfig)| {
            let mut new_layout = (*layout).clone();
            if let Some(node) = new_layout.to_serialized_mut().get_node_mut(&id) {
                node.config = config;
                push_to_history(new_layout.clone());

                if let Some(callback) = &on_layout_change {
                    callback.emit(new_layout);
                }
            }
        })
    };

    let on_widget_move_up = {
        let push_to_history = push_to_history.clone();
        let layout = layout.clone();
        let on_layout_change = props.on_layout_change.clone();
        Callback::from(move |id: WidgetId| {
            let mut new_layout = (*layout).clone();
            if new_layout.move_widget_up(&id).is_ok() {
                push_to_history(new_layout.clone());

                if let Some(callback) = &on_layout_change {
                    callback.emit(new_layout);
                }
            }
        })
    };

    let on_widget_move_down = {
        let push_to_history = push_to_history.clone();
        let layout = layout.clone();
        let on_layout_change = props.on_layout_change.clone();
        Callback::from(move |id: WidgetId| {
            let mut new_layout = (*layout).clone();
            if new_layout.move_widget_down(&id).is_ok() {
                push_to_history(new_layout.clone());

                if let Some(callback) = &on_layout_change {
                    callback.emit(new_layout);
                }
            }
        })
    };

    let on_drop_widget = {
        let push_to_history = push_to_history.clone();
        let layout = layout.clone();
        let registry = registry.clone();
        let on_layout_change = props.on_layout_change.clone();
        Callback::from(
            move |(widget_type, parent_id, position): (String, Option<WidgetId>, usize)| {
                // Create widget with default config
                if let Ok(widget) = registry.create_widget(&widget_type) {
                    let mut new_layout = (*layout).clone();
                    let id = WidgetId::new_v4();
                    let config = widget.default_config();

                    // Insert at the specified position
                    if let Some(parent_id) = parent_id {
                        // Insert as child
                        if new_layout
                            .insert_child_widget(parent_id, id, config, position)
                            .is_ok()
                        {
                            push_to_history(new_layout.clone());
                            if let Some(callback) = &on_layout_change {
                                callback.emit(new_layout);
                            }
                        }
                    } else {
                        // Insert as root
                        new_layout.insert_root_widget(id, config, position);
                        push_to_history(new_layout.clone());
                        if let Some(callback) = &on_layout_change {
                            callback.emit(new_layout);
                        }
                    }
                }
            },
        )
    };

    let on_import = {
        let push_to_history = push_to_history.clone();
        let selected_widget = selected_widget.clone();
        let on_layout_change = props.on_layout_change.clone();
        Callback::from(move |json: String| match Layout::from_json(&json) {
            Ok(new_layout) => {
                push_to_history(new_layout.clone());
                selected_widget.set(None);

                if let Some(callback) = &on_layout_change {
                    callback.emit(new_layout);
                }
            }
            Err(e) => {
                log::error!("Failed to import layout: {}", e);
            }
        })
    };

    let on_toggle_edit_mode = {
        let edit_mode = edit_mode.clone();
        let selected_widget = selected_widget.clone();
        Callback::from(move |_| {
            edit_mode.set(!*edit_mode);
            // Clear selection when toggling to preview mode
            if *edit_mode {
                selected_widget.set(None);
            }
        })
    };

    let on_clear = {
        let layout = layout.clone();
        let history = history.clone();
        let history_index = history_index.clone();
        let selected_widget = selected_widget.clone();
        let on_layout_change = props.on_layout_change.clone();
        Callback::from(move |_| {
            // Clear localStorage
            clear_storage();

            // Create new empty layout
            let new_layout = Layout::new();
            layout.set(new_layout.clone());

            // Reset history
            history.set(vec![new_layout.clone()]);
            history_index.set(0);

            // Clear selection
            selected_widget.set(None);

            // Notify parent
            if let Some(callback) = &on_layout_change {
                callback.emit(new_layout);
            }
        })
    };

    // Keyboard shortcuts for undo/redo
    {
        let on_undo = on_undo.clone();
        let on_redo = on_redo.clone();

        use_effect(move || {
            let callback = {
                let on_undo = on_undo.clone();
                let on_redo = on_redo.clone();

                Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                    // Check for Ctrl/Cmd key
                    let is_ctrl_or_cmd = e.ctrl_key() || e.meta_key();

                    if is_ctrl_or_cmd && !e.shift_key() && e.key() == "z" {
                        // Ctrl+Z or Cmd+Z - Undo
                        if can_undo {
                            e.prevent_default();
                            on_undo.emit(());
                        }
                    } else if is_ctrl_or_cmd
                        && (e.key() == "y" || (e.shift_key() && e.key() == "Z"))
                    {
                        // Ctrl+Y or Ctrl+Shift+Z or Cmd+Y or Cmd+Shift+Z - Redo
                        if can_redo {
                            e.prevent_default();
                            on_redo.emit(());
                        }
                    }
                }) as Box<dyn FnMut(_)>)
            };

            let window = web_sys::window().expect("no global window exists");
            let _ = window
                .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref());

            // Cleanup
            move || {
                let window = web_sys::window().expect("no global window exists");
                let _ = window.remove_event_listener_with_callback(
                    "keydown",
                    callback.as_ref().unchecked_ref(),
                );
                drop(callback);
            }
        });
    }

    // Auto-save layout to localStorage on every change
    {
        let layout = (*layout).clone();
        use_effect_with(layout.clone(), move |layout| {
            save_to_storage(layout);
            || ()
        });
    }

    // Apply theme CSS variables
    let theme_style = {
        let vars: String = theme
            .config()
            .css_variables
            .iter()
            .map(|(k, v)| format!("{}: {};", k, v))
            .collect::<Vec<_>>()
            .join(" ");
        format!("display: flex; height: 100vh; {}", vars)
    };

    html! {
        <div class="yew-wysiwyg-editor" style={theme_style}>
            if props.show_palette && *edit_mode {
                <WidgetPalette
                    registry={(*registry).clone()}
                    on_add_widget={on_add_widget}
                />
            }
            <div style="flex: 1; display: flex; flex-direction: column; overflow: hidden;">
                if props.show_toolbar {
                    <Toolbar
                        layout={(*layout).clone()}
                        selected_widget={*selected_widget}
                        on_import={on_import}
                        on_clear={on_clear}
                        edit_mode={*edit_mode}
                        on_toggle_edit_mode={on_toggle_edit_mode}
                        on_undo={on_undo}
                        on_redo={on_redo}
                        can_undo={can_undo}
                        can_redo={can_redo}
                    />
                }
                <Canvas
                    layout={(*layout).clone()}
                    registry={(*registry).clone()}
                    selected_widget={*selected_widget}
                    on_widget_select={on_widget_select.clone()}
                    on_widget_delete={on_widget_delete}
                    on_widget_move_up={on_widget_move_up}
                    on_widget_move_down={on_widget_move_down}
                    on_config_change={on_config_change.clone()}
                    on_drop_widget={on_drop_widget}
                    edit_mode={*edit_mode}
                />
            </div>
            if props.show_config_panel && *edit_mode {
                <ConfigPanel
                    layout={(*layout).clone()}
                    registry={(*registry).clone()}
                    selected_widget={*selected_widget}
                    on_config_change={on_config_change.clone()}
                    on_widget_select={on_widget_select.clone()}
                />
            }
        </div>
    }
}
