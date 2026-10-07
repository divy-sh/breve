use dioxus::prelude::*;

use crate::core::models::controller as models_ctrl;
use crate::ui::composables::use_conversation::use_conversation;
use crate::ui::panels::{home::Home, model_picker::ModelPicker};

#[component]
pub fn App(theme: Signal<bool>) -> Element {
    let mut model_ready = use_signal(|| models_ctrl::get_model_status() == "SET");
    let conversation_composable = use_conversation();

    rsx! {
        main { class: "h-screen w-screen overflow-hidden flex flex-col",
            if model_ready() {
                Home { conversation_composable: conversation_composable, theme: theme }
            } else {
                ModelPicker { on_model_selected: move |_| model_ready.set(true) }
            }
        }
    }
}
