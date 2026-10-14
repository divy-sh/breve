use dioxus::prelude::*;
use lucide_dioxus::{ArrowDown, Book, Settings as SettingsIcon};

use crate::ui::components::dialog::{
    Dialog, DialogBody, DialogContent, DialogHeader, DialogTitle, DialogTrigger,
};
use crate::ui::composables::use_conversation::UseConversation;
use crate::ui::composables::use_model::use_model;
use crate::ui::panels::chat::Chat;
use crate::ui::panels::conversations::Conversations;
use crate::ui::panels::model_picker::ModelPicker;
use crate::ui::panels::settings::Settings;
use crate::ui::panels::top_bar::{TopBar, TopBarLeft, TopBarRight, TopBarTitle};

#[component]
pub fn Home(conversation_composable: UseConversation, theme: Signal<bool>) -> Element {
    let model_state = use_model();

    rsx! {
        div { class: "flex flex-col h-screen w-full overflow-hidden",
            TopBar { class: "gap-4",
                // Left side: Chat History Dialog
                TopBarLeft { class: "shrink-0",
                    Dialog {
                        DialogTrigger {
                            Book { class: "w-4 h-4" }
                        }
                        DialogContent { class: "flex flex-col overflow-hidden",
                            DialogHeader { class: "shrink-0",
                                DialogTitle { "Chat History" }
                            }
                            DialogBody { class: "flex-1 min-h-0 overflow-hidden",
                                Conversations { conversation_composable }
                            }
                        }
                    }
                }

                TopBarTitle { class: "min-w-0 flex-1 flex justify-center",
                    Dialog { class: "w-full min-w-0",
                        DialogTrigger { class: "w-full min-w-0",
                            span { class: "flex-1 min-w-0 truncate text-center",
                                "{model_state.default_model}"
                            }
                            ArrowDown { class: "w-4 h-4 shrink-0" }
                        }
                        DialogContent { class: "flex flex-col h-[85vh] overflow-hidden",
                            DialogHeader {
                                DialogTitle { "Models" }
                            }
                            DialogBody { class: "flex-1 min-h-0 overflow-hidden",
                                div { class: "flex flex-col flex-1 min-h-0 w-full overflow-hidden",
                                    ModelPicker { on_model_selected: move |_| {} }
                                }
                            }
                        }
                    }
                }

                // Right side: Settings Dialog
                TopBarRight { class: "shrink-0",
                    Dialog {
                        DialogTrigger {
                            SettingsIcon { class: "w-4 h-4" }
                        }
                        DialogContent { class: "flex flex-col max-h-[85vh] overflow-hidden",
                            DialogHeader {
                                DialogTitle { "Settings" }
                            }
                            DialogBody { class: "min-h-0 overflow-y-auto",
                                Settings { theme }
                            }
                        }
                    }
                }
            }
            Chat { conv_state: conversation_composable }
        }
    }
}
