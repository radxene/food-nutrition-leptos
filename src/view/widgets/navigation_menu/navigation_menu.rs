use leptos::*;

#[component]
pub fn NavigationMenu(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="flex items-center justify-between">
            {children(cx)}
        </div>
    }
}
