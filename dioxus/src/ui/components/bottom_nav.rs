use dioxus::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn BottomNav(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "fixed bottom-4 left-1/2 -translate-x-1/2 z-50 w-[calc(100%-2rem)] max-w-lg rounded-full border border-border bg-background/95 backdrop-blur shadow-lg px-2 py-1",
        class.as_deref().unwrap_or("")
    );
    rsx! { nav { "data-name": "BottomNav", class: "{c}", {children} } }
}

#[component]
pub fn BottomNavGrid(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let c = tw_merge!(
        "grid grid-flow-col auto-cols-fr h-14 font-medium items-center",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "BottomNavGrid", class: "{c}", {children} } }
}

#[component]
pub fn BottomNavLabel(
    #[props(into, optional)] class: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!(
        "text-xs text-muted-foreground group-hover:text-primary group-aria-[current=page]:text-primary",
        class.as_deref().unwrap_or("")
    );
    rsx! { span { "data-name": "BottomNavLabel", class: "{c}", {children} } }
}

#[component]
pub fn BottomNavButton(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] onclick: EventHandler<MouseEvent>,
    #[props(into, optional)] aria_current: Option<String>,
    children: Element,
) -> Element {
    let c = tw_merge!(
        "inline-flex flex-col justify-center items-center py-1.5 px-3 rounded-full group [&_svg]:mb-1 [&_svg]:text-muted-foreground hover:[&_svg]:text-primary aria-[current=page]:[&_svg]:text-primary active:scale-[0.95] transition-all",
        "touch-manipulation [-webkit-tap-highlight-color:transparent] select-none",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button {
            "data-name": "BottomNavButton",
            class: "{c}",
            onclick: move |e| onclick.call(e),
            "aria-current": aria_current.as_deref().unwrap_or(""),
            {children}
        }
    }
}
