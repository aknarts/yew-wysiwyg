//! Widget palette for selecting and adding widgets

use yew::prelude::*;

use crate::core::registry::WidgetRegistry;
use crate::core::widget::WidgetConfig;

/// Properties for the WidgetPalette component
#[derive(Properties, PartialEq)]
pub struct WidgetPaletteProps {
    pub registry: WidgetRegistry,
    pub on_add_widget: Callback<(String, WidgetConfig)>,
}

/// Widget palette component - shows available widgets
#[function_component(WidgetPalette)]
pub fn widget_palette(props: &WidgetPaletteProps) -> Html {
    let widget_types = props.registry.widget_types();

    html! {
        <div
            class="wysiwyg-palette"
            style="
                width: 250px;
                background: #ffffff;
                border-right: 1px solid #e5e7eb;
                padding: 16px;
                overflow-y: auto;
                display: flex;
                flex-direction: column;
                gap: 8px;
            "
        >
            <h3 style="margin: 0 0 16px 0; font-size: 16px; font-weight: 600;">
                { "Widgets" }
            </h3>

            <div style="display: flex; flex-direction: column; gap: 8px;">
                {
                    for widget_types.iter().map(|widget_type| {
                        let widget = props.registry.create_widget(widget_type);

                        match widget {
                            Ok(widget) => {
                                let widget_type_clone = widget_type.clone();
                                let default_config = widget.default_config();
                                let on_add = props.on_add_widget.clone();

                                let onclick = Callback::from(move |_: MouseEvent| {
                                    on_add.emit((widget_type_clone.clone(), default_config.clone()));
                                });

                                let widget_type_for_drag = widget_type.clone();
                                let ondragstart = Callback::from(move |e: DragEvent| {
                                    e.stop_propagation();
                                    if let Some(dt) = e.data_transfer() {
                                        let _ = dt.set_data("application/widget-type", &widget_type_for_drag);
                                        dt.set_effect_allowed("copy");
                                    }
                                });

                                html! {
                                    <button
                                        {onclick}
                                        draggable="true"
                                        {ondragstart}
                                        class="wysiwyg-palette-item"
                                        style="
                                            display: flex;
                                            align-items: center;
                                            gap: 8px;
                                            padding: 12px;
                                            background: #f9fafb;
                                            border: 1px solid #e5e7eb;
                                            border-radius: 6px;
                                            cursor: pointer;
                                            text-align: left;
                                            transition: all 0.15s;
                                        "
                                        onmouseenter={Callback::from(|e: MouseEvent| {
                                            if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>() {
                                                let _ = target.style().set_property("background", "#f3f4f6");
                                                let _ = target.style().set_property("border-color", "#d1d5db");
                                            }
                                        })}
                                        onmouseleave={Callback::from(|e: MouseEvent| {
                                            if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>() {
                                                let _ = target.style().set_property("background", "#f9fafb");
                                                let _ = target.style().set_property("border-color", "#e5e7eb");
                                            }
                                        })}
                                    >
                                        <span style="font-size: 24px;">
                                            { widget.icon() }
                                        </span>
                                        <div style="flex: 1; min-width: 0;">
                                            <div style="font-weight: 500; font-size: 14px; margin-bottom: 2px;">
                                                { widget.display_name() }
                                            </div>
                                            <div style="font-size: 11px; color: #6b7280; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                                                { widget.description() }
                                            </div>
                                        </div>
                                    </button>
                                }
                            }
                            Err(_) => html! {}
                        }
                    })
                }
            </div>

            if widget_types.is_empty() {
                <div style="
                    text-align: center;
                    padding: 20px;
                    color: #9ca3af;
                    font-size: 14px;
                ">
                    { "No widgets available" }
                </div>
            }
        </div>
    }
}
