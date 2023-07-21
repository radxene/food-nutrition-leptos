use leptos::*;

#[component]
pub fn SidebarMenuNav(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <nav aria-label="main" class="relative flex flex-col flex-1 max-h-full gap-4 px-3 ps">
            {children(cx)}
        </nav>
    }
}
