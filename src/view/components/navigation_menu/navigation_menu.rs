use leptos::*;

use crate::providers::main_layout_provider::{
    is_collapsed_sidebar_slice, is_minimized_sidebar_slice, sidebar_width_slice, SidebarWidthEnum,
};
use crate::utils::theme::{ThemeMode, ThemeUtil};
use crate::view::ui::aria::SrOnly;
use crate::view::ui::buttons::ButtonMode;
use crate::view::ui::buttons::ButtonProfile;
use crate::view::ui::icons::IconMaximize;
use crate::view::ui::icons::IconMenuOpen;
use crate::view::ui::icons::IconMinimize;
use crate::view::ui::icons::IconMoon;
use crate::view::ui::icons::IconSun;

#[component]
pub fn NavigationMenu(cx: Scope) -> impl IntoView {
    let (dark, set_dark) = create_signal(cx, ThemeUtil::is_dark_mode());
    let (fullscreen, set_fullscreen) = create_signal(cx, false);
    let (_, set_sidebar_width) = sidebar_width_slice(cx);
    let (_, set_is_minimized) = is_minimized_sidebar_slice(cx);
    let (_, set_is_collapsed) = is_collapsed_sidebar_slice(cx);

    let change_dark_mode = move |_ev| {
        if dark.get() {
            ThemeUtil::set_preferred_color_schema(ThemeMode::Light);
        } else {
            ThemeUtil::set_preferred_color_schema(ThemeMode::Dark);
        }
        set_dark.set(!dark.get());
    };

    let change_maximize_mode = move |_ev| {
        let document = document();

        if document.fullscreen_element().is_none() {
            document
                .document_element()
                .unwrap()
                .request_fullscreen()
                .unwrap();
        } else {
            document.exit_fullscreen();
        }
        set_fullscreen(!fullscreen());
    };

    let render_dark_mode_icon = move || {
        view! { cx,
            <>
                <SrOnly>"Toggle dark mode"</SrOnly>
                <Show when=move || { dark.get() } fallback=|cx| view! { cx, <IconMoon /> }>
                    <IconSun />
                </Show>
            </>
        }
    };

    view! { cx,
        <Menu>
            <MenuGroup>
                // <ButtonMode on:click=change_dark_mode md_hidden=true>
                //     {render_dark_mode_icon}
                // </ButtonMode>
                // <span />
                <button
                    class="inline-flex items-center transition-colors font-medium select-none disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:ring focus:ring-offset-2 focus:ring-offset-white dark:focus:ring-offset-dark-vague-1 p-2 bg-white text-gray-500 hover:bg-gray-100 focus:ring-brand-500 dark:text-gray-400 dark:bg-dark-vague-1 dark:hover:bg-dark-vague-1 dark:hover:text-gray-200 rounded-md"
                    type="button"
                    on:click=move |_| {
                        set_is_collapsed(false);
                        set_is_minimized(false);
                        set_sidebar_width(SidebarWidthEnum::Expanded);
                    }
                >
                    <span class="lg:hidden w-6 h-6">
                        <IconMenuOpen />
                    </span>
                </button>
            </MenuGroup>

            <MenuGroup>
                <ButtonMode on:click=change_dark_mode /* md_only=true */>
                    {render_dark_mode_icon}
                </ButtonMode>

                <ButtonMode on:click=change_maximize_mode /* md_only=true */>
                    <SrOnly>"Toggle screen mode"</SrOnly>
                    <Show when=move || { fullscreen() } fallback=|cx| view! { cx, <IconMinimize /> }>
                        <IconMaximize />
                    </Show>
                </ButtonMode>

                <ButtonProfile src="/assets/img/avatar.png" />
            </MenuGroup>
        </Menu>
    }
}

#[component]
fn Menu(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="flex items-center justify-between">
            {children(cx)}
        </div>
    }
}

#[component]
pub fn MenuGroup(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="flex items-center gap-2">
            {children(cx)}
        </div>
    }
}
