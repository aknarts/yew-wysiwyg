//! Text-based widgets

use pulldown_cmark::{html, Parser};
use yew::prelude::*;

use crate::core::widget::{SimpleWidgetFactory, Widget, WidgetConfig, WidgetProps};

/// Convert markdown to HTML
fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Generic text widget with rich text support
#[derive(Default)]
pub struct TextWidget;

impl TextWidget {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for TextWidget {
    fn widget_type(&self) -> &'static str {
        "text"
    }

    fn display_name(&self) -> &'static str {
        "Text"
    }

    fn description(&self) -> &'static str {
        "Rich text with formatting support"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "T" }</span> }
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("content", serde_json::json!("Enter text here..."))
            .with_property("bold", serde_json::json!(false))
            .with_property("italic", serde_json::json!(false))
            .with_property("underline", serde_json::json!(false))
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let content = props
            .config
            .get_property("content")
            .and_then(|v| v.as_str())
            .unwrap_or("Enter text here...");

        let bold = props
            .config
            .get_property("bold")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let italic = props
            .config
            .get_property("italic")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let underline = props
            .config
            .get_property("underline")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let mut style = build_style(&props.config);
        if bold {
            style.push_str("font-weight: bold;");
        }
        if italic {
            style.push_str("font-style: italic;");
        }
        if underline {
            style.push_str("text-decoration: underline;");
        }

        let class = build_class(&props.config);

        html! {
            <span {class} {style}>{ content }</span>
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let content = config
            .get_property("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let bold = config
            .get_property("bold")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let italic = config
            .get_property("italic")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let underline = config
            .get_property("underline")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let config_clone = config.clone();
        let on_content_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config.set_property("content", serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        let config_clone = config.clone();
        let on_bold_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: Event| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config.set_property("bold", serde_json::json!(input.checked()));
                on_change.emit(new_config);
            })
        };

        let config_clone = config.clone();
        let on_italic_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: Event| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config.set_property("italic", serde_json::json!(input.checked()));
                on_change.emit(new_config);
            })
        };

        let config_clone = config.clone();
        let on_underline_change = {
            Callback::from(move |e: Event| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config.set_property("underline", serde_json::json!(input.checked()));
                on_change.emit(new_config);
            })
        };

        html! {
            <div style="display: flex; flex-direction: column; gap: 8px;">
                <label>
                    { "Content: " }
                    <input
                        type="text"
                        value={content}
                        oninput={on_content_change}
                        style="width: 100%;"
                    />
                </label>
                <div style="display: flex; gap: 12px;">
                    <label>
                        <input type="checkbox" checked={bold} onchange={on_bold_change} />
                        { " Bold" }
                    </label>
                    <label>
                        <input type="checkbox" checked={italic} onchange={on_italic_change} />
                        { " Italic" }
                    </label>
                    <label>
                        <input type="checkbox" checked={underline} onchange={on_underline_change} />
                        { " Underline" }
                    </label>
                </div>
            </div>
        }
    }
}

/// Heading widget (H1-H6)
#[derive(Default)]
pub struct HeadingWidget;

impl HeadingWidget {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for HeadingWidget {
    fn widget_type(&self) -> &'static str {
        "text.heading"
    }

    fn display_name(&self) -> &'static str {
        "Heading"
    }

    fn description(&self) -> &'static str {
        "Heading element (H1-H6)"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "H" }</span> }
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property("content", serde_json::json!("Heading"))
            .with_property("level", serde_json::json!(1))
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let content = props
            .config
            .get_property("content")
            .and_then(|v| v.as_str())
            .unwrap_or("Heading");

        let level = props
            .config
            .get_property("level")
            .and_then(|v| v.as_i64())
            .unwrap_or(1)
            .clamp(1, 6);

        let style = build_style(&props.config);
        let class = build_class(&props.config);

        match level {
            1 => html! { <h1 {class} {style}>{ content }</h1> },
            2 => html! { <h2 {class} {style}>{ content }</h2> },
            3 => html! { <h3 {class} {style}>{ content }</h3> },
            4 => html! { <h4 {class} {style}>{ content }</h4> },
            5 => html! { <h5 {class} {style}>{ content }</h5> },
            _ => html! { <h6 {class} {style}>{ content }</h6> },
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let content = config
            .get_property("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let level = config
            .get_property("level")
            .and_then(|v| v.as_i64())
            .unwrap_or(1);

        let config_clone = config.clone();
        let on_content_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config.set_property("content", serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        let config_clone = config.clone();
        let on_level_change = {
            Callback::from(move |e: Event| {
                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                if let Ok(level) = select.value().parse::<i64>() {
                    new_config.set_property("level", serde_json::json!(level));
                    on_change.emit(new_config);
                }
            })
        };

        html! {
            <div style="display: flex; flex-direction: column; gap: 8px;">
                <label>
                    { "Content: " }
                    <input
                        type="text"
                        value={content}
                        oninput={on_content_change}
                        style="width: 100%;"
                    />
                </label>
                <label>
                    { "Level: " }
                    <select onchange={on_level_change} value={level.to_string()}>
                        <option value="1" selected={level == 1}>{ "H1" }</option>
                        <option value="2" selected={level == 2}>{ "H2" }</option>
                        <option value="3" selected={level == 3}>{ "H3" }</option>
                        <option value="4" selected={level == 4}>{ "H4" }</option>
                        <option value="5" selected={level == 5}>{ "H5" }</option>
                        <option value="6" selected={level == 6}>{ "H6" }</option>
                    </select>
                </label>
            </div>
        }
    }
}

/// Paragraph widget
#[derive(Default)]
pub struct ParagraphWidget;

impl ParagraphWidget {
    pub fn factory() -> SimpleWidgetFactory<Self> {
        SimpleWidgetFactory::new()
    }
}

impl Widget for ParagraphWidget {
    fn widget_type(&self) -> &'static str {
        "text.paragraph"
    }

    fn display_name(&self) -> &'static str {
        "Paragraph"
    }

    fn description(&self) -> &'static str {
        "Paragraph of text with optional Markdown support"
    }

    fn icon(&self) -> Html {
        html! { <span>{ "¶" }</span> }
    }

    fn default_config(&self) -> WidgetConfig {
        WidgetConfig::new(self.widget_type())
            .with_property(
                "content",
                serde_json::json!(
                    "This is a paragraph of text. You can edit it in the configuration panel."
                ),
            )
            .with_property("markdown", serde_json::json!(false))
    }

    fn render(&self, props: &WidgetProps) -> Html {
        let content = props
            .config
            .get_property("content")
            .and_then(|v| v.as_str())
            .unwrap_or("Paragraph text");

        let markdown = props
            .config
            .get_property("markdown")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let style = build_style(&props.config);
        let class = build_class(&props.config);

        if markdown {
            let html_content = markdown_to_html(content);
            // Use Html::from_html_unchecked to render the markdown HTML
            let inner_html = Html::from_html_unchecked(html_content.into());
            html! {
                <div {class} {style}>{ inner_html }</div>
            }
        } else {
            html! {
                <p {class} {style}>{ content }</p>
            }
        }
    }

    fn render_config_ui(&self, config: &WidgetConfig, on_change: Callback<WidgetConfig>) -> Html {
        let content = config
            .get_property("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let markdown = config
            .get_property("markdown")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let config_clone = config.clone();
        let on_content_change = {
            let on_change = on_change.clone();
            Callback::from(move |e: InputEvent| {
                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config.set_property("content", serde_json::json!(input.value()));
                on_change.emit(new_config);
            })
        };

        let config_clone = config.clone();
        let on_markdown_change = {
            Callback::from(move |e: Event| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let mut new_config = config_clone.clone();
                new_config.set_property("markdown", serde_json::json!(input.checked()));
                on_change.emit(new_config);
            })
        };

        html! {
            <div style="display: flex; flex-direction: column; gap: 12px;">
                <label>
                    <input type="checkbox" checked={markdown} onchange={on_markdown_change} />
                    { " Enable Markdown" }
                </label>
                {
                    if markdown {
                        html! {
                            <div style="
                                background: #f0f9ff;
                                border: 1px solid #bae6fd;
                                border-radius: 4px;
                                padding: 8px;
                                font-size: 12px;
                                color: #0369a1;
                            ">
                                <strong>{ "Markdown enabled:" }</strong>
                                { " Use **bold**, *italic*, [links](url), lists, and more" }
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <label>
                    { "Content: " }
                    <textarea
                        value={content}
                        oninput={on_content_change}
                        style="width: 100%; min-height: 150px; font-family: monospace;"
                        rows="6"
                    />
                </label>
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
