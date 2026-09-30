use dioxus::document::eval;
use dioxus::prelude::*;

use ui::panels::app::App;

mod core;
mod types;
mod ui;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    core::init();
    dioxus::launch(Main);
}

#[component]
fn Main() -> Element {
    let mut is_dark = use_signal(|| true);

    use_effect(move || {
        let dark_class = if is_dark() { "dark" } else { "" };
        let js = format!("document.documentElement.className = '{}';", dark_class);
        eval(&js);
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        App {
            theme: is_dark,
        }
    }
}
