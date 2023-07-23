use leptos::*;

#[component]
pub fn LineHorizontal(cx: Scope) -> impl IntoView {
    view! { cx,
        <hr class="h-px bg-gray-200 border-0 dark:bg-gray-700" />
    }
}
