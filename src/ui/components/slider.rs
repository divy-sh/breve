use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Default, Clone, Copy, PartialEq, Eq, strum::IntoStaticStr)]
pub enum SliderVariant {
    #[default]
    Round,
    Flat,
}

impl SliderVariant {
    fn as_str(self) -> &'static str {
        self.into()
    }
}

#[component]
pub fn Slider(
    #[props(into, optional)] class: Option<String>,
    #[props(default)] variant: SliderVariant,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default = 50.0)] value: f64,
    #[props(optional)] disabled: bool,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let merged = tw_merge!(
        "block w-full min-w-0 cursor-pointer disabled:cursor-not-allowed disabled:opacity-50",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        input {
            "data-name": "Slider",
            "data-variant": "{variant.as_str()}",
            r#type: "range",
            class: "{merged}",
            style: "accent-color: var(--primary);",
            min: "{min}",
            max: "{max}",
            step: "{step}",
            value: "{value}",
            disabled,
            oninput: move |e| {
                if let Some(handler) = &oninput {
                    handler.call(e);
                }
            },
        }
    }
}
