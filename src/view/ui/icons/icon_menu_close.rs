use leptos::*;

#[component]
pub fn IconMenuClose(cx: Scope) -> impl IntoView {
    view! { cx,
        <svg viewBox="0 0 24 24" stroke="currentColor" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" class="w-6 h-6">
            <path d="M12 6H20M12 12H20M4 18H20M8 6L4 9L8 12" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
    }
}
