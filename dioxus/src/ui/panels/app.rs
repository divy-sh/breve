use dioxus::prelude::*;

use crate::core::models::controller as models_ctrl;

use crate::ui::components::sidenav::{
    Sidenav, SidenavContent, SidenavFooter, SidenavGroup, SidenavGroupContent, SidenavHeader,
    SidenavInner, SidenavInset, SidenavMenu, SidenavMenuButton, SidenavMenuItem,
    SidenavResizeHandle, SidenavTrigger, SidenavWrapper,
};
use crate::ui::composables::use_conversation::use_conversation;
use crate::ui::panels::{chat::Chat, model_picker::ModelPicker};

#[component]
pub fn App() -> Element {
    let mut model_ready = use_signal(|| models_ctrl::get_model_status() == "SET");
    let mut conversation_composable = use_conversation();

    rsx! {
        SidenavWrapper {
            class: "h-screen w-screen",

            Sidenav {
                SidenavInner {
                    SidenavHeader {
                        div { class: "flex items-center justify-between",
                            span { class: "font-semibold text-sm", "Chats" }
                        }
                    }

                    SidenavContent {
                        SidenavGroup {
                            SidenavGroupContent {
                                SidenavMenu {
                                    for conv in conversation_composable.list.read().iter() {
                                        {
                                            let is_current = conversation_composable
                                                .current
                                                .read()
                                                .as_ref()
                                                .map_or(false, |c| c.id == conv.id);

                                            let conv_item = conv.clone();

                                            rsx! {
                                                SidenavMenuItem {
                                                    key: "{conv.id}",
                                                    div {
                                                        class: "w-full cursor-pointer",
                                                        onclick: move |_| {
                                                            conversation_composable.current.set(Some(conv_item.clone()));
                                                        },
                                                        SidenavMenuButton {
                                                            aria_current: if is_current { "page" } else { "false" },
                                                            span { "{conv.title}" }
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
                SidenavResizeHandle {}
            }

            SidenavInset {
                header { class: "flex items-center border-b px-4 h-14 bg-background shrink-0 gap-2",
                    SidenavTrigger {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M3 12h18M3 6h18M3 18h18" }
                        }
                    }
                    span { class: "font-medium text-sm", "AI Chat Assistant" }
                }

                main { class: "flex-1 overflow-hidden flex flex-col",
                    if model_ready() {
                        Chat {}
                    } else {
                        ModelPicker { on_model_selected: move |_| model_ready.set(true) }
                    }
                }
            }
        }
    }
}
