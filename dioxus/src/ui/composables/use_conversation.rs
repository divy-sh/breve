use crate::core::conversation::controller as conv_controller;
use crate::types::conversation::Conversation;
use dioxus::prelude::*;

async fn run_blocking<T>(
    task: impl FnOnce() -> Result<T, String> + Send + 'static,
) -> Result<T, String>
where
    T: Send + 'static,
{
    tokio::task::spawn_blocking(task)
        .await
        .map_err(|error| error.to_string())?
}

#[derive(Clone, Copy, PartialEq)]
pub struct UseConversation {
    pub current: Signal<Option<Conversation>>,
    pub list: Signal<Vec<Conversation>>,
    pub is_loading: Signal<bool>,
    pub streaming: Signal<String>,
    pub deleting: Signal<Option<String>>,
    pub error: Signal<Option<String>>,
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

    pub fn new_conversation(&mut self) {
        self.current.set(None);
        self.error.set(None);
    }

    pub fn select_conversation(&mut self, conversation: Conversation) {
        let id = conversation.id.clone();
        self.current.set(Some(conversation));
        self.error.set(None);

        let mut error = self.error;
        spawn(async move {
            if let Err(message) =
                crate::ui::composables::use_settings::set_config("lastConversationId".into(), id)
                    .await
            {
                error.set(Some(format!(
                    "Could not save selected conversation: {message}"
                )));
            }
        });
    }

    pub fn delete_conversation(&mut self, id: String) {
        let deleted_id = id.clone();
        self.error.set(None);
        self.deleting.set(Some(id.clone()));
        let mut state = *self;

        spawn(async move {
            match run_blocking(move || conv_controller::delete_conversation(id)).await {
                Ok(_) => {
                    if state
                        .conversation()
                        .is_some_and(|current| current.id == deleted_id)
                    {
                        state.current.set(None);
                    }
                    state
                        .list
                        .write()
                        .retain(|conversation| conversation.id != deleted_id);
                }
                Err(message) => state
                    .error
                    .set(Some(format!("Could not delete conversation: {message}"))),
            }
            state.deleting.set(None);
        });
    }

    fn upsert_conversation(&mut self, conversation: Conversation) {
        let mut conversations = (self.list)();
        conversations.retain(|existing| existing.id != conversation.id);
        conversations.insert(0, conversation);
        self.list.set(conversations);
    }

    pub fn send_message(&mut self, text: String) {
        let mut state = *self;
        let text = text.trim().to_string();
        if text.is_empty() || state.is_loading() {
            return;
        }

        if let Some(mut conv) = state.conversation() {
            conv.body.push(crate::types::conversation::Message {
                role: "user".to_string(),
                content: text.clone(),
            });
            state.current.set(Some(conv.clone()));
            state.upsert_conversation(conv);
        }

        state.error.set(None);
        state.is_loading.set(true);

        spawn(async move {
            send_message_task(state, text).await;
        });
    }
}

async fn send_message_task(mut state: UseConversation, text: String) {
    let conv_id = match state.conversation() {
        Some(conversation) => conversation.id,
        None => {
            let title = text.chars().take(30).collect::<String>();
            let conversation_title = title.clone();
            match run_blocking(move || conv_controller::start_new_conversation(&conversation_title))
                .await
            {
                Ok(id) => {
                    let mut conversation = Conversation::new(id.clone(), title);
                    conversation.body.push(crate::types::conversation::Message {
                        role: "user".to_string(),
                        content: text.clone(),
                    });
                    state.current.set(Some(conversation.clone()));
                    state.upsert_conversation(conversation);
                    id
                }
                Err(message) => {
                    state
                        .error
                        .set(Some(format!("Could not create conversation: {message}")));
                    state.is_loading.set(false);
                    return;
                }
            }
        }
    };

    state.streaming.set(String::new());
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    let send_id = conv_id.clone();
    let send_text = text.clone();
    let send = tokio::task::spawn_blocking(move || {
        conv_controller::continue_conversation(send_id, send_text, move |token| {
            let _ = tx.send(token.to_string());
        })
    });

    while let Some(token) = rx.recv().await {
        state.streaming.write().push_str(&token);
    }

    if let Err(message) = send
        .await
        .map_err(|error| error.to_string())
        .and_then(|result| result)
    {
        state
            .error
            .set(Some(format!("Could not send message: {message}")));
    }

    match run_blocking(move || conv_controller::get_conversation(conv_id)).await {
        Ok(Some(conversation)) => state.current.set(Some(conversation)),
        Ok(None) => state
            .error
            .set(Some("Sent message could not be reloaded.".to_string())),
        Err(message) => state
            .error
            .set(Some(format!("Could not reload conversation: {message}"))),
    }

    match run_blocking(conv_controller::get_all_conversations).await {
        Ok(conversations) => state.list.set(conversations),
        Err(message) => state.error.set(Some(format!(
            "Message sent, but conversations could not be refreshed: {message}"
        ))),
    }

    state.streaming.set(String::new());
    state.is_loading.set(false);
}

pub fn use_conversation() -> UseConversation {
    let current = use_signal(|| None);
    let mut list = use_signal(Vec::new);
    let is_loading = use_signal(|| false);
    let streaming = use_signal(String::new);
    let deleting = use_signal(|| None);
    let error = use_signal(|| None);

    // Initial load using get_all_conversations
    use_effect(move || {
        spawn(async move {
            if let Ok(conversations) = run_blocking(conv_controller::get_all_conversations).await {
                list.set(conversations);
            }
        });
    });

    UseConversation {
        current,
        list,
        is_loading,
        streaming,
        deleting,
        error,
    }
}
