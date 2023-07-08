mod navigation_menu;
mod navigation_menu_group;

use leptos::*;

use crate::services::theme::{ThemeMode, ThemeService};
use crate::view::ui::aria::sr_only::SrOnly;
use crate::view::ui::buttons::button_avatar::ButtonAvatar;
use crate::view::ui::buttons::button_mode::ButtonMode;
use crate::view::ui::icons::icon_maximize::IconMaximize;
use crate::view::ui::icons::icon_minimize::IconMinimize;
use crate::view::ui::icons::icon_moon::IconMoon;
use crate::view::ui::icons::icon_sun::IconSun;

use navigation_menu::NavigationMenu as Menu;
use navigation_menu_group::NavigationMenuGroup as MenuGroup;

#[component]
pub fn NavigationMenu(cx: Scope) -> impl IntoView {
    let (dark, set_dark) = create_signal(cx, ThemeService::is_dark_mode());
    let (maximize, set_maximize) = create_signal(cx, false);

    let change_dark_mode = move |_ev| {
        if dark() {
            ThemeService::set_preferred_color_schema(ThemeMode::Light);
        } else {
            ThemeService::set_preferred_color_schema(ThemeMode::Dark);
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
                <Show when=move || { dark() } fallback=|cx| view! { cx, <IconMoon /> }>
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
                    <Show when=move || { maximize() } fallback=|cx| view! { cx, <IconMinimize /> }>
                        <IconMaximize />
                    </Show>
                </ButtonMode>

                <ButtonAvatar src="/assets/img/avatar.png" />
            </MenuGroup>
        </Menu>
    }
}
