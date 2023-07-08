use leptos::*;

use crate::view::layouts::navigation::Navigation;

#[component]
pub fn Application(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="h-full dark:bg-slate-900">
            <Navigation />
        </div>
    }
}
