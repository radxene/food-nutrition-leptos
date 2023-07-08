use leptos::*;

#[component]
pub fn SrOnly(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <span class="sr-only">
            {children(cx)}
        </span>
    }
}
