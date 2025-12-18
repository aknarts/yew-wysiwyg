//! Toolbar component for editor actions

use yew::prelude::*;

use crate::core::widget::WidgetId;
use crate::serialization::Layout;

/// Properties for the Toolbar component
#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    pub layout: Layout,
    pub selected_widget: Option<WidgetId>,
    pub on_import: Callback<String>,
    pub on_clear: Callback<()>,
    pub edit_mode: bool,
    pub on_toggle_edit_mode: Callback<()>,
    pub on_undo: Callback<()>,
    pub on_redo: Callback<()>,
    pub can_undo: bool,
    pub can_redo: bool,
}

/// Toolbar component
#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let show_modal = use_state(|| false);
    let json_content = use_state(String::new);
    let import_error = use_state(|| Option::<String>::None);
    let show_clear_confirm = use_state(|| false);

    let on_modal_open = {
        let show_modal = show_modal.clone();
        let json_content = json_content.clone();
        let layout = props.layout.clone();
        let import_error = import_error.clone();
        Callback::from(move |_: MouseEvent| {
            let export_json = layout
                .to_json_pretty()
                .unwrap_or_else(|e| format!("Error generating JSON: {}", e));
            json_content.set(export_json);
            import_error.set(None);
            show_modal.set(true);
        })
    };

    let on_json_change = {
        let json_content = json_content.clone();
        let import_error = import_error.clone();
        Callback::from(move |e: InputEvent| {
            let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            json_content.set(textarea.value());
            import_error.set(None); // Clear error when user types
        })
    };

    let on_load_click = {
        let json_content = json_content.clone();
        let on_import = props.on_import.clone();
        let import_error = import_error.clone();
        let show_modal = show_modal.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            let json = (*json_content).clone();

            // Validate JSON before importing
            match Layout::from_json(&json) {
                Ok(_) => {
                    on_import.emit(json);
                    import_error.set(None);
                    show_modal.set(false);
                }
                Err(err) => {
                    import_error.set(Some(format!("Invalid JSON: {}", err)));
                }
            }
        })
    };

    let on_copy_json = {
        let json_content = json_content.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            // In a real implementation, we'd use the clipboard API
            log::info!("Copy JSON: {}", *json_content);
        })
    };

    let on_close_modal = {
        let show_modal = show_modal.clone();
        Callback::from(move |_: MouseEvent| {
            show_modal.set(false);
        })
    };

    let on_clear_click = {
        let show_clear_confirm = show_clear_confirm.clone();
        Callback::from(move |_: MouseEvent| {
            show_clear_confirm.set(true);
        })
    };

    let on_clear_cancel = {
        let show_clear_confirm = show_clear_confirm.clone();
        Callback::from(move |_: MouseEvent| {
            show_clear_confirm.set(false);
        })
    };

    let on_clear_confirm = {
        let show_clear_confirm = show_clear_confirm.clone();
        let on_clear = props.on_clear.clone();
        Callback::from(move |_: MouseEvent| {
            show_clear_confirm.set(false);
            on_clear.emit(());
        })
    };

    html! {
        <>
            <div
                class="wysiwyg-toolbar"
                style="
                    height: 50px;
                    background: #ffffff;
                    border-bottom: 1px solid #e5e7eb;
                    padding: 0 16px;
                    display: flex;
                    align-items: center;
                    gap: 12px;
                "
            >
                <h2 style="margin: 0; font-size: 18px; font-weight: 600; flex: 1;">
                    { "Page Editor" }
                </h2>

                <button
                    onclick={props.on_undo.reform(|_| ())}
                    disabled={!props.can_undo}
                    style={format!("
                        padding: 8px 12px;
                        background: {};
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: {};
                        font-size: 14px;
                        font-weight: 500;
                        opacity: {};
                    ", if props.can_undo { "#3b82f6" } else { "#d1d5db" },
                       if props.can_undo { "pointer" } else { "not-allowed" },
                       if props.can_undo { "1" } else { "0.6" })}
                    title="Undo (Ctrl+Z)"
                >
                    { "↶ Undo" }
                </button>

                <button
                    onclick={props.on_redo.reform(|_| ())}
                    disabled={!props.can_redo}
                    style={format!("
                        padding: 8px 12px;
                        background: {};
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: {};
                        font-size: 14px;
                        font-weight: 500;
                        opacity: {};
                    ", if props.can_redo { "#3b82f6" } else { "#d1d5db" },
                       if props.can_redo { "pointer" } else { "not-allowed" },
                       if props.can_redo { "1" } else { "0.6" })}
                    title="Redo (Ctrl+Y)"
                >
                    { "↷ Redo" }
                </button>

                <button
                    onclick={on_modal_open}
                    style="
                        padding: 8px 16px;
                        background: #3b82f6;
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                        font-size: 14px;
                        font-weight: 500;
                    "
                >
                    { "Import/Export" }
                </button>

                <button
                    onclick={on_clear_click}
                    style="
                        padding: 8px 16px;
                        background: #ef4444;
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                        font-size: 14px;
                        font-weight: 500;
                    "
                    title="Clear all widgets and start fresh"
                >
                    { "Clear" }
                </button>

                <button
                    onclick={props.on_toggle_edit_mode.reform(|_| ())}
                    style={format!("
                        padding: 8px 16px;
                        background: {};
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                        font-size: 14px;
                        font-weight: 500;
                    ", if props.edit_mode { "#10b981" } else { "#6b7280" })}
                >
                    { if props.edit_mode { "Preview" } else { "Edit" } }
                </button>

                <div style="
                    padding: 8px 12px;
                    background: #f3f4f6;
                    border-radius: 4px;
                    font-size: 13px;
                    color: #6b7280;
                ">
                    { format!("{} widgets", props.layout.to_serialized().nodes.len()) }
                </div>
            </div>

            // Import/Export modal
            if *show_modal {
                <div
                    style="
                        position: fixed;
                        top: 0;
                        left: 0;
                        right: 0;
                        bottom: 0;
                        background: rgba(0, 0, 0, 0.5);
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        z-index: 1000;
                    "
                    onclick={on_close_modal.clone()}
                >
                    <div
                        style="
                            background: white;
                            border-radius: 8px;
                            padding: 24px;
                            max-width: 600px;
                            width: 90%;
                            max-height: 80vh;
                            display: flex;
                            flex-direction: column;
                            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
                        "
                        onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                    >
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 16px;">
                            <h3 style="margin: 0; font-size: 18px; font-weight: 600;">
                                { "Import/Export Layout" }
                            </h3>
                            <button
                                onclick={on_close_modal.clone()}
                                style="
                                    background: none;
                                    border: none;
                                    font-size: 24px;
                                    cursor: pointer;
                                    color: #6b7280;
                                    padding: 0;
                                    width: 32px;
                                    height: 32px;
                                "
                            >
                                { "×" }
                            </button>
                        </div>

                        <p style="margin: 0 0 12px 0; color: #6b7280; font-size: 14px;">
                            { "Copy the JSON below to export, or paste JSON and click Load to import." }
                        </p>

                        <textarea
                            value={(*json_content).clone()}
                            oninput={on_json_change}
                            style="
                                flex: 1;
                                font-family: monospace;
                                font-size: 12px;
                                padding: 12px;
                                border: 1px solid #e5e7eb;
                                border-radius: 4px;
                                resize: none;
                                margin-bottom: 8px;
                            "
                        />

                        if let Some(error) = (*import_error).clone() {
                            <div style="
                                padding: 8px 12px;
                                background: #fef2f2;
                                border: 1px solid #fecaca;
                                border-radius: 4px;
                                color: #dc2626;
                                font-size: 13px;
                                margin-bottom: 12px;
                            ">
                                { error }
                            </div>
                        }

                        <div style="display: flex; gap: 8px; justify-content: flex-end;">
                            <button
                                onclick={on_copy_json}
                                style="
                                    padding: 8px 16px;
                                    background: #6b7280;
                                    color: white;
                                    border: none;
                                    border-radius: 4px;
                                    cursor: pointer;
                                    font-size: 14px;
                                "
                            >
                                { "Copy" }
                            </button>
                            <button
                                onclick={on_load_click}
                                style="
                                    padding: 8px 16px;
                                    background: #10b981;
                                    color: white;
                                    border: none;
                                    border-radius: 4px;
                                    cursor: pointer;
                                    font-size: 14px;
                                    font-weight: 500;
                                "
                            >
                                { "Load" }
                            </button>
                            <button
                                onclick={on_close_modal.clone()}
                                style="
                                    padding: 8px 16px;
                                    background: #f3f4f6;
                                    color: #374151;
                                    border: none;
                                    border-radius: 4px;
                                    cursor: pointer;
                                    font-size: 14px;
                                "
                            >
                                { "Close" }
                            </button>
                        </div>
                    </div>
                </div>
            }

            // Clear confirmation modal
            if *show_clear_confirm {
                <div
                    style="
                        position: fixed;
                        top: 0;
                        left: 0;
                        right: 0;
                        bottom: 0;
                        background: rgba(0, 0, 0, 0.5);
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        z-index: 1000;
                    "
                    onclick={on_clear_cancel.clone()}
                >
                    <div
                        style="
                            background: white;
                            border-radius: 8px;
                            padding: 24px;
                            max-width: 400px;
                            width: 90%;
                            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
                        "
                        onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                    >
                        <h3 style="margin: 0 0 16px 0; font-size: 18px; font-weight: 600; color: #111827;">
                            { "Clear All Widgets?" }
                        </h3>
                        <p style="margin: 0 0 24px 0; color: #6b7280; line-height: 1.5;">
                            { "This will remove all widgets from the editor and clear the saved layout. This action cannot be undone." }
                        </p>
                        <div style="display: flex; gap: 12px; justify-content: flex-end;">
                            <button
                                onclick={on_clear_cancel}
                                style="
                                    padding: 8px 16px;
                                    background: #f3f4f6;
                                    color: #374151;
                                    border: none;
                                    border-radius: 4px;
                                    cursor: pointer;
                                    font-size: 14px;
                                    font-weight: 500;
                                "
                            >
                                { "Cancel" }
                            </button>
                            <button
                                onclick={on_clear_confirm}
                                style="
                                    padding: 8px 16px;
                                    background: #ef4444;
                                    color: white;
                                    border: none;
                                    border-radius: 4px;
                                    cursor: pointer;
                                    font-size: 14px;
                                    font-weight: 500;
                                "
                            >
                                { "Clear All" }
                            </button>
                        </div>
                    </div>
                </div>
            }
        </>
    }
}
