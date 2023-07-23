use leptos::*;

#[component]
pub fn CardBox(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <button class="p-2 border border-transparent hover:border-brand-500 rounded-md shadow-md bg-white dark:bg-dark-vague-1 transition-colors text-gray-500 hover:text-gray-700 hover:bg-gray-100 dark:hover:text-gray-300 dark:hover:bg-dark-vague-2">
            {children(cx)}
        </button>
    }
}
