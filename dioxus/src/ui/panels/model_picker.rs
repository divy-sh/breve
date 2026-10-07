use dioxus::prelude::*;
use lucide_dioxus::{Check, Download, LoaderCircle};

use crate::ui::components::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::composables::use_model::use_model;

#[component]
pub fn ModelPicker(on_model_selected: EventHandler<()>) -> Element {
    let model_state = use_model();

    rsx! {
        div { class: "flex flex-col flex-1 min-h-0 w-full max-w-lg mx-auto gap-4 overflow-y-auto",

            div { class: "flex flex-col gap-1 shrink-0",
                h1 { class: "text-lg font-semibold", "Choose a model" }
                p { class: "text-sm text-muted-foreground",
                    "Download a model to start chatting. Smaller models are faster to download and run; larger models are more capable."
                }
            }
            for (name , model) in model_state.available() {
                {
                    let is_downloaded = model_state.is_downloaded(&name);
                    let is_default = model_state.is_default(&name);
                    let progress = model_state.progress(&name);
                    let is_selecting = model_state.is_selecting(&name);

                    let name_for_download = name.clone();
                    let name_for_select = name.clone();
                    let state_select = model_state;
                    let state_download = model_state;

                    rsx! {
                        div {
                            key: "{name}",
                            class: "flex items-center justify-between gap-3 rounded-md border border-border p-3 shrink-0",
                            div { class: "flex flex-col min-w-0",
                                span { class: "text-sm font-medium truncate", "{model.name}" }
                                span { class: "text-xs text-muted-foreground", "{model.params} \u{2022} {model.size as u64} MB" }
                            }
                            div { class: "flex items-center gap-2 shrink-0",
                                if is_selecting {
                                    Button {
                                        variant: ButtonVariant::Secondary,
                                        size: ButtonSize::Sm,
                                        disabled: true,
                                        LoaderCircle { class: "animate-spin" }
                                        "Loading..."
                                    }
                                } else if is_default {
                                    Button {
                                        variant: ButtonVariant::Secondary,
                                        size: ButtonSize::Sm,
                                        disabled: true,
                                        Check {}
                                        "Selected"
                                    }
                                } else if let Some(pct) = progress {
                                    Button {
                                        variant: ButtonVariant::Secondary,
                                        size: ButtonSize::Sm,
                                        disabled: true,
                                        LoaderCircle { class: "animate-spin" }
                                        "{pct as u64}%"
                                    }
                                } else if is_downloaded {
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        size: ButtonSize::Sm,
                                        onclick: move |_| {
                                            state_select.select_model(name_for_select.clone(), Some(on_model_selected));
                                        },
                                        "Select"
                                    }
                                } else {
                                    Button {
                                        variant: ButtonVariant::Default,
                                        size: ButtonSize::Sm,
                                        onclick: move |_| {
                                            state_download.download_model(name_for_download.clone());
                                        },
                                        Download {}
                                        "Download"
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
