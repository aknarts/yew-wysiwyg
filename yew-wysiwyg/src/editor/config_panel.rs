//! Configuration panel for editing widget properties

use yew::prelude::*;

use crate::core::registry::WidgetRegistry;
use crate::core::widget::{WidgetConfig, WidgetId};
use crate::serialization::Layout;

/// Build breadcrumb path from root to selected widget
fn build_breadcrumb_path(
    layout: &Layout,
    registry: &WidgetRegistry,
    widget_id: &WidgetId,
) -> Vec<(WidgetId, String)> {
    let mut path = Vec::new();
    let mut current_id = Some(*widget_id);

    // Walk up the tree to build path
    while let Some(id) = current_id {
        if let Some(node) = layout.get_widget(&id) {
            let name = registry
                .create_widget(&node.config.widget_type)
                .map(|w| w.display_name().to_string())
                .unwrap_or_else(|_| node.config.widget_type.clone());

            path.push((id, name));
            current_id = node.parent;
        } else {
            break;
        }
    }

    // Reverse to get root-to-leaf order
    path.reverse();
    path
}

/// Properties for the ConfigPanel component
#[derive(Properties, PartialEq)]
pub struct ConfigPanelProps {
    pub layout: Layout,
    pub registry: WidgetRegistry,
    pub selected_widget: Option<WidgetId>,
    pub on_config_change: Callback<(WidgetId, WidgetConfig)>,
    pub on_widget_select: Callback<Option<WidgetId>>,
}

/// Configuration panel component - shows widget properties
#[function_component(ConfigPanel)]
pub fn config_panel(props: &ConfigPanelProps) -> Html {
    html! {
        <div
            class="wysiwyg-config-panel"
            style="
                width: 300px;
                background: #ffffff;
                border-left: 1px solid #e5e7eb;
                padding: 16px;
                overflow-y: auto;
                display: flex;
                flex-direction: column;
                gap: 16px;
            "
        >
            {
                if let Some(widget_id) = props.selected_widget {
                    if let Some(node) = props.layout.get_widget(&widget_id) {
                        if let Ok(widget) = props.registry.create_widget(&node.config.widget_type) {
                            let config = node.config.clone();
                            let widget_id_copy = widget_id;
                            let on_change = {
                                let on_config_change = props.on_config_change.clone();
                                Callback::from(move |new_config: WidgetConfig| {
                                    on_config_change.emit((widget_id_copy, new_config));
                                })
                            };

                            // Build breadcrumb path
                            let breadcrumb_path = build_breadcrumb_path(&props.layout, &props.registry, &widget_id);

                            html! {
                                <>
                                    // Breadcrumb navigation
                                    if breadcrumb_path.len() > 1 {
                                        <div style="
                                            padding: 12px;
                                            background: #f9fafb;
                                            border-bottom: 1px solid #e5e7eb;
                                            margin: -16px -16px 16px -16px;
                                        ">
                                            <div style="
                                                display: flex;
                                                align-items: center;
                                                gap: 8px;
                                                font-size: 13px;
                                                flex-wrap: wrap;
                                            ">
                                                {
                                                    for breadcrumb_path.iter().enumerate().map(|(idx, (id, name))| {
                                                        let is_last = idx == breadcrumb_path.len() - 1;
                                                        let id_copy = *id;
                                                        let on_select = props.on_widget_select.clone();

                                                        html! {
                                                            <>
                                                                if !is_last {
                                                                    <button
                                                                        class="breadcrumb-link"
                                                                        onclick={Callback::from(move |e: MouseEvent| {
                                                                            e.stop_propagation();
                                                                            on_select.emit(Some(id_copy));
                                                                        })}
                                                                        style="
                                                                            background: none;
                                                                            border: none;
                                                                            color: #3b82f6;
                                                                            cursor: pointer;
                                                                            padding: 4px 8px;
                                                                            border-radius: 4px;
                                                                            font-size: 13px;
                                                                        "
                                                                    >
                                                                        { name }
                                                                    </button>
                                                                    <span style="color: #9ca3af;">{ "›" }</span>
                                                                } else {
                                                                    <span style="
                                                                        color: #111827;
                                                                        font-weight: 600;
                                                                        padding: 4px 8px;
                                                                    ">
                                                                        { name }
                                                                    </span>
                                                                }
                                                            </>
                                                        }
                                                    })
                                                }
                                            </div>
                                        </div>
                                    }

                                    <div>
                                        <h3 style="
                                            margin: 0 0 8px 0;
                                            font-size: 16px;
                                            font-weight: 600;
                                            color: #111827;
                                        ">
                                            { widget.display_name() }
                                        </h3>
                                        <p style="
                                            margin: 0;
                                            font-size: 13px;
                                            color: #6b7280;
                                        ">
                                            { widget.description() }
                                        </p>
                                    </div>

                                    <div style="
                                        border-top: 1px solid #e5e7eb;
                                        padding-top: 16px;
                                    ">
                                        <h4 style="
                                            margin: 0 0 12px 0;
                                            font-size: 14px;
                                            font-weight: 600;
                                            color: #374151;
                                        ">
                                            { "Properties" }
                                        </h4>
                                        { widget.render_config_ui(&config, on_change) }
                                    </div>

                                    <div style="
                                        border-top: 1px solid #e5e7eb;
                                        padding-top: 16px;
                                    ">
                                        <h4 style="
                                            margin: 0 0 8px 0;
                                            font-size: 14px;
                                            font-weight: 600;
                                            color: #374151;
                                        ">
                                            { "Widget Info" }
                                        </h4>
                                        <div style="
                                            font-size: 12px;
                                            color: #6b7280;
                                            font-family: monospace;
                                            background: #f9fafb;
                                            padding: 8px;
                                            border-radius: 4px;
                                        ">
                                            <div>{ format!("Type: {}", node.config.widget_type) }</div>
                                            <div>{ format!("ID: {}", widget_id) }</div>
                                        </div>
                                    </div>
                                </>
                            }
                        } else {
                            html! {
                                <div style="
                                    padding: 20px;
                                    text-align: center;
                                    color: #dc2626;
                                ">
                                    { "Unknown widget type" }
                                </div>
                            }
                        }
                    } else {
                        html! {
                            <div style="
                                padding: 20px;
                                text-align: center;
                                color: #dc2626;
                            ">
                                { "Widget not found" }
                            </div>
                        }
                    }
                } else {
                    html! {
                        <div style="
                            padding: 20px;
                            text-align: center;
                            color: #9ca3af;
                        ">
                            <div style="font-size: 48px; margin-bottom: 16px;">
                                { "⚙️" }
                            </div>
                            <h3 style="
                                margin: 0 0 8px 0;
                                font-size: 16px;
                                font-weight: 600;
                                color: #6b7280;
                            ">
                                { "No Widget Selected" }
                            </h3>
                            <p style="
                                margin: 0;
                                font-size: 14px;
                                color: #9ca3af;
                            ">
                                { "Select a widget to edit its properties" }
                            </p>
                        </div>
                    }
                }
            }
        </div>
    }
}
