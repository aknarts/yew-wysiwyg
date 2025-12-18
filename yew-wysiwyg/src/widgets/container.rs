//! Container widgets for layout

use yew::prelude::*;

use crate::core::widget::{SimpleWidgetFactory, Widget, WidgetConfig, WidgetProps};

/// Row container - arranges children horizontally
#[derive(Default)]
pub struct RowContainer;

impl RowContainer {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for RowContainer {
    fn widget_type(&self) -> &'static str {
        "container.row"
    }

    fn display_name(&self) -> &'static str {
        "Row Container"
    }

    fn description(&self) -> &'static str {
        "Arranges child widgets horizontally in a row"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "⬌" }</span> }
    }

    fn can_have_children(&self) -> bool {
        true
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_style("display", "flex")
            .with_style("flex-direction", "row")
            .with_style("gap", "var(--wysiwyg-spacing, 8px)")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let style = build_style(&props.config);
        let class = build_class(&props.config);

        html! {
            <div {class} {style}>
                // Children will be rendered by the editor
            </div>
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let gap = config
            .inline_styles
            .get("gap")
            .and_then(|v| v.strip_suffix("px"))
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(8);

        let config_clone = config.clone();
        let on_gap_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .inline_styles
                    .insert("gap".to_string(), format!("{}px", input.value()));
                on_change.emit(new_config);
            })
        };

        html! {
            <div>
                <label>
                    { "Gap (px): " }
                    <input
                        type="number"
                        value={gap.to_string()}
                        oninput={on_gap_change}
                        min="0"
                        max="100"
                    />
                </label>
            </div>
        }
    }
}

/// Column container - arranges children vertically
#[derive(Default)]
pub struct ColumnContainer;

impl ColumnContainer {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for ColumnContainer {
    fn widget_type(&self) -> &'static str {
        "container.column"
    }

    fn display_name(&self) -> &'static str {
        "Column Container"
    }

    fn description(&self) -> &'static str {
        "Arranges child widgets vertically in a column"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "⬍" }</span> }
    }

    fn can_have_children(&self) -> bool {
        true
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_style("display", "flex")
            .with_style("flex-direction", "column")
            .with_style("gap", "var(--wysiwyg-spacing, 8px)")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let style = build_style(&props.config);
        let class = build_class(&props.config);

        html! {
            <div {class} {style}>
            </div>
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let gap = config
            .inline_styles
            .get("gap")
            .and_then(|v| v.strip_suffix("px"))
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(8);

        let config_clone = config.clone();
        let on_gap_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .inline_styles
                    .insert("gap".to_string(), format!("{}px", input.value()));
                on_change.emit(new_config);
            })
        };

        html! {
            <div>
                <label>
                    { "Gap (px): " }
                    <input
                        type="number"
                        value={gap.to_string()}
                        oninput={on_gap_change}
                        min="0"
                        max="100"
                    />
                </label>
            </div>
        }
    }
}

/// Grid container - arranges children in a grid
#[derive(Default)]
pub struct GridContainer;

impl GridContainer {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for GridContainer {
    fn widget_type(&self) -> &'static str {
        "container.grid"
    }

    fn display_name(&self) -> &'static str {
        "Grid Container"
    }

    fn description(&self) -> &'static str {
        "Arranges child widgets in a responsive grid"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "▦" }</span> }
    }

    fn can_have_children(&self) -> bool {
        true
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_style("display", "grid")
            .with_style(
                "grid-template-columns",
                "repeat(auto-fit, minmax(200px, 1fr))",
            )
            .with_style("gap", "var(--wysiwyg-spacing, 8px)")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let style = build_style(&props.config);
        let class = build_class(&props.config);

        html! {
            <div {class} {style}>
            </div>
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let columns = config
            .inline_styles
            .get("grid-template-columns")
            .cloned()
            .unwrap_or_else(|| "repeat(auto-fit, minmax(200px, 1fr))".to_string());

        let config_clone = config.clone();
        let on_columns_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .inline_styles
                    .insert("grid-template-columns".to_string(), input.value());
                on_change.emit(new_config);
            })
        };

        html! {
            <div>
                <label>
                    { "Grid Template Columns: " }
                    <input
                        type="text"
                        value={columns}
                        oninput={on_columns_change}
                        style="width: 100%;"
                    />
                </label>
                <small style="color: #666;">
                    { "e.g., 'repeat(3, 1fr)' or '200px 1fr'" }
                </small>
            </div>
        }
    }
}

/// Card/Panel container - a styled box that can contain content
#[derive(Default)]
pub struct Card;

impl Card {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for Card {
    fn widget_type(&self) -> &'static str {
        "container.card"
    }

    fn display_name(&self) -> &'static str {
        "Card"
    }

    fn description(&self) -> &'static str {
        "A styled card/panel that can contain other widgets"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "▢" }</span> }
    }

    fn can_have_children(&self) -> bool {
        true
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("title", serde_json::json!(""))
            .with_style("border", "1px solid #e5e7eb")
            .with_style("border-radius", "8px")
            .with_style("padding", "16px")
            .with_style("background", "#ffffff")
            .with_style("box-shadow", "0 1px 3px rgba(0,0,0,0.1)")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let title = props
            .config
            .properties
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let style = build_style(&props.config);
        let class = build_class(&props.config);

        html! {
            <div {class} {style}>
                if !title.is_empty() {
                    <div style="font-size: 18px; font-weight: 600; margin-bottom: 12px; color: #111827;">
                        { title }
                    </div>
                }
            </div>
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let title = config
            .properties
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let config_clone = config.clone();
        let on_title_change = {
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .properties
                    .insert("title".to_string(), serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        html! {
            <div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Card Title (optional):" }
                    </label>
                    <input
                        type="text"
                        value={title}
                        oninput={on_title_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                        placeholder="Leave empty for no title"
                    />
                </div>
            </div>
        }
    }
}

// Helper functions

fn build_style(config: &WidgetConfig) -> String {
    config
        .inline_styles
        .iter()
        .map(|(k, v)| format!("{}: {};", k, v))
        .collect::<Vec<_>>()
        .join(" ")
}

fn build_class(config: &WidgetConfig) -> String {
    config.css_classes.join(" ")
}
