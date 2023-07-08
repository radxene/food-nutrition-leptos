use leptos::*;

#[component]
pub fn NavigationMenuGroup(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="flex items-center gap-2">
            {children(cx)}
        </div>
    }
}
