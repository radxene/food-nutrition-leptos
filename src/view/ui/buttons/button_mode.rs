use leptos::*;

#[component]
pub fn ButtonMode(
    cx: Scope,
    #[prop(default = false)] md_hidden: bool,
    #[prop(default = false)] md_only: bool,
    children: Children,
) -> impl IntoView {
    view! { cx,
        <button
            class="inline-flex items-center transition-colors font-medium select-none disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:ring focus:ring-offset-2 focus:ring-offset-white dark:focus:ring-offset-slate-800 p-2 bg-white text-gray-500 hover:bg-gray-100 focus:ring-purple-500 dark:text-gray-400 dark:bg-slate-800 dark:hover:bg-slate-800 dark:hover:text-gray-200 rounded-md"
            // md_hidden
            class=("md:hidden", move || md_hidden)
            // md_only
            class=("hidden", move || md_only)
            class=("md:inline-flex", move || md_only)
            type="button">
            {children(cx)}
        </button>
    }
}
