use dioxus::document::eval;
use dioxus::prelude::*;
use dioxus_mdx::MdxContent;
use lucide_dioxus::Send;

use crate::ui::composables::use_conversation::UseConversation;

use crate::ui::components::{
    bubble::{Bubble, BubbleAlign, BubbleContent, BubbleVariant},
    input_prompt::{InputPrompt, InputPromptSubmit, InputPromptTextarea, InputPromptTools},
    message::{Message, MessageAlign, MessageContent},
    skeleton::Skeleton,
};

const ASSISTANT_MARKDOWN_CLASS: &str = concat!(
    "text-sm leading-relaxed break-words ",
    "[&_p+p]:mt-3 [&_h1]:my-3 [&_h1]:text-2xl [&_h1]:font-semibold ",
    "[&_h2]:my-3 [&_h2]:text-xl [&_h2]:font-semibold ",
    "[&_h3]:my-2 [&_h3]:text-lg [&_h3]:font-semibold ",
    "[&_ul]:my-2 [&_ul]:list-disc [&_ul]:pl-5 ",
    "[&_ol]:my-2 [&_ol]:list-decimal [&_ol]:pl-5 [&_li]:my-1 ",
    "[&_blockquote]:my-3 [&_blockquote]:border-l-2 [&_blockquote]:border-border ",
    "[&_blockquote]:pl-3 [&_blockquote]:text-muted-foreground ",
    "[&_a]:text-primary [&_a]:underline [&_a]:underline-offset-2 ",
    "[&_pre]:my-3 [&_pre]:overflow-x-auto [&_pre]:rounded-md [&_pre]:bg-muted [&_pre]:p-3 ",
    "[&_code]:font-mono [&_code]:text-[0.9em] ",
    "[&_code:not(pre_code)]:rounded [&_code:not(pre_code)]:bg-muted ",
    "[&_code:not(pre_code)]:px-1 [&_code:not(pre_code)]:py-0.5 ",
    "[&_table]:my-3 [&_table]:w-full [&_table]:border-collapse ",
    "[&_th]:border [&_th]:border-border [&_th]:bg-muted [&_th]:p-2 ",
    "[&_td]:border [&_td]:border-border [&_td]:p-2 [&_hr]:my-4 ",
    "[&_.dk-code-block]:my-3 [&_.dk-code-block]:max-w-full ",
    "[&_.dk-code-block]:overflow-x-auto [&_.dk-code-block]:rounded-lg ",
    "[&_.dk-code-block]:border [&_.dk-code-block]:border-border [&_.dk-code-block]:bg-muted ",
    "[&_.dk-code-block-body]:bg-muted ",
    "[&_.hl-keyword]:text-purple-600 dark:[&_.hl-keyword]:text-purple-300 ",
    "[&_.hl-string]:text-green-700 dark:[&_.hl-string]:text-green-300 ",
    "[&_.hl-comment]:text-muted-foreground ",
    "[&_.hl-number]:text-amber-700 dark:[&_.hl-number]:text-amber-300 ",
    "[&_.hl-type]:text-blue-700 dark:[&_.hl-type]:text-blue-300 ",
    "[&_.hl-function]:text-indigo-700 dark:[&_.hl-function]:text-indigo-300"
);

#[component]
fn AssistantMarkdown(content: String) -> Element {
    rsx! {
        div { class: ASSISTANT_MARKDOWN_CLASS, MdxContent { content } }
    }
}

/// A single-conversation chat UI driven by the `use_conversation` hook.
#[component]
pub fn Chat(conv_state: UseConversation) -> Element {
    let mut input_value = use_signal(String::new);

    use_effect(move || {
        let _ = conv_state.conversation();
        let _ = conv_state.is_loading();
        let _ = conv_state.streaming();
        eval(
            r#"
            const container = document.getElementById('chat-messages');
            if (container) {
                if (container.dataset.autoScrollInitialized !== 'true') {
                    container.dataset.autoScrollInitialized = 'true';
                    container.dataset.following = 'true';
                    container.addEventListener('scroll', () => {
                        const distanceFromBottom =
                            container.scrollHeight - container.scrollTop - container.clientHeight;
                        container.dataset.following = distanceFromBottom < 2 ? 'true' : 'false';
                    }, { passive: true });
                    container.addEventListener('wheel', (event) => {
                        if (event.deltaY < 0) {
                            container.dataset.following = 'false';
                        }
                    }, { passive: true });
                }
                if (container.dataset.following === 'true') {
                    container.scrollTop = container.scrollHeight;
                }
            }
            "#,
        );
    });

    let mut handle_send = move || {
        let text = input_value().trim().to_string();
        if text.is_empty() || conv_state.is_loading() {
            return;
        }

        input_value.set(String::new());
        conv_state.send_message(text);
    };

    rsx! {
        // Ensure the root container is constrained properly
        div { class: "flex flex-col h-full w-full overflow-hidden",
            div { id: "chat-messages", class: "flex-1 min-h-0 overflow-y-auto p-4 flex flex-col gap-3",
                if let Some(conv) = conv_state.conversation() {
                    for msg in conv.body {
                        {
                            let is_user = msg.role == "user";
                            let align = if is_user { MessageAlign::End } else { MessageAlign::Start };
                            let bubble_align = if is_user { BubbleAlign::End } else { BubbleAlign::Start };
                            let variant = if is_user {
                                BubbleVariant::Default
                            } else {
                                BubbleVariant::Secondary
                            };
                            rsx! {
                                Message { align,
                                    MessageContent {
                                        Bubble { variant, align: bubble_align,
                                            BubbleContent {
                                                if is_user {
                                                    "{msg.content}"
                                                } else {
                                                    AssistantMarkdown { content: msg.content.clone() }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if conv_state.is_loading() {
                        Message { align: MessageAlign::Start,
                            MessageContent {
                                Bubble {
                                    variant: BubbleVariant::Secondary,
                                    align: BubbleAlign::Start,
                                    BubbleContent {
                                        if conv_state.streaming().is_empty() {
                                            Skeleton { class: "h-4 w-24" }
                                        } else {
                                            AssistantMarkdown { content: conv_state.streaming() }
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
                            p { class: "text-sm text-muted-foreground",
                                "Start a new conversation by typing a message below."
                            }
                        }
                    }
                }
                div { id: "chat-bottom", class: "shrink-0" }
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
                        Send {}
                    }
                }
            }
        }
    }
}
