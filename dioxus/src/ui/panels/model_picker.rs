use dioxus::prelude::*;
use lucide_dioxus::{Check, Download, LoaderCircle, Trash2};

use crate::ui::components::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::composables::use_model::use_model;

#[component]
pub fn ModelPicker(
    on_model_selected: EventHandler<()>,
    on_model_deleted: EventHandler<()>,
) -> Element {
    let model_state = use_model();

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

                        let name_for_action = name.clone();
                        let name_for_delete = name.clone();
                        let state_action = model_state;
                        let state_delete = model_state;
                        let action_disabled =
                            is_selecting || is_default || progress.is_some() || is_deleting;

                        rsx! {
                        div {
                            key: "{name}",
                            class: "flex items-center justify-between gap-3 rounded-md border border-border p-3 shrink-0",
                            div { class: "flex flex-col min-w-0",
                                span { class: "text-sm font-medium truncate", "{model.name}" }
                                span { class: "text-xs text-muted-foreground", "{model.params} \u{2022} {model.size as u64} MB" }
                            }
                            div { class: "flex items-center gap-2 shrink-0",
                                Button {
                                    variant: if is_default || is_selecting || progress.is_some() {
                                        ButtonVariant::Secondary
                                    } else if is_downloaded {
                                        ButtonVariant::Outline
                                    } else {
                                        ButtonVariant::Default
                                    },
                                    size: ButtonSize::Sm,
                                    disabled: action_disabled,
                                    onclick: move |_| {
                                        if is_downloaded {
                                            state_action.select_model(
                                                name_for_action.clone(),
                                                Some(on_model_selected),
                                            );
                                        } else {
                                            state_action.download_model(name_for_action.clone());
                                        }
                                    },
                                    if is_selecting || progress.is_some() {
                                        LoaderCircle { class: "animate-spin" }
                                        if is_selecting {
                                            "Loading..."
                                        } else {
                                            "{progress.unwrap_or_default() as u64}%"
                                        }
                                    } else if is_default {
                                        Check {}
                                        "Selected"
                                    } else if is_downloaded {
                                        "Select"
                                    } else {
                                        Download {}
                                        "Download"
                                    }
                                }
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    size: ButtonSize::IconSm,
                                    class: if is_downloaded {
                                        "text-muted-foreground hover:text-destructive"
                                    } else {
                                        "hidden"
                                    },
                                    aria_label: "Delete model",
                                    title: "Delete model",
                                    disabled: !is_downloaded || is_deleting || is_selecting || progress.is_some(),
                                    onclick: move |_| {
                                        state_delete.delete_model(
                                            name_for_delete.clone(),
                                            on_model_deleted,
                                        );
                                    },
                                    if is_deleting {
                                        LoaderCircle { class: "animate-spin" }
                                    } else {
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
