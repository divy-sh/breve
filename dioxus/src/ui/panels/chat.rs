use dioxus::prelude::*;
use lucide_dioxus::Send;

// Import your custom hook here
use crate::ui::composables::use_conversation::use_conversation;

use crate::ui::components::{
    avatar::{Avatar, AvatarFallback},
    bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant},
    input_prompt::{InputPrompt, InputPromptSubmit, InputPromptTextarea, InputPromptTools},
    message::{Message, MessageAlign, MessageAvatar, MessageContent},
    skeleton::Skeleton,
};

/// A single-conversation chat UI driven by the `use_conversation` hook.
#[component]
pub fn Chat() -> Element {
    let conv_state = use_conversation();
    let mut input_value = use_signal(String::new);

    let mut handle_send = move || {
        let text = input_value().trim().to_string();
        if text.is_empty() || conv_state.is_loading() {
            return;
        }

        input_value.set(String::new());
        conv_state.send(text);
    };

    rsx! {
        // Ensure the root container is constrained properly
        div { class: "flex flex-col h-full w-full overflow-hidden",
            div { class: "flex-1 min-h-0 overflow-y-auto p-4 flex flex-col gap-3",
                if let Some(conv) = conv_state.conversation() {
                    for msg in conv.body {
                        {
                            let is_user = msg.role == "user";
                            let align = if is_user { MessageAlign::End } else { MessageAlign::Start };
                            let bubble_align = if is_user { BubbleAlign::End } else { BubbleAlign::Start };
                            let variant = if is_user { BubbleVariant::Default } else { BubbleVariant::Secondary };
                            let initials = if is_user { "You" } else { "AI" };

                            rsx! {
                                Message { align,
                                    MessageAvatar { Avatar { AvatarFallback { "{initials}" } } }
                                    MessageContent {
                                        Bubble { variant, align: bubble_align,
                                            BubbleContent { "{msg.content}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if conv_state.is_loading() {
                        Message { align: MessageAlign::Start,
                            MessageAvatar { Avatar { AvatarFallback { "AI" } } }
                            MessageContent {
                                Bubble { variant: BubbleVariant::Secondary, align: BubbleAlign::Start,
                                    BubbleContent {
                                        if conv_state.streaming().is_empty() {
                                            Skeleton { class: "h-4 w-24" }
                                        } else {
                                            "{conv_state.streaming()}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "flex flex-1 items-center justify-center text-center",
                        div {
                            p { class: "text-lg font-semibold", "Hello there!" }
                            p { class: "text-sm text-muted-foreground", "Start a new conversation by typing a message below." }
                        }
                    }
                }
            }

            div { class: "p-4 border-t border-border shrink-0",
                InputPrompt {
                    InputPromptTextarea {
                        value: input_value,
                        placeholder: "Write a message...",
                        on_submit: move |_| handle_send(),
                    }
                    InputPromptTools {}
                    InputPromptSubmit {
                        disabled: input_value().trim().is_empty() || conv_state.is_loading(),
                        onclick: move |_| handle_send(),
                        Send { }
                    }
                }
            }
        }
    }
}
