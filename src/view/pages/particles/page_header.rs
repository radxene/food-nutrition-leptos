use leptos::*;

#[component]
pub fn PageHeader(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <header class="p-4 sm:p-6">
            {children(cx)}
        </header>
    }
}
