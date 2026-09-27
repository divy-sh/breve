use crate::core::conversation::controller as conv_controller;
use crate::types::conversation::Conversation;
use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct UseConversation {
    pub current: Signal<Option<Conversation>>,
    pub list: Signal<Vec<Conversation>>,
    pub is_loading: Signal<bool>,
    pub streaming: Signal<String>,
}

impl UseConversation {
    pub fn is_loading(&self) -> bool {
        (self.is_loading)()
    }

    pub fn conversation(&self) -> Option<Conversation> {
        (self.current)()
    }

    pub fn streaming(&self) -> String {
        (self.streaming)()
    }

    pub fn send(&self, text: String) {
        send_message(*self, text);
    }
}

pub fn use_conversation() -> UseConversation {
    let current = use_signal(|| None);
    let mut list = use_signal(Vec::new);
    let is_loading = use_signal(|| false);
    let streaming = use_signal(String::new);

    // Initial load using get_all_conversations
    use_effect(move || {
        spawn(async move {
            if let Ok(convs) =
                tokio::task::spawn_blocking(conv_controller::get_all_conversations).await
            {
                if let Ok(convs) = convs {
                    list.set(convs);
                }
            }
        });
    });

    UseConversation {
        current,
        list,
        is_loading,
        streaming,
    }
}

pub fn send_message(mut state: UseConversation, text: String) {
    let text = text.trim().to_string();
    if text.is_empty() || state.is_loading() {
        return;
    }

    state.is_loading.set(true);

    spawn(async move {
        let current_conv = state.conversation();
        let conv_id = match current_conv {
            Some(ref conv) => conv.id.clone(),
            None => {
                let title: String = text.chars().take(30).collect();
                let title_clone = title.clone();

                let result = tokio::task::spawn_blocking(move || {
                    conv_controller::start_new_conversation(&title_clone)
                })
                .await;

                match result {
                    Ok(Ok(id)) => {
                        let id_clone = id.clone();
                        if let Ok(Ok(Some(loaded))) = tokio::task::spawn_blocking(move || {
                            conv_controller::get_conversation(id_clone)
                        })
                        .await
                        {
                            state.current.set(Some(loaded));
                        }
                        id
                    }
                    _ => {
                        state.is_loading.set(false);
                        return;
                    }
                }
            }
        };

        state.streaming.set(String::new());

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
        let conv_id_task = conv_id.clone();
        let text_task = text.clone();

        let handle = tokio::task::spawn_blocking(move || {
            conv_controller::continue_conversation(conv_id_task, text_task, move |token: &str| {
                let _ = tx.send(token.to_string());
            })
        });

        while let Some(token) = rx.recv().await {
            state.streaming.write().push_str(&token);
        }

        let _ = handle.await;

        let conv_id_fetch = conv_id.clone();
        if let Ok(Ok(Some(loaded))) =
            tokio::task::spawn_blocking(move || conv_controller::get_conversation(conv_id_fetch))
                .await
        {
            state.current.set(Some(loaded));
        }

        // Refresh sidebar list
        if let Ok(Ok(convs)) =
            tokio::task::spawn_blocking(conv_controller::get_all_conversations).await
        {
            state.list.set(convs);
        }

        state.streaming.set(String::new());
        state.is_loading.set(false);
    });
}
