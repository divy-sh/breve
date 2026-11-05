use dioxus::document::eval;
use dioxus::prelude::*;
use lucide_dioxus::{Moon, Sun};

use crate::ui::components::button::{Button, ButtonVariant};
use crate::ui::components::dialog::{use_dialog_trigger_id, DialogClose};
use crate::ui::components::separator::Separator;
use crate::ui::components::slider::Slider;
use crate::ui::composables::use_settings::{
    get_model_config, set_config, set_model_config, ModelSettings,
};

#[component]
pub fn Settings(theme: Signal<bool>) -> Element {
    let mut settings = use_signal(|| None::<ModelSettings>);
    let mut is_loading = use_signal(|| true);
    let mut is_saving = use_signal(|| false);
    let mut error = use_signal(|| None::<String>);
    let dialog_id = use_dialog_trigger_id();

    use_effect(move || {
        spawn(async move {
            match get_model_config().await {
                Ok(config) => settings.set(Some(config)),
                Err(message) => error.set(Some(format!("Could not load settings: {message}"))),
            }
            is_loading.set(false);
        });
    });

    let save_settings = move |_| {
        let Some(settings_to_save) = settings() else {
            return;
        };
        is_saving.set(true);
        error.set(None);
        let dialog_id = dialog_id.clone();
        spawn(async move {
            match set_model_config(settings_to_save).await {
                Ok(()) => {
                    if let Some(id) = dialog_id.as_deref() {
                        let script = format!(
                            "document.querySelector('[data-dialog-close=\"{id}\"]').click();"
                        );
                        eval(&script);
                    }
                }
                Err(message) => error.set(Some(format!("Could not save settings: {message}"))),
            }
            is_saving.set(false);
        });
    };

    rsx! {
        main { class: "flex flex-col gap-5 overflow-y-auto",
            section { class: "flex flex-col gap-3",
                h4 { class: "text-sm font-semibold", "Appearance" }
                Button {
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        let next_theme = !theme();
                        theme.set(next_theme);
                        error.set(None);
                        spawn(async move {
                            if let Err(message) = set_config("darkMode".into(), next_theme.to_string())
                                .await
                            {
                                error.set(Some(format!("Could not save theme preference: {message}")));
                            }
                        });
                    },
                    if theme() {
                        Sun { class: "w-4 h-4" }
                        "Light"
                    } else {
                        Moon { class: "w-4 h-4" }
                        "Dark"
                    }
                }
            }

            Separator {}

            if is_loading() {
                p { class: "text-sm text-muted-foreground", "Loading settings..." }
            } else if let Some(config) = settings() {
                section { class: "flex flex-col gap-3",
                    h4 { class: "text-sm font-semibold", "Model Parameters" }
                    label { class: "flex flex-col gap-2 text-sm",
                        span { class: "flex justify-between",
                            span { "Temperature" }
                            span { class: "text-muted-foreground", "{config.temperature:.2}" }
                        }
                        Slider {
                            class: "w-full",
                            min: 0.0,
                            max: 1.0,
                            step: 0.01,
                            value: f64::from(config.temperature),
                            oninput: move |event: FormEvent| {
                                if let Ok(value) = event.value().parse::<f32>() {
                                    if let Some(config) = settings.write().as_mut() {
                                        config.temperature = value;
                                    }
                                }
                            },
                        }
                        span { class: "flex justify-between text-xs text-muted-foreground",
                            span { "0" }
                            span { "1.0" }
                        }
                    }
                    label { class: "flex flex-col gap-2 text-sm",
                        span { class: "flex justify-between",
                            span { "Max Output Tokens" }
                            span { class: "text-muted-foreground", "{config.max_output_length}" }
                        }
                        Slider {
                            class: "w-full",
                            min: config.min_output_length as f64,
                            max: config.output_length_limit as f64,
                            step: 1.0,
                            value: config.max_output_length as f64,
                            oninput: move |event: FormEvent| {
                                if let Ok(value) = event.value().parse::<u64>() {
                                    if let Some(config) = settings.write().as_mut() {
                                        config.max_output_length = value;
                                    }
                                }
                            },
                        }
                        span { class: "flex justify-between text-xs text-muted-foreground",
                            span { "{config.min_output_length}" }
                            span { "{config.output_length_limit}" }
                        }
                    }
                    label { class: "flex flex-col gap-2 text-sm",
                        span { class: "flex justify-between",
                            span { "Context Window" }
                            span { class: "text-muted-foreground", "{config.max_context_length}" }
                        }
                        Slider {
                            class: "w-full",
                            min: config.min_context_length as f64,
                            max: config.context_length_limit as f64,
                            step: 1.0,
                            value: config.max_context_length as f64,
                            oninput: move |event: FormEvent| {
                                if let Ok(value) = event.value().parse::<u64>() {
                                    if let Some(config) = settings.write().as_mut() {
                                        config.max_context_length = value;
                                    }
                                }
                            },
                        }
                        span { class: "flex justify-between text-xs text-muted-foreground",
                            span { "{config.min_context_length}" }
                            span { "{config.context_length_limit}" }
                        }
                    }
                }

                Separator {}

                label { class: "flex flex-col gap-2 text-sm font-semibold",
                    "System Prompt"
                    textarea {
                        id: "system-prompt",
                        class: "min-h-32 w-full resize-y rounded-md border border-input bg-transparent px-3 py-2 font-normal outline-none placeholder:text-muted-foreground focus-visible:border-ring focus-visible:ring-2 focus-visible:ring-ring/50",
                        placeholder: "e.g. You are a helpful assistant...",
                        value: "{config.system_prompt}",
                        oninput: move |event| {
                            if let Some(config) = settings.write().as_mut() {
                                config.system_prompt = event.value();
                            }
                        },
                    }
                }
            }
            if let Some(message) = error() {
                p { class: "text-sm text-destructive", "{message}" }
            }
            footer { class: "flex justify-end gap-2",
                DialogClose { class: "w-fit", "Cancel" }
                Button {
                    disabled: is_loading() || is_saving() || settings().is_none(),
                    onclick: save_settings,
                    "Save"
                }
            }
        }
    }
}
