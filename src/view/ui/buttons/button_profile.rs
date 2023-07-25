use std::time::Duration;

use leptos::{html::Div, *};

use crate::routes::PageRoute;

#[component]
pub fn ButtonProfile(
    cx: Scope,
    src: &'static str,
    #[prop(default = "avatar")] alt: &'static str,
) -> impl IntoView {
    let (is_opened, set_is_opened) = create_signal(cx, false);

    let div_ref = create_node_ref::<Div>(cx);

    let toggle_menu = move |flag: bool| {
        move |_| {
            div_ref.get().unwrap().focus().unwrap();
            set_is_opened.try_set(flag);
        }
    };

    view! { cx,
        <div class="relative">
            <div aria-haspopup="true" aria-expanded="false" id="user-profile-menu">
                <button
                    class="flex text-sm transition border-2 border-transparent rounded-md focus:outline-none focus:ring focus:ring-brand-500 focus:ring-offset-1 focus:ring-offset-white dark:focus:ring-offset-dark-vague-1"
                    on:click=toggle_menu(true)
                >
                    <img class="object-cover w-8 h-8 rounded-md" src=src alt=alt />
                </button>
            </div>

            <div
                class="absolute z-50 mt-2 rounded-md shadow-lg w-48 origin-top-right right-0"
                aria-labelledby="user-profile-menu"
                role="menu"
                tabindex="0"
                on:blur=move |_| set_timeout(move || { set_is_opened.try_set(false); }, Duration::from_millis(50))
                node_ref=div_ref
            >
                {
                    move || {
                        match is_opened() {
                            true => Some(
                                view! { cx,
                                    <div class="rounded-md ring-1 ring-black dark:ring-gray-700 ring-opacity-5 py-1 bg-white dark:bg-dark-vague-1" role="none">
                                        <a
                                            class="router-link-active router-link-exact-active block w-full px-4 py-2 text-sm leading-5 text-left text-gray-700 transition duration-150 ease-in-out focus:outline-none dark:focus:text-white dark:focus:bg-dark-vague-3 dark:text-gray-400"
                                            href=PageRoute::AuthLogoutPage.path()
                                            role="menuitem"
                                            tabindex="-1"
                                            on:click=toggle_menu(false)
                                        >
                                            Log Out
                                        </a>
                                    </div>
                                }
                            ),
                            _ => None
                        }
                    }
                }
            </div>
        </div>
    }
}
