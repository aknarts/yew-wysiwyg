//! Basic interactive widgets

use yew::prelude::*;

use crate::core::widget::{SimpleWidgetFactory, Widget, WidgetConfig, WidgetProps};

/// Button widget
#[derive(Default)]
pub struct Button;

impl Button {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for Button {
    fn widget_type(&self) -> &'static str {
        "basic.button"
    }

    fn display_name(&self) -> &'static str {
        "Button"
    }

    fn description(&self) -> &'static str {
        "A clickable button"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "🔘" }</span> }
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("text", serde_json::json!("Click me"))
            .with_property("variant", serde_json::json!("primary"))
            .with_style("padding", "8px 16px")
            .with_style("border", "none")
            .with_style("border-radius", "4px")
            .with_style("cursor", "pointer")
            .with_style("font-size", "14px")
            .with_style("font-weight", "500")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let text = props
            .config
            .properties
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("Click me");

        let variant = props
            .config
            .properties
            .get("variant")
            .and_then(|v| v.as_str())
            .unwrap_or("primary");

        let bg_color = match variant {
            "primary" => "#3b82f6",
            "secondary" => "#6b7280",
            "success" => "#10b981",
            "danger" => "#ef4444",
            _ => "#3b82f6",
        };

        let mut style = format!("background: {}; color: white; ", bg_color);
        for (k, v) in &props.config.inline_styles {
            style.push_str(&format!("{}: {}; ", k, v));
        }

        let class = props.config.css_classes.join(" ");

        html! {
            <button {class} {style}>
                { text }
            </button>
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let text = config
            .properties
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("Click me")
            .to_string();

        let variant = config
            .properties
            .get("variant")
            .and_then(|v| v.as_str())
            .unwrap_or("primary")
            .to_string();

        let config_clone = config.clone();
        let on_text_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .properties
                    .insert("text".to_string(), serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        let config_clone = config.clone();
        let on_variant_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: Event| {
                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .properties
                    .insert("variant".to_string(), serde_json::json!(select.value()));
                on_change.emit(new_config);
            })
        };

        html! {
            <div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Button Text:" }
                    </label>
                    <input
                        type="text"
                        value={text}
                        oninput={on_text_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Variant:" }
                    </label>
                    <select
                        value={variant.clone()}
                        onchange={on_variant_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    >
                        <option value="primary" selected={variant == "primary"}>{ "Primary (Blue)" }</option>
                        <option value="secondary" selected={variant == "secondary"}>{ "Secondary (Gray)" }</option>
                        <option value="success" selected={variant == "success"}>{ "Success (Green)" }</option>
                        <option value="danger" selected={variant == "danger"}>{ "Danger (Red)" }</option>
                    </select>
                </div>
            </div>
        }
    }
}

/// Image widget
#[derive(Default)]
pub struct Image;

impl Image {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for Image {
    fn widget_type(&self) -> &'static str {
        "basic.image"
    }

    fn display_name(&self) -> &'static str {
        "Image"
    }

    fn description(&self) -> &'static str {
        "An image with configurable source and alt text"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "🖼️" }</span> }
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("src", serde_json::json!("https://via.placeholder.com/400x300"))
            .with_property("alt", serde_json::json!("Placeholder image"))
            .with_style("max-width", "100%")
            .with_style("height", "auto")
            .with_style("display", "block")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let src = props
            .config
            .properties
            .get("src")
            .and_then(|v| v.as_str())
            .unwrap_or("https://via.placeholder.com/400x300")
            .to_string();

        let alt = props
            .config
            .properties
            .get("alt")
            .and_then(|v| v.as_str())
            .unwrap_or("Image")
            .to_string();

        let mut style = String::new();
        for (k, v) in &props.config.inline_styles {
            style.push_str(&format!("{}: {}; ", k, v));
        }

        let class = props.config.css_classes.join(" ");

        html! {
            <img {src} {alt} {class} {style} />
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let src = config
            .properties
            .get("src")
            .and_then(|v| v.as_str())
            .unwrap_or("https://via.placeholder.com/400x300")
            .to_string();

        let alt = config
            .properties
            .get("alt")
            .and_then(|v| v.as_str())
            .unwrap_or("Image")
            .to_string();

        let config_clone = config.clone();
        let on_src_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .properties
                    .insert("src".to_string(), serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        let config_clone = config.clone();
        let on_alt_change = {
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .properties
                    .insert("alt".to_string(), serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        html! {
            <div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Image URL:" }
                    </label>
                    <input
                        type="text"
                        value={src}
                        oninput={on_src_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                        placeholder="https://example.com/image.jpg"
                    />
                </div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Alt Text:" }
                    </label>
                    <input
                        type="text"
                        value={alt}
                        oninput={on_alt_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                        placeholder="Description of the image"
                    />
                </div>
            </div>
        }
    }
}

/// Link container - wraps children in a clickable link
#[derive(Default)]
pub struct Link;

impl Link {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for Link {
    fn widget_type(&self) -> &'static str {
        "basic.link"
    }

    fn display_name(&self) -> &'static str {
        "Link"
    }

    fn description(&self) -> &'static str {
        "A clickable link container - put images, text, or any widget inside"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "🔗" }</span> }
    }

    fn can_have_children(&self) -> bool {
        true
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("href", serde_json::json!("https://example.com"))
            .with_property("target", serde_json::json!("_self"))
            .with_style("color", "#3b82f6")
            .with_style("text-decoration", "none")
            .with_style("cursor", "pointer")
            .with_style("display", "inline-block")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let href = props
            .config
            .properties
            .get("href")
            .and_then(|v| v.as_str())
            .unwrap_or("https://example.com")
            .to_string();

        let target = props
            .config
            .properties
            .get("target")
            .and_then(|v| v.as_str())
            .unwrap_or("_self")
            .to_string();

        let mut style = String::new();
        for (k, v) in &props.config.inline_styles {
            style.push_str(&format!("{}: {}; ", k, v));
        }

        let class = props.config.css_classes.join(" ");

        html! {
            <a {href} {target} {class} {style}>
                // Children will be rendered by the editor
            </a>
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let href = config
            .properties
            .get("href")
            .and_then(|v| v.as_str())
            .unwrap_or("https://example.com")
            .to_string();

        let target = config
            .properties
            .get("target")
            .and_then(|v| v.as_str())
            .unwrap_or("_self")
            .to_string();

        let config_clone = config.clone();
        let on_href_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .properties
                    .insert("href".to_string(), serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        let config_clone = config.clone();
        let on_target_change = {
            Callback::from(move |e: Event| {
                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .properties
                    .insert("target".to_string(), serde_json::json!(select.value()));
                on_change.emit(new_config);
            })
        };

        html! {
            <div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "URL:" }
                    </label>
                    <input
                        type="text"
                        value={href}
                        oninput={on_href_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                        placeholder="https://example.com"
                    />
                </div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Open in:" }
                    </label>
                    <select
                        value={target.clone()}
                        onchange={on_target_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    >
                        <option value="_self" selected={target == "_self"}>{ "Same tab" }</option>
                        <option value="_blank" selected={target == "_blank"}>{ "New tab" }</option>
                    </select>
                </div>
                <p style="margin: 12px 0 0 0; font-size: 12px; color: #6b7280;">
                    { "💡 Add text, images, or other widgets inside this link to make them clickable" }
                </p>
            </div>
        }
    }
}

/// Divider widget
#[derive(Default)]
pub struct Divider;

impl Divider {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for Divider {
    fn widget_type(&self) -> &'static str {
        "basic.divider"
    }

    fn display_name(&self) -> &'static str {
        "Divider"
    }

    fn description(&self) -> &'static str {
        "A horizontal line to separate content"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "─" }</span> }
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("thickness", serde_json::json!("1"))
            .with_property("color", serde_json::json!("#e5e7eb"))
            .with_style("margin", "16px 0")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let thickness = props
            .config
            .properties
            .get("thickness")
            .and_then(|v| v.as_str())
            .unwrap_or("1");

        let color = props
            .config
            .properties
            .get("color")
            .and_then(|v| v.as_str())
            .unwrap_or("#e5e7eb");

        let mut style = format!("border: none; border-top: {}px solid {}; ", thickness, color);
        for (k, v) in &props.config.inline_styles {
            style.push_str(&format!("{}: {}; ", k, v));
        }

        let class = props.config.css_classes.join(" ");

        html! {
            <hr {class} {style} />
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let thickness = config
            .properties
            .get("thickness")
            .and_then(|v| v.as_str())
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(1);

        let color = config
            .properties
            .get("color")
            .and_then(|v| v.as_str())
            .unwrap_or("#e5e7eb")
            .to_string();

        let config_clone = config.clone();
        let on_thickness_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .properties
                    .insert("thickness".to_string(), serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        let config_clone = config.clone();
        let on_color_change = {
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config
                    .properties
                    .insert("color".to_string(), serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        html! {
            <div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Thickness (px):" }
                    </label>
                    <input
                        type="number"
                        value={thickness.to_string()}
                        oninput={on_thickness_change}
                        min="1"
                        max="10"
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Color:" }
                    </label>
                    <input
                        type="color"
                        value={color}
                        oninput={on_color_change}
                        style="width: 100%; padding: 4px; border: 1px solid #ddd; border-radius: 4px; height: 40px;"
                    />
                </div>
            </div>
        }
    }
}
