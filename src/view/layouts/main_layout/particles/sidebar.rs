use leptos::*;

use crate::providers::main_layout_provider::{
    is_leaved_sidebar_slice, sidebar_width_slice, SidebarWidthEnum,
};
use crate::view::components::sidebar_menu::SidebarMenu;

#[component]
pub fn Sidebar(cx: Scope) -> impl IntoView {
    let (sidebar_width, _) = sidebar_width_slice(cx);
    let (is_leaved, set_is_leaved) = is_leaved_sidebar_slice(cx);

    let sidebar_classes = move || -> String {
        let mut classes = vec!["fixed inset-y-0 z-20 flex flex-col space-y-6 py-4 bg-white shadow-lg dark:bg-dark-vague-1 translate-x-0 transition-width-transform overflow-hidden".to_owned()];

        let width_class = if is_leaved() {
            format!("{}", sidebar_width())
        } else {
            SidebarWidthEnum::Expanded.to_string()
        };

        classes.push(width_class);
        classes.join(" ")
    };

    view! { cx,
        <aside
            on:mouseenter=move |_| set_is_leaved(false)
            on:mouseleave=move |_| set_is_leaved(true)
            class=move || sidebar_classes()
        >
            <SidebarMenu />
        </aside>
    }
}
