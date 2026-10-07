use dioxus::prelude::*;
use lucide_dioxus::{Plus, Trash2};

use crate::ui::components::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::components::button_group::ButtonGroup;
use crate::ui::components::dialog::DialogClose;
use crate::ui::composables::use_conversation::UseConversation;

#[component]
pub fn Conversations(conversation_composable: UseConversation) -> Element {
    let conversations = (conversation_composable.list)();
    let current_id = conversation_composable
        .conversation()
        .map(|conversation| conversation.id);

    rsx! {
        div { class: "flex flex-col flex-1 min-h-0 w-full gap-4 overflow-hidden",
            div {
                onclick: {
                    let mut state = conversation_composable;
                    move |_| state.new_conversation()
                },
                DialogClose { class: "w-full justify-start",
                    Plus {}
                    "New Chat"
                }
            }

            if let Some(message) = (conversation_composable.error)() {
                p { class: "shrink-0 text-sm", "{message}" }
            }

            if conversations.is_empty() {
                p { class: "shrink-0 text-center text-sm", "No conversations yet" }
            } else {
                div { class: "flex flex-col flex-1 min-h-0 gap-4 overflow-y-auto",
                    for conversation in conversations {
                        {
                            let id = conversation.id.clone();
                            let title = if conversation.title.trim().is_empty() {
                                "Untitled Chat".to_string()
                            } else {
                                conversation.title.clone()
                            };
                            let is_active = current_id.as_deref() == Some(id.as_str());
                            let is_deleting = (conversation_composable.deleting)()
                                .as_deref()
                                == Some(id.as_str());
                            let mut selection_state = conversation_composable;
                            let mut deletion_state = conversation_composable;

                            rsx! {
                                div {
                                    onclick: move |_| {
                                        selection_state.select_conversation(conversation.clone());
                                    },
                                    ButtonGroup { class: "w-full shrink-0",
                                        DialogClose {
                                            class: if is_active {
                                                "min-w-0 flex-1 justify-start font-semibold"
                                            } else {
                                                "min-w-0 flex-1 justify-start"
                                            },
                                            "{title}"
                                        }
                                        Button {
                                            variant: ButtonVariant::Outline,
                                            size: ButtonSize::Icon,
                                            aria_label: "Delete conversation",
                                            title: "Delete conversation",
                                            disabled: is_deleting,
                                            onclick: move |event: MouseEvent| {
                                                event.stop_propagation();
                                                deletion_state.delete_conversation(id.clone());
                                            },
                                            Trash2 {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
