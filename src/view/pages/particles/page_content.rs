use leptos::*;

#[component]
pub fn PageContent(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <main class="flex-1 mt-6">
            {children(cx)}
        </main>
    }
}
