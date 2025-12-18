//! Canvas component for rendering the widget layout

use web_sys::HtmlElement;
use yew::prelude::*;

use crate::core::registry::WidgetRegistry;
use crate::core::widget::{WidgetConfig, WidgetId, WidgetProps};
use crate::serialization::Layout;

/// Properties for drop zone
#[derive(Properties, PartialEq)]
struct DropZoneProps {
    parent_id: Option<WidgetId>,
    position: usize,
    on_drop: Callback<(String, Option<WidgetId>, usize)>,
    is_dragging: bool,
}

/// Properties for empty container drop zone
#[derive(Properties, PartialEq)]
struct EmptyContainerDropZoneProps {
    parent_id: WidgetId,
    on_drop: Callback<(String, Option<WidgetId>, usize)>,
}

/// Empty container drop zone - large, prominent drop zone for empty containers
#[function_component(EmptyContainerDropZone)]
fn empty_container_drop_zone(props: &EmptyContainerDropZoneProps) -> Html {
    let is_dragging_over = use_state(|| false);

    let ondragover = {
        let is_dragging_over = is_dragging_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_dragging_over.set(true);
        })
    };

    let ondragleave = {
        let is_dragging_over = is_dragging_over.clone();
        Callback::from(move |_: DragEvent| {
            is_dragging_over.set(false);
        })
    };

    let ondrop = {
        let parent_id = props.parent_id;
        let on_drop = props.on_drop.clone();
        let is_dragging_over = is_dragging_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_dragging_over.set(false);

            if let Some(dt) = e.data_transfer() {
                if let Ok(widget_type) = dt.get_data("application/widget-type") {
                    on_drop.emit((widget_type, Some(parent_id), 0));
                }
            }
        })
    };

    let style = if *is_dragging_over {
        "min-height: 80px; width: 100%; border: 2px dashed #3b82f6; background: #eff6ff; border-radius: 4px; margin: 8px 0; transition: all 0.2s; display: flex; align-items: center; justify-content: center; color: #3b82f6; font-size: 13px; font-weight: 500;"
    } else {
        "min-height: 50px; width: 100%; border: 2px dashed #d1d5db; background: #fafafa; border-radius: 4px; margin: 8px 0; transition: all 0.2s; opacity: 1; display: flex; align-items: center; justify-content: center; color: #9ca3af; font-size: 13px;"
    };

    html! {
        <div
            class="wysiwyg-empty-container-drop-zone"
            {style}
            {ondragover}
            {ondragleave}
            {ondrop}
        >
            { if *is_dragging_over { "Drop widget here" } else { "Drop widgets here" } }
        </div>
    }
}

/// Drop zone component - shows where widgets can be dropped
#[function_component(DropZone)]
fn drop_zone(props: &DropZoneProps) -> Html {
    let is_dragging_over = use_state(|| false);

    let ondragover = {
        let is_dragging_over = is_dragging_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_dragging_over.set(true);
        })
    };

    let ondragleave = {
        let is_dragging_over = is_dragging_over.clone();
        Callback::from(move |_: DragEvent| {
            is_dragging_over.set(false);
        })
    };

    let ondrop = {
        let parent_id = props.parent_id;
        let position = props.position;
        let on_drop = props.on_drop.clone();
        let is_dragging_over = is_dragging_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_dragging_over.set(false);

            if let Some(dt) = e.data_transfer() {
                if let Ok(widget_type) = dt.get_data("application/widget-type") {
                    on_drop.emit((widget_type, parent_id, position));
                }
            }
        })
    };

    let style = if *is_dragging_over {
        // Hovering over this zone
        "height: 50px; border: 2px dashed #3b82f6; background: #eff6ff; border-radius: 4px; margin: 8px 0; display: flex; align-items: center; justify-content: center; color: #3b82f6; font-size: 13px; font-weight: 500; transition: all 0.2s;"
    } else if props.is_dragging {
        // Dragging but not over this zone - show visible
        "height: 30px; border: 2px dashed #d1d5db; background: #f9fafb; border-radius: 4px; margin: 8px 0; transition: all 0.2s;"
    } else {
        // Not dragging - show subtle
        "height: 4px; border: 1px dashed transparent; border-radius: 4px; margin: 4px 0; transition: all 0.2s;"
    };

    html! {
        <div
            class="wysiwyg-drop-zone"
            {style}
            {ondragover}
            {ondragleave}
            {ondrop}
        >
            if *is_dragging_over {
                { "Drop here" }
            }
        </div>
    }
}

/// Properties for the Canvas component
#[derive(Properties, PartialEq)]
pub struct CanvasProps {
    pub layout: Layout,
    pub registry: WidgetRegistry,
    pub selected_widget: Option<WidgetId>,
    pub on_widget_select: Callback<Option<WidgetId>>,
    pub on_widget_delete: Callback<WidgetId>,
    pub on_widget_move_up: Callback<WidgetId>,
    pub on_widget_move_down: Callback<WidgetId>,
    pub on_config_change: Callback<(WidgetId, WidgetConfig)>,
    pub on_drop_widget: Callback<(String, Option<WidgetId>, usize)>, // (widget_type, parent_id, position)
    pub edit_mode: bool,
}

/// Canvas component - renders the editable layout
#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {
    let canvas_ref = use_node_ref();
    let is_dragging = use_state(|| false);

    let on_canvas_click = {
        let on_widget_select = props.on_widget_select.clone();
        Callback::from(move |e: MouseEvent| {
            // Deselect when clicking on the canvas background
            if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                if target.class_list().contains("wysiwyg-canvas") {
                    on_widget_select.emit(None);
                }
            }
        })
    };

    let on_dragenter = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_dragging.set(true);
        })
    };

    let on_dragover = {
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
        })
    };

    let on_dragleave = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: DragEvent| {
            // Only set to false if we're leaving the canvas entirely
            if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                if target.class_list().contains("wysiwyg-canvas") {
                    is_dragging.set(false);
                }
            }
        })
    };

    let on_drop = {
        let is_dragging = is_dragging.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            is_dragging.set(false);
        })
    };

    html! {
        <div
            ref={canvas_ref}
            class="wysiwyg-canvas"
            onclick={on_canvas_click}
            ondragenter={on_dragenter}
            ondragover={on_dragover}
            ondragleave={on_dragleave}
            ondrop={on_drop}
            style="
                flex: 1;
                overflow: auto;
                padding: 20px;
                background: #f5f5f5;
                position: relative;
            "
        >
            <div style="
                max-width: 1200px;
                margin: 0 auto;
                background: white;
                min-height: 500px;
                padding: 20px;
                box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            ">
                {
                    // Render drop zones and widgets for root level
                    for props.layout.root_widgets().iter().enumerate().flat_map(|(idx, id)| {
                        let mut elements = vec![];

                        // Drop zone before widget (only in edit mode)
                        if props.edit_mode {
                            elements.push(html! {
                                <DropZone
                                    parent_id={None}
                                    position={idx}
                                    on_drop={props.on_drop_widget.clone()}
                                    is_dragging={*is_dragging}
                                />
                            });
                        }

                        // The widget itself
                        elements.push(render_widget_node(
                            id,
                            &props.layout,
                            &props.registry,
                            props.selected_widget,
                            props.on_widget_select.clone(),
                            props.on_widget_delete.clone(),
                            props.on_widget_move_up.clone(),
                            props.on_widget_move_down.clone(),
                            props.on_config_change.clone(),
                            props.on_drop_widget.clone(),
                            *is_dragging,
                            props.edit_mode,
                        ));

                        elements
                    })
                }
                // Drop zone after all widgets (or as the only zone if empty) - only in edit mode
                if props.edit_mode {
                    <DropZone
                        parent_id={None}
                        position={props.layout.root_widgets().len()}
                        on_drop={props.on_drop_widget.clone()}
                        is_dragging={*is_dragging}
                    />
                }
            </div>
        </div>
    }
}

#[allow(clippy::too_many_arguments)]
fn render_widget_node(
    id: &WidgetId,
    layout: &Layout,
    registry: &WidgetRegistry,
    selected_widget: Option<WidgetId>,
    on_widget_select: Callback<Option<WidgetId>>,
    on_widget_delete: Callback<WidgetId>,
    on_widget_move_up: Callback<WidgetId>,
    on_widget_move_down: Callback<WidgetId>,
    on_config_change: Callback<(WidgetId, WidgetConfig)>,
    on_drop_widget: Callback<(String, Option<WidgetId>, usize)>,
    is_dragging: bool,
    edit_mode: bool,
) -> Html {
    let node = match layout.get_widget(id) {
        Some(node) => node,
        None => return html! {},
    };

    let widget = match registry.create_widget(&node.config.widget_type) {
        Ok(w) => w,
        Err(_) => {
            return html! {
                <div style="color: red; border: 2px solid red; padding: 10px;">
                    { format!("Unknown widget type: {}", node.config.widget_type) }
                </div>
            };
        }
    };

    let is_selected = selected_widget == Some(*id);

    let id_copy = *id;
    let on_click = {
        let on_widget_select = on_widget_select.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_widget_select.emit(Some(id_copy));
        })
    };

    let id_copy = *id;
    let on_delete_click = {
        let on_widget_delete = on_widget_delete.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_widget_delete.emit(id_copy);
        })
    };

    let id_copy = *id;
    let on_move_up_click = {
        let on_widget_move_up = on_widget_move_up.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_widget_move_up.emit(id_copy);
        })
    };

    let id_copy = *id;
    let on_move_down_click = {
        let on_widget_move_down = on_widget_move_down.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_widget_move_down.emit(id_copy);
        })
    };

    let id_copy = *id;
    let on_config_change_clone = on_config_change.clone();
    let config_change = {
        Callback::from(move |config: WidgetConfig| {
            on_config_change_clone.emit((id_copy, config));
        })
    };

    let props = WidgetProps {
        id: *id,
        edit_mode,
        config: node.config.clone(),
        children: node.children.clone(),
        on_config_change: config_change,
        on_delete: on_widget_delete.clone(),
    };

    let widget_html = widget.render(&props);

    let wrapper_style = if is_selected && edit_mode {
        "position: relative; outline: 2px solid #3b82f6; outline-offset: 2px; margin: 4px 0;"
    } else {
        "position: relative; margin: 4px 0;"
    };

    // Special handling for Link widget - children must be inside <a> tag
    let is_link_widget = node.config.widget_type == "basic.link";

    // Prepare link attributes if this is a Link widget
    let (link_href, link_target, link_style, link_class) = if is_link_widget {
        let href = node
            .config
            .properties
            .get("href")
            .and_then(|v| v.as_str())
            .unwrap_or("https://example.com")
            .to_string();

        let target = node
            .config
            .properties
            .get("target")
            .and_then(|v| v.as_str())
            .unwrap_or("_self")
            .to_string();

        let mut style = String::new();
        for (k, v) in &node.config.inline_styles {
            style.push_str(&format!("{}: {}; ", k, v));
        }

        let class = node.config.css_classes.join(" ");

        (href, target, style, class)
    } else {
        (String::new(), String::new(), String::new(), String::new())
    };

    html! {
        <div
            class="wysiwyg-widget-wrapper"
            style={wrapper_style}
            onclick={on_click}
        >
            if is_link_widget && widget.can_have_children() {
                // For Link widgets, use <span> in edit mode, <a> in preview mode
                {
                    if edit_mode {
                        // Edit mode: use <span> so it's not clickable
                        html! {
                            <span class={link_class} style={link_style}>
                                <div class="wysiwyg-widget-children" style="min-height: 40px; display: block;">
                                    {
                                        if node.children.is_empty() {
                                            vec![html! {
                                                <EmptyContainerDropZone
                                                    parent_id={*id}
                                                    on_drop={on_drop_widget.clone()}
                                                />
                                            }]
                                        } else {
                                            node.children.iter().enumerate().flat_map(|(idx, child_id)| {
                                                vec![
                                                    html! {
                                                        <DropZone
                                                            parent_id={Some(*id)}
                                                            position={idx}
                                                            on_drop={on_drop_widget.clone()}
                                                            is_dragging={is_dragging}
                                                        />
                                                    },
                                                    render_widget_node(
                                                        child_id,
                                                        layout,
                                                        registry,
                                                        selected_widget,
                                                        on_widget_select.clone(),
                                                        on_widget_delete.clone(),
                                                        on_widget_move_up.clone(),
                                                        on_widget_move_down.clone(),
                                                        on_config_change.clone(),
                                                        on_drop_widget.clone(),
                                                        is_dragging,
                                                        edit_mode,
                                                    ),
                                                ]
                                            }).chain(vec![
                                                html! {
                                                    <DropZone
                                                        parent_id={Some(*id)}
                                                        position={node.children.len()}
                                                        on_drop={on_drop_widget.clone()}
                                                        is_dragging={is_dragging}
                                                    />
                                                }
                                            ]).collect()
                                        }
                                    }
                                </div>
                            </span>
                        }
                    } else {
                        // Preview mode: use <a> for actual link functionality
                        html! {
                            <a href={link_href} target={link_target} class={link_class} style={link_style}>
                                <div class="wysiwyg-widget-children" style="min-height: 40px; display: block;">
                                    {
                                        node.children.iter().map(|child_id| {
                                            render_widget_node(
                                                child_id,
                                                layout,
                                                registry,
                                                selected_widget,
                                                on_widget_select.clone(),
                                                on_widget_delete.clone(),
                                                on_widget_move_up.clone(),
                                                on_widget_move_down.clone(),
                                                on_config_change.clone(),
                                                on_drop_widget.clone(),
                                                is_dragging,
                                                edit_mode,
                                            )
                                        }).collect::<Vec<_>>()
                                    }
                                </div>
                            </a>
                        }
                    }
                }
            } else {
                // Normal rendering for non-Link widgets
                { widget_html }

                // Render children if it's a container
                if widget.can_have_children() {
                    <div class="wysiwyg-widget-children" style="min-height: 40px;">
                    {
                        if node.children.is_empty() {
                            // For empty containers, show a single prominent drop zone (only in edit mode)
                            if edit_mode {
                                vec![html! {
                                    <EmptyContainerDropZone
                                        parent_id={*id}
                                        on_drop={on_drop_widget.clone()}
                                    />
                                }]
                            } else {
                                vec![]
                            }
                        } else {
                            // For containers with children, show drop zones between them
                            node.children.iter().enumerate().flat_map(|(idx, child_id)| {
                                let mut elements = vec![];

                                // Drop zone before child (only in edit mode)
                                if edit_mode {
                                    elements.push(html! {
                                        <DropZone
                                            parent_id={Some(*id)}
                                            position={idx}
                                            on_drop={on_drop_widget.clone()}
                                            is_dragging={is_dragging}
                                        />
                                    });
                                }

                                // The child widget
                                elements.push(render_widget_node(
                                    child_id,
                                    layout,
                                    registry,
                                    selected_widget,
                                    on_widget_select.clone(),
                                    on_widget_delete.clone(),
                                    on_widget_move_up.clone(),
                                    on_widget_move_down.clone(),
                                    on_config_change.clone(),
                                    on_drop_widget.clone(),
                                    is_dragging,
                                    edit_mode,
                                ));

                                elements
                            }).chain(
                                // Drop zone after all children (only in edit mode)
                                if edit_mode {
                                    vec![html! {
                                        <DropZone
                                            parent_id={Some(*id)}
                                            position={node.children.len()}
                                            on_drop={on_drop_widget.clone()}
                                            is_dragging={is_dragging}
                                        />
                                    }]
                                } else {
                                    vec![]
                                }
                            ).collect()
                        }
                    }
                </div>
                }
            }

            // Control buttons when selected (only in edit mode)
            if is_selected && edit_mode {
                <div style="
                    position: absolute;
                    top: -30px;
                    right: 0;
                    display: flex;
                    gap: 4px;
                    background: white;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                    padding: 4px;
                    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                ">
                    <button
                        onclick={on_move_up_click}
                        style="
                            background: #3b82f6;
                            color: white;
                            border: none;
                            padding: 4px 8px;
                            border-radius: 3px;
                            cursor: pointer;
                            font-size: 12px;
                        "
                        title="Move up"
                    >
                        { "↑" }
                    </button>
                    <button
                        onclick={on_move_down_click}
                        style="
                            background: #3b82f6;
                            color: white;
                            border: none;
                            padding: 4px 8px;
                            border-radius: 3px;
                            cursor: pointer;
                            font-size: 12px;
                        "
                        title="Move down"
                    >
                        { "↓" }
                    </button>
                    <button
                        onclick={on_delete_click}
                        style="
                            background: #ef4444;
                            color: white;
                            border: none;
                            padding: 4px 8px;
                            border-radius: 3px;
                            cursor: pointer;
                            font-size: 12px;
                        "
                        title="Delete"
                    >
                        { "Delete" }
                    </button>
                </div>
            }
        </div>
    }
}
