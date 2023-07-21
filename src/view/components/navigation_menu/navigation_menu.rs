use leptos::*;

use crate::utils::theme::{ThemeMode, ThemeUtil};
use crate::view::ui::aria::SrOnly;
use crate::view::ui::buttons::ButtonAvatar;
use crate::view::ui::buttons::ButtonMode;
use crate::view::ui::icons::IconMaximize;
use crate::view::ui::icons::IconMinimize;
use crate::view::ui::icons::IconMoon;
use crate::view::ui::icons::IconSun;

#[component]
pub fn NavigationMenu(cx: Scope) -> impl IntoView {
    let (dark, set_dark) = create_signal(cx, ThemeUtil::is_dark_mode());
    let (maximize, set_maximize) = create_signal(cx, false);

    let change_dark_mode = move |_ev| {
        if dark.get() {
            ThemeUtil::set_preferred_color_schema(ThemeMode::Light);
        } else {
            ThemeUtil::set_preferred_color_schema(ThemeMode::Dark);
        }
        set_dark.set(!dark.get());
        // log!("Change dark mode");
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
        set_maximize.set(!maximize.get());
        // log!("Change maximize mode");
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
                <ButtonMode on:click=change_dark_mode md_hidden=true>
                    {render_dark_mode_icon}
                </ButtonMode>
            </MenuGroup>

            <MenuGroup>
                <ButtonMode on:click=change_dark_mode md_only=true>
                    {render_dark_mode_icon}
                </ButtonMode>

                <ButtonMode on:click=change_maximize_mode md_only=true>
                    <SrOnly>"Toggle screen mode"</SrOnly>
                    <Show when=move || { maximize.get() } fallback=|cx| view! { cx, <IconMinimize /> }>
                        <IconMaximize />
                    </Show>
                </ButtonMode>

                <ButtonAvatar src="/assets/img/avatar.png" />
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
