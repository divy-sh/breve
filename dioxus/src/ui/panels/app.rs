use dioxus::prelude::*;

use crate::core::models::controller as models_ctrl;
use crate::ui::panels::{chat::Chat, model_picker::ModelPicker};

/// Root application panel: shows the model picker until a model is
/// active, then switches to the chat UI.
#[component]
pub fn App() -> Element {
    let mut model_ready = use_signal(|| models_ctrl::get_model_status() == "SET");

    rsx! {
        if model_ready() {
            Chat {}
        } else {
            ModelPicker { on_model_selected: move |_| model_ready.set(true) }
        }
    }
}
