use std::collections::HashMap;

use dioxus::prelude::*;
use lucide_dioxus::{Check, Download, LoaderCircle};

use crate::core::models::{controller as models_ctrl, models::Model};
use crate::ui::components::button::{Button, ButtonSize, ButtonVariant};

/// Lets the user download and select the local model used for inference.
///
/// Calls `on_model_selected` once a model has been downloaded (if needed)
/// and successfully activated as the default model.
#[component]
pub fn ModelPicker(on_model_selected: EventHandler<()>) -> Element {
    let available = use_signal(|| {
        let mut models: Vec<(String, Model)> = models_ctrl::get_available_models()
            .iter()
            .map(|(name, model)| (name.clone(), model.clone()))
            .collect();
        models.sort_by(|a, b| a.1.size.partial_cmp(&b.1.size).unwrap());
        models
    });

    let mut downloaded = use_signal(models_ctrl::list_downloaded_models);
    let mut default_model = use_signal(models_ctrl::get_default_model);
    let mut downloading: Signal<HashMap<String, f64>> = use_signal(HashMap::new);
    let mut selecting: Signal<Option<String>> = use_signal(|| None);

    rsx! {
        div { class: "flex flex-col h-screen w-full max-w-lg mx-auto p-6 gap-4 overflow-y-auto",
            div { class: "flex flex-col gap-1",
                h1 { class: "text-lg font-semibold", "Choose a model" }
                p { class: "text-sm text-muted-foreground",
                    "Download a model to start chatting. Smaller models are faster to download and run; larger models are more capable."
                }
            }

            for (name, model) in available() {
                {
                    let is_downloaded = downloaded().contains(&name);
                    let is_default = default_model() == name;
                    let progress = downloading().get(&name).copied();
                    let is_selecting = selecting().as_deref() == Some(name.as_str());

                    let name_for_download = name.clone();
                    let name_for_select = name.clone();

                    rsx! {
                        div {
                            key: "{name}",
                            class: "flex items-center justify-between gap-3 rounded-md border border-border p-3",
                            div { class: "flex flex-col min-w-0",
                                span { class: "text-sm font-medium truncate", "{model.name}" }
                                span { class: "text-xs text-muted-foreground",
                                    "{model.params} \u{2022} {model.size as u64} MB"
                                }
                            }
                            div { class: "flex items-center gap-2 shrink-0",
                                if is_selecting {
                                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, disabled: true,
                                        LoaderCircle { class: "animate-spin" }
                                        "Loading..."
                                    }
                                } else if is_default {
                                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, disabled: true,
                                        Check {}
                                        "Selected"
                                    }
                                } else if let Some(pct) = progress {
                                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, disabled: true,
                                        LoaderCircle { class: "animate-spin" }
                                        "{pct as u64}%"
                                    }
                                } else if is_downloaded {
                                    Button {
                                        variant: ButtonVariant::Outline,
                                        size: ButtonSize::Sm,
                                        onclick: move |_| {
                                            let name = name_for_select.clone();
                                            spawn(async move {
                                                selecting.set(Some(name.clone()));
                                                let result = {
                                                    let name = name.clone();
                                                    tokio::task::spawn_blocking(move || {
                                                        models_ctrl::set_default_model(name)
                                                    })
                                                        .await
                                                };
                                                selecting.set(None);
                                                if matches!(result, Ok(Ok(()))) {
                                                    default_model.set(models_ctrl::get_default_model());
                                                    on_model_selected.call(());
                                                }
                                            });
                                        },
                                        "Select"
                                    }
                                } else {
                                    Button {
                                        variant: ButtonVariant::Default,
                                        size: ButtonSize::Sm,
                                        onclick: move |_| {
                                            let name = name_for_download.clone();
                                            spawn(async move {
                                                downloading.write().insert(name.clone(), 0.0);
                                                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<f64>();
                                                let handle = {
                                                    let name = name.clone();
                                                    tokio::task::spawn_blocking(move || {
                                                        models_ctrl::download_model(
                                                            name,
                                                            move |pct: f64| {
                                                                let _ = tx.send(pct);
                                                            },
                                                        )
                                                    })
                                                };
                                                while let Some(pct) = rx.recv().await {
                                                    downloading.write().insert(name.clone(), pct);
                                                }
                                                let result = handle.await;
                                                downloading.write().remove(&name);
                                                if matches!(result, Ok(Ok(()))) {
                                                    downloaded.set(models_ctrl::list_downloaded_models());
                                                }
                                            });
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
