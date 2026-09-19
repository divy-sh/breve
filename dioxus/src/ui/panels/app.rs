use crate::ui::components::{
    avatar::{Avatar, AvatarFallback, AvatarImage},
    bubble::{Bubble, BubbleContent},
    message::{Message, MessageAvatar, MessageContent},
};

use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        Message {
            MessageAvatar {
                Avatar {
                    AvatarImage { src: "/avatars/02.png", alt: "@avatar" }
                    AvatarFallback { "CN" }
                }
            }
            MessageContent {
                Bubble {
                    BubbleContent { "How can I help you today?" }
                }
            }
        }
    }
}
