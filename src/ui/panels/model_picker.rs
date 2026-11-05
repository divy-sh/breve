use dioxus::prelude::*;
use lucide_dioxus::{Check, CheckCheck, ChevronDown, ChevronUp, Download, LoaderCircle, Trash2};
use std::collections::HashSet;

use crate::ui::components::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::composables::use_model::use_model;

#[component]
pub fn ModelPicker(
    on_model_selected: EventHandler<()>,
    on_model_deleted: EventHandler<()>,
) -> Element {
    let model_state = use_model();
    let expanded_models = use_signal(HashSet::<String>::new);

    rsx! {
        div { class: "flex flex-col flex-1 min-h-0 w-full max-w-lg mx-auto gap-4 overflow-y-auto",
            div { class: "flex flex-col gap-1 shrink-0",
                h1 { class: "text-lg font-semibold", "Choose a model" }
                p { class: "text-sm text-muted-foreground",
                    "Download a model to start chatting. Smaller models are faster to download and run; larger models are more capable."
                }
            }
            if let Some(message) = (model_state.error)() {
                p { class: "shrink-0 text-sm text-destructive", role: "alert", "{message}" }
            }
            for (name , model) in model_state.available() {
                {
                    let is_downloaded = model_state.is_downloaded(&name);
                    let is_default = model_state.is_default(&name);
                    let progress = model_state.progress(&name);
                    let is_selecting = model_state.is_selecting(&name);
                    let is_deleting = model_state.is_deleting(&name);
                    let delete_disabled = !is_downloaded || is_deleting || is_selecting
                        || progress.is_some();
                    let is_expanded = expanded_models().contains(&name);
                    let name_for_action = name.clone();
                    let name_for_delete = name.clone();
                    let name_for_details = name.clone();
                    let state_action = model_state;
                    let state_delete = model_state;
                    let mut state_details = expanded_models;
                    let action_disabled = is_selecting || is_default || progress.is_some()
                        || is_deleting;
                    rsx! {
                        div {
                            key: "{name}",
                            class: "flex flex-col gap-2 rounded-md border border-border p-3 shrink-0",
                            div { class: "flex items-center justify-between gap-3",
                                div { class: "flex flex-col min-w-0",
                                    span { class: "text-sm font-medium truncate", "{model.name}" }
                                    div { class: "flex items-center gap-2",
                                        span { class: "text-xs text-muted-foreground",
                                            "{model.params} \u{2022} {model.size as u64} MB"
                                        }
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            size: ButtonSize::Sm,
                                            class: "h-6 gap-1 px-1 text-xs text-muted-foreground hover:text-foreground",
                                            aria_label: if is_expanded { "Hide model details" } else { "Show model details" },
                                            title: if is_expanded { "Hide model details" } else { "Show model details" },
                                            onclick: move |_| {
                                                let mut expanded = state_details.write();
                                                if !expanded.remove(&name_for_details) {
                                                    expanded.insert(name_for_details.clone());
                                                }
                                            },
                                            if is_expanded {
                                                ChevronUp { class: "size-3" }
                                            }
                                            if !is_expanded {
                                                ChevronDown { class: "size-3" }
                                            }
                                            "Details"
                                        }
                                    }
                                }
                                div { class: "flex items-center gap-2 shrink-0",
                                    Button {
                                        variant: ButtonVariant::Destructive,
                                        size: ButtonSize::IconSm,
                                        class: if !is_downloaded { "hidden" },
                                        aria_label: "Delete model",
                                        title: "Delete model",
                                        disabled: delete_disabled,
                                        onclick: move |_| {
                                            state_delete.delete_model(name_for_delete.clone(), on_model_deleted);
                                        },
                                        if is_deleting {
                                            LoaderCircle { class: "animate-spin" }
                                        } else {
                                            Trash2 {}
                                        }
                                    }
                                    Button {
                                        variant: if is_default || is_selecting || progress.is_some() { ButtonVariant::Secondary } else if is_downloaded { ButtonVariant::Outline } else { ButtonVariant::Default },
                                        size: ButtonSize::Sm,
                                        disabled: action_disabled,
                                        onclick: move |_| {
                                            if is_downloaded {
                                                state_action.select_model(name_for_action.clone(), Some(on_model_selected));
                                            } else {
                                                state_action.download_model(name_for_action.clone());
                                            }
                                        },
                                        if is_selecting || progress.is_some() {
                                            LoaderCircle { class: "animate-spin" }
                                        }
                                        if !is_selecting && progress.is_some() {
                                            "{progress.unwrap_or_default() as u64}%"
                                        }
                                        if !is_selecting && progress.is_none() && is_default {
                                            CheckCheck {}
                                        }
                                        if !is_selecting && progress.is_none() && !is_default && is_downloaded {
                                            Check {}
                                        }
                                        if !is_selecting && progress.is_none() && !is_default && !is_downloaded {
                                            Download {}
                                        }
                                    }
                                }
                            }
                            if is_expanded {
                                div { class: "flex flex-col gap-2 rounded-md bg-muted/50 px-3 py-2.5",
                                    div { class: "flex min-w-0 flex-col gap-0.5",
                                        span { class: "text-[10px] font-medium uppercase tracking-wide text-muted-foreground",
                                            "Repository"
                                        }
                                        span { class: "break-all font-mono text-xs text-foreground/80", "{model.repo}" }
                                    }
                                    div { class: "flex flex-wrap items-center gap-1.5",
                                        span { class: "rounded-full border border-border/70 bg-background/60 px-2 py-0.5 text-xs text-muted-foreground",
                                            "Size: {model.size} MB"
                                        }
                                        if model.is_thinking {
                                            span { class: "rounded-full border border-border/70 bg-background/60 px-2 py-0.5 text-xs text-muted-foreground",
                                                "Thinking"
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
}
