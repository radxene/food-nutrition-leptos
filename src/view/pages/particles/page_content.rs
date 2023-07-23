use leptos::*;

#[component]
pub fn PageContent(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <main class="flex-1 px-4 pb-4 sm:px-6 sm:pb-6">
            {children(cx)}
        </main>
    }
}
