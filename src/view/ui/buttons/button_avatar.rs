use leptos::*;

#[component]
pub fn ButtonAvatar(
    cx: Scope,
    src: &'static str,
    #[prop(default = "avatar")] alt: &'static str,
) -> impl IntoView {
    view! { cx,
        <div class="relative">
            <div aria-haspopup="true" aria-expanded="false">
                <button class="flex text-sm transition border-2 border-transparent rounded-md focus:outline-none focus:ring focus:ring-purple-500 focus:ring-offset-1 focus:ring-offset-white dark:focus:ring-offset-slate-800">
                    <img class="object-cover w-8 h-8 rounded-md" alt="User Name" src={src} alt={alt} />
                </button>
            </div>
        </div>
    }
}
