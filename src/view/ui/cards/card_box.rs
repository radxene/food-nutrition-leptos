use leptos::*;

#[component]
pub fn CardBox<F>(cx: Scope, children: Children, is_active: F) -> impl IntoView
where
    F: Fn() -> bool + 'static + Clone,
{
    view! { cx,
        <button
            class="max-w-[70px] p-2 border rounded-md shadow-md bg-white dark:bg-dark-vague-1 transition-colors text-gray-500 hover:text-gray-700 hover:bg-gray-100 dark:hover:text-gray-300 dark:hover:bg-dark-vague-2"
            class=("border-transparent", { let is_active = is_active.clone(); move || !is_active() })
            class=("hover:border-gray-400", { let is_active = is_active.clone(); move || !is_active() })
            class=("dark:hover:border-white", { let is_active = is_active.clone(); move || !is_active() })
            class=("border-brand-500", { let is_active = is_active.clone(); move || is_active() })
        >
            {children(cx)}
        </button>
    }
}
