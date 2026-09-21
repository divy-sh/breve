use dioxus::prelude::*;
use lucide_dioxus::Send;

use crate::core::conversation::{
    controller as conversation_ctrl,
    models::{Conversation, Message as ChatMessage},
};
use crate::ui::components::{
    avatar::{Avatar, AvatarFallback},
    bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant},
    input_prompt::{
        InputPrompt, InputPromptFooter, InputPromptSubmit, InputPromptTextarea, InputPromptTools,
    },
    message::{Message, MessageAlign, MessageAvatar, MessageContent},
    skeleton::Skeleton,
};

/// A single-conversation chat UI: renders the message history, a streaming
/// bubble while the model is replying, and a text input to send new
/// messages.
#[component]
pub fn Chat() -> Element {
    let mut conversation: Signal<Option<Conversation>> = use_signal(|| None);
    let mut is_loading = use_signal(|| false);
    let mut streaming = use_signal(String::new);
    let mut input_value = use_signal(String::new);

    let mut send_message = move |text: String| {
        let text = text.trim().to_string();
        if text.is_empty() || is_loading() {
            return;
        }
        input_value.set(String::new());
        is_loading.set(true);

        spawn(async move {
            let conv_id = match conversation() {
                Some(conv) => conv.id,
                None => {
                    let title: String = text.chars().take(30).collect();
                    match conversation_ctrl::start_conversation(title) {
                        Ok(id) => {
                            if let Ok(loaded) = conversation_ctrl::get_conversation(id.clone()) {
                                conversation.set(loaded);
                            }
                            id
                        }
                        Err(_) => {
                            is_loading.set(false);
                            return;
                        }
                    }
                }
            };

            // Show the user's message immediately, without waiting for the
            // model to finish replying.
            if let Some(mut conv) = conversation() {
                conv.body.push(ChatMessage {
                    role: "user".to_string(),
                    content: text.clone(),
                });
                conversation.set(Some(conv));
            }

            streaming.set(String::new());

            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
            let handle = {
                let conv_id = conv_id.clone();
                let text = text.clone();
                tokio::task::spawn_blocking(move || {
                    conversation_ctrl::continue_conversation(conv_id, text, move |token: &str| {
                        let _ = tx.send(token.to_string());
                    })
                })
            };

            while let Some(token) = rx.recv().await {
                streaming.write().push_str(&token);
            }

            let _ = handle.await;

            if let Ok(loaded) = conversation_ctrl::get_conversation(conv_id) {
                conversation.set(loaded);
            }
            streaming.set(String::new());
            is_loading.set(false);
        });
    };

    rsx! {
        div { class: "flex flex-col h-screen w-full",
            div { class: "flex-1 overflow-y-auto p-4 flex flex-col gap-3",
                if let Some(conv) = conversation() {
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
                    if is_loading() {
                        Message { align: MessageAlign::Start,
                            MessageAvatar { Avatar { AvatarFallback { "AI" } } }
                            MessageContent {
                                Bubble { variant: BubbleVariant::Secondary, align: BubbleAlign::Start,
                                    BubbleContent {
                                        if streaming().is_empty() {
                                            Skeleton { class: "h-4 w-24" }
                                        } else {
                                            "{streaming()}"
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
            div { class: "p-4 border-t border-border",
                InputPrompt {
                    InputPromptTextarea {
                        value: input_value,
                        placeholder: "Write a message...",
                        on_submit: move |_| send_message(input_value()),
                    }
                    InputPromptFooter {
                        InputPromptTools {}
                        InputPromptSubmit {
                            disabled: input_value().trim().is_empty() || is_loading(),
                            onclick: move |_| send_message(input_value()),
                            Send {}
                        }
                    }
                }
            }
        }
    }
}
