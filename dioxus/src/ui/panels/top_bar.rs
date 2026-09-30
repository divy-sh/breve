use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use tw_merge::tw_merge;

static TOPBAR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn use_topbar_id() -> String {
    use_hook(|| {
        let id = TOPBAR_COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("{id}")
    })
}

#[derive(Clone)]
struct TopBarContext {
    id: String,
}

#[component]
pub fn TopBar(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let id = use_topbar_id();
    provide_context(TopBarContext { id });

    rsx! {
        header {
            "data-name": "TopBar",
            class: tw_merge!(
                "flex items-center justify-between px-4 py-3 border-b border-border bg-background shrink-0 w-full",
                class.as_deref().unwrap_or("")
            ),
            {children}
        }
    }
}

#[component]
pub fn TopBarLeft(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            "data-name": "TopBarLeft",
            class: tw_merge!("flex items-center gap-2", class.as_deref().unwrap_or("")),
            {children}
        }
    }
}

#[component]
pub fn TopBarRight(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        div {
            "data-name": "TopBarRight",
            class: tw_merge!("flex items-center gap-2", class.as_deref().unwrap_or("")),
            {children}
        }
    }
}

#[component]
pub fn TopBarTitle(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    rsx! {
        span {
            "data-name": "TopBarTitle",
            class: tw_merge!("font-semibold text-sm text-foreground", class.as_deref().unwrap_or("")),
            {children}
        }
    }
}
