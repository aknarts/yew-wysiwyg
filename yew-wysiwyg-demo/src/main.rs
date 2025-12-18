use yew::prelude::*;
use yew_wysiwyg::Editor;

#[function_component(App)]
fn app() -> Html {
    // Initialize logger
    wasm_logger::init(wasm_logger::Config::default());

    html! {
        <div style="width: 100vw; height: 100vh; margin: 0; padding: 0;">
            <Editor />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
