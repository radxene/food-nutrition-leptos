use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="flex flex-col flex-shrink-0 w-full gap-2 px-6 py-4 lg:flex-row lg:justify-between">
            <p class="text-sm text-gray-500 dark:text-gray-400">
                " © "
                <span>"2023"</span>
                " Apache License, Version 2.0 "
            </p>
            <p class="flex items-center gap-1 text-sm text-gray-500 dark:text-gray-400">
                <span>"Made with"</span>
                <span>
                    <span class="sr-only">"love"</span>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" class="w-6 h-6 text-brand-500">
                        <path fill-rule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clip-rule="evenodd" />
                    </svg>
                </span>
                <span>"for Rust community"</span>
            </p>
        </footer>
    }
}
//
