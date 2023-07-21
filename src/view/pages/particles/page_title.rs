use leptos::*;

#[component]
pub fn PageTitle(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <h2 class="text-xl font-semibold leading-tight">
            {children(cx)}
        </h2>
    }
}
