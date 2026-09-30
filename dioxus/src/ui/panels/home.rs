use dioxus::prelude::*;
use lucide_dioxus::{Book, Settings};

use crate::ui::components::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, DialogTrigger,
};
use lucide_dioxus::{Moon, Sun};

use crate::ui::components::button::Button;

use crate::ui::composables::use_conversation::UseConversation;
use crate::ui::panels::chat::Chat;
use crate::ui::panels::model_picker::ModelPicker;
use crate::ui::panels::top_bar::{TopBar, TopBarLeft, TopBarRight, TopBarTitle};

#[component]
pub fn Home(conversation_composable: UseConversation, theme: Signal<bool>) -> Element {
    rsx! {
        div { class: "flex flex-col h-screen w-full overflow-hidden",
            TopBar {
                // Left side: Chat History Dialog
                TopBarLeft {
                    Dialog {
                        DialogTrigger {
                            Book { class: "w-4 h-4" }
                        }
                        DialogContent {
                            DialogHeader {
                                DialogTitle { "Chat History" }
                                DialogDescription { "Your past conversations will appear here." }
                            }
                            DialogBody {
                                div { class: "py-4 text-sm text-muted-foreground", "No past conversations found." }
                            }
                            DialogFooter {
                                DialogClose { "Close" }
                            }
                        }
                    }
                }

                // Center: Title
                TopBarTitle { "{conversation_composable.current.read().as_ref().map(|c| c.id.clone()).unwrap_or_default()}" }

                // Right side: Settings Dialog
                TopBarRight {
                    Dialog {
                        DialogTrigger {
                            Settings { class: "w-4 h-4" }
                        }
                        DialogContent {
                            DialogHeader {
                                DialogTitle { "Settings" }
                                DialogDescription { "Manage your preferences and configurations." }
                            }
                            DialogBody {
                                Button {
                                    class: "fixed bottom-4 right-4 z-50 p-2 text-xs font-medium rounded-full border border-border bg-card text-card-foreground shadow-md cursor-pointer hover:bg-muted",
                                    onclick: move |_| theme.toggle(),
                                    if theme() { Sun {} "Light" } else { Moon {} "Dark" },
                                }
                                ModelPicker { on_model_selected: move |_| {} }
                            }
                            DialogFooter {
                                DialogClose { "Close" }
                            }
                        }
                    }
                }
            }
            Chat {}
        }
    }
}
