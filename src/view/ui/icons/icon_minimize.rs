use leptos::*;

#[component]
pub fn IconMinimize(cx: Scope) -> impl IntoView {
    view! { cx,
        <svg viewBox="0 0 24 24" stroke="currentColor" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true" class="w-6 h-6">
            <path d="M4 4L9 9M9 9V5M9 9H5M20 4L15 9M15 9V5M15 9H19M4 20L9 15M9 15H5M9 15V19M20 20L15 15M15 15H19M15 15V19" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
    }
}
