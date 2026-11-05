use dioxus::document::eval;
use dioxus::prelude::*;
use lucide_dioxus::{Plus, Trash2};

use crate::ui::components::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::components::dialog::{use_dialog_trigger_id, DialogClose};
use crate::ui::composables::use_conversation::UseConversation;

#[component]
pub fn Conversations(conversation_composable: UseConversation) -> Element {
    let conversations = (conversation_composable.list)();
    let current_id = conversation_composable
        .conversation()
        .map(|conversation| conversation.id);
    let dialog_close_id = use_dialog_trigger_id();

    rsx! {
        div { class: "flex flex-col flex-1 min-h-0 w-full gap-3 overflow-hidden",
            div {
                class: "py-2",
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
                p { class: "shrink-0 text-sm text-destructive", "{message}" }
            }

            div { class: "flex flex-col flex-1 min-h-0 gap-2 overflow-y-auto",
                if conversations.is_empty() {
                    p { class: "shrink-0 rounded-md border border-dashed border-border p-2 text-center text-sm text-muted-foreground",
                        "No conversations yet"
                    }
                } else {
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
                            let close_id = dialog_close_id.clone();

                            rsx! {
                                div {
                                    key: "{id}",
                                    class:
                                    if is_active {
                                        "flex cursor-pointer items-center justify-between gap-2 rounded-md border border-primary bg-secondary p-2 shrink-0 hover:bg-secondary"
                                    } else {
                                        "flex cursor-pointer items-center justify-between gap-2 rounded-md border border-border p-2 shrink-0 hover:bg-secondary"
                                    },
                                    onclick: move |_| {
                                        selection_state.select_conversation(conversation.clone());
                                        if let Some(close_id) = close_id.as_deref() {
                                            eval(
                                                &format!(
                                                    "document.querySelector('#{close_id} [data-dialog-close]')?.click();",
                                                ),
                                            );
                                        }
                                    },
                                    span { class: if is_active { "min-w-0 flex-1 truncate text-sm font-semibold" } else { "min-w-0 flex-1 truncate text-sm" },
                                        "{title}"
                                    }
                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        size: ButtonSize::IconSm,
                                        class: "text-muted-foreground hover:text-destructive",
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
