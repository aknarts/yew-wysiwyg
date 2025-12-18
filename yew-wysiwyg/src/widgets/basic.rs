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
            .with_property(
                "src",
                serde_json::json!("https://via.placeholder.com/400x300"),
            )
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

        let mut style = format!(
            "border: none; border-top: {}px solid {}; ",
            thickness, color
        );
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
/// Text Input widget
#[derive(Default)]
pub struct TextInput;

impl TextInput {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for TextInput {
    fn widget_type(&self) -> &'static str {
        "form.textinput"
    }

    fn display_name(&self) -> &'static str {
        "Text Input"
    }

    fn description(&self) -> &'static str {
        "Single-line text input field"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "📝" }</span> }
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("placeholder", serde_json::json!("Enter text..."))
            .with_property("label", serde_json::json!(""))
            .with_property("type", serde_json::json!("text"))
            .with_style("width", "100%")
            .with_style("padding", "8px 12px")
            .with_style("border", "1px solid #d1d5db")
            .with_style("border-radius", "4px")
            .with_style("font-size", "14px")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let placeholder = props
            .config
            .properties
            .get("placeholder")
            .and_then(|v| v.as_str())
            .unwrap_or("Enter text...")
            .to_string();

        let label = props
            .config
            .properties
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let input_type = props
            .config
            .properties
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("text")
            .to_string();

        let mut style = String::new();
        for (k, v) in &props.config.inline_styles {
            style.push_str(&format!("{}: {}; ", k, v));
        }

        let class = props.config.css_classes.join(" ");

        html! {
            <div style="display: flex; flex-direction: column; gap: 4px;">
                if !label.is_empty() {
                    <label style="font-weight: 500; font-size: 14px; color: #374151;">
                        { label }
                    </label>
                }
                <input
                    type={input_type}
                    placeholder={placeholder}
                    {class}
                    {style}
                />
            </div>
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let placeholder = config
            .properties
            .get("placeholder")
            .and_then(|v| v.as_str())
            .unwrap_or("Enter text...")
            .to_string();

        let label = config
            .properties
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let input_type = config
            .properties
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("text")
            .to_string();

        let config_clone = config.clone();
        let on_placeholder_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_config = config_clone.clone();
                    new_config.set_property("placeholder", serde_json::json!(input.value()));
                    on_change.emit(new_config);
                }
            })
        };

        let config_clone = config.clone();
        let on_label_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_config = config_clone.clone();
                    new_config.set_property("label", serde_json::json!(input.value()));
                    on_change.emit(new_config);
                }
            })
        };

        let config_clone = config.clone();
        let on_type_change = {
            Callback::from(move |e: Event| {
                if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                    let mut new_config = config_clone.clone();
                    new_config.set_property("type", serde_json::json!(select.value()));
                    on_change.emit(new_config);
                }
            })
        };

        html! {
            <div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Label:" }
                    </label>
                    <input
                        type="text"
                        value={label}
                        oninput={on_label_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Placeholder:" }
                    </label>
                    <input
                        type="text"
                        value={placeholder}
                        oninput={on_placeholder_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Input Type:" }
                    </label>
                    <select
                        value={input_type}
                        onchange={on_type_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    >
                        <option value="text">{ "Text" }</option>
                        <option value="email">{ "Email" }</option>
                        <option value="password">{ "Password" }</option>
                        <option value="tel">{ "Telephone" }</option>
                        <option value="url">{ "URL" }</option>
                        <option value="number">{ "Number" }</option>
                    </select>
                </div>
            </div>
        }
    }
}

/// Text Area widget
#[derive(Default)]
pub struct TextArea;

impl TextArea {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for TextArea {
    fn widget_type(&self) -> &'static str {
        "form.textarea"
    }

    fn display_name(&self) -> &'static str {
        "Text Area"
    }

    fn description(&self) -> &'static str {
        "Multi-line text input field"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "📄" }</span> }
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("placeholder", serde_json::json!("Enter text..."))
            .with_property("label", serde_json::json!(""))
            .with_property("rows", serde_json::json!(4))
            .with_style("width", "100%")
            .with_style("padding", "8px 12px")
            .with_style("border", "1px solid #d1d5db")
            .with_style("border-radius", "4px")
            .with_style("font-size", "14px")
            .with_style("font-family", "inherit")
            .with_style("resize", "vertical")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let placeholder = props
            .config
            .properties
            .get("placeholder")
            .and_then(|v| v.as_str())
            .unwrap_or("Enter text...")
            .to_string();

        let label = props
            .config
            .properties
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let rows = props
            .config
            .properties
            .get("rows")
            .and_then(|v| v.as_u64())
            .unwrap_or(4)
            .to_string();

        let mut style = String::new();
        for (k, v) in &props.config.inline_styles {
            style.push_str(&format!("{}: {}; ", k, v));
        }

        let class = props.config.css_classes.join(" ");

        html! {
            <div style="display: flex; flex-direction: column; gap: 4px;">
                if !label.is_empty() {
                    <label style="font-weight: 500; font-size: 14px; color: #374151;">
                        { label }
                    </label>
                }
                <textarea
                    placeholder={placeholder}
                    rows={rows}
                    {class}
                    {style}
                />
            </div>
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let placeholder = config
            .properties
            .get("placeholder")
            .and_then(|v| v.as_str())
            .unwrap_or("Enter text...")
            .to_string();

        let label = config
            .properties
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let rows = config
            .properties
            .get("rows")
            .and_then(|v| v.as_u64())
            .unwrap_or(4);

        let config_clone = config.clone();
        let on_placeholder_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_config = config_clone.clone();
                    new_config.set_property("placeholder", serde_json::json!(input.value()));
                    on_change.emit(new_config);
                }
            })
        };

        let config_clone = config.clone();
        let on_label_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_config = config_clone.clone();
                    new_config.set_property("label", serde_json::json!(input.value()));
                    on_change.emit(new_config);
                }
            })
        };

        let config_clone = config.clone();
        let on_rows_change = {
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(rows) = input.value().parse::<u64>() {
                        let mut new_config = config_clone.clone();
                        new_config.set_property("rows", serde_json::json!(rows));
                        on_change.emit(new_config);
                    }
                }
            })
        };

        html! {
            <div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Label:" }
                    </label>
                    <input
                        type="text"
                        value={label}
                        oninput={on_label_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Placeholder:" }
                    </label>
                    <input
                        type="text"
                        value={placeholder}
                        oninput={on_placeholder_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Rows:" }
                    </label>
                    <input
                        type="number"
                        value={rows.to_string()}
                        oninput={on_rows_change}
                        min="1"
                        max="20"
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
            </div>
        }
    }
}

/// Checkbox widget
#[derive(Default)]
pub struct Checkbox;

impl Checkbox {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for Checkbox {
    fn widget_type(&self) -> &'static str {
        "form.checkbox"
    }

    fn display_name(&self) -> &'static str {
        "Checkbox"
    }

    fn description(&self) -> &'static str {
        "Checkbox input field"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "☑️" }</span> }
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("label", serde_json::json!("Check me"))
            .with_property("checked", serde_json::json!(false))
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let label = props
            .config
            .properties
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("Check me");

        let checked = props
            .config
            .properties
            .get("checked")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        html! {
            <div style="display: flex; align-items: center; gap: 8px;">
                <input
                    type="checkbox"
                    checked={checked}
                    style="width: 16px; height: 16px; cursor: pointer;"
                />
                <label style="font-size: 14px; color: #374151; cursor: pointer;">
                    { label }
                </label>
            </div>
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let label = config
            .properties
            .get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("Check me")
            .to_string();

        let checked = config
            .properties
            .get("checked")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let config_clone = config.clone();
        let on_label_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_config = config_clone.clone();
                    new_config.set_property("label", serde_json::json!(input.value()));
                    on_change.emit(new_config);
                }
            })
        };

        let config_clone = config.clone();
        let on_checked_change = {
            Callback::from(move |e: Event| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    let mut new_config = config_clone.clone();
                    new_config.set_property("checked", serde_json::json!(input.checked()));
                    on_change.emit(new_config);
                }
            })
        };

        html! {
            <div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Label:" }
                    </label>
                    <input
                        type="text"
                        value={label}
                        oninput={on_label_change}
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
                <div style="margin-bottom: 12px;">
                    <label style="display: flex; align-items: center; gap: 8px;">
                        <input
                            type="checkbox"
                            checked={checked}
                            onchange={on_checked_change}
                            style="width: 16px; height: 16px;"
                        />
                        <span style="font-weight: 500;">{ "Default Checked" }</span>
                    </label>
                </div>
            </div>
        }
    }
}

/// Spacer widget for layout control
#[derive(Default)]
pub struct Spacer;

impl Spacer {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for Spacer {
    fn widget_type(&self) -> &'static str {
        "layout.spacer"
    }

    fn display_name(&self) -> &'static str {
        "Spacer"
    }

    fn description(&self) -> &'static str {
        "Empty space for layout control"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "⬜" }</span> }
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("height", serde_json::json!(20))
            .with_style("width", "100%")
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let height = props
            .config
            .properties
            .get("height")
            .and_then(|v| v.as_u64())
            .unwrap_or(20);

        let mut style = format!("height: {}px; ", height);
        for (k, v) in &props.config.inline_styles {
            style.push_str(&format!("{}: {}; ", k, v));
        }

        let class = props.config.css_classes.join(" ");

        html! {
            <div {class} {style} />
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let height = config
            .properties
            .get("height")
            .and_then(|v| v.as_u64())
            .unwrap_or(20);

        let config_clone = config.clone();
        let on_height_change = {
            Callback::from(move |e: InputEvent| {
                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                    if let Ok(height) = input.value().parse::<u64>() {
                        let mut new_config = config_clone.clone();
                        new_config.set_property("height", serde_json::json!(height));
                        on_change.emit(new_config);
                    }
                }
            })
        };

        html! {
            <div>
                <div style="margin-bottom: 12px;">
                    <label style="display: block; margin-bottom: 4px; font-weight: 500;">
                        { "Height (px):" }
                    </label>
                    <input
                        type="number"
                        value={height.to_string()}
                        oninput={on_height_change}
                        min="0"
                        max="500"
                        style="width: 100%; padding: 6px; border: 1px solid #ddd; border-radius: 4px;"
                    />
                </div>
            </div>
        }
    }
}
