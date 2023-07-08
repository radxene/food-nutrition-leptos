use leptos::*;

use crate::view::widgets::navigation_menu::NavigationMenu;

#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav aria-label="secondary" class="sticky top-0 z-10 px-6 py-4 bg-white transition-transform duration-500 dark:bg-slate-800">
            <NavigationMenu />
        </nav>
    }
}
