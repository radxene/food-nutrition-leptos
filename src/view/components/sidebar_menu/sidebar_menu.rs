use leptos::*;
use leptos_icons::{BsIcon, Icon, SiIcon};

use super::particles::SidebarMenuNav;
use super::particles::SidebarMenuNavItem;
use super::particles::SidebarMenuNavItemLink;
use super::particles::SidebarMenuTop;

#[component]
pub fn SidebarMenu(cx: Scope) -> impl IntoView {
    view! { cx,
        <SidebarMenuTop />

        <SidebarMenuNav>
            <SidebarMenuNavItem icon=Icon::from(SiIcon::SiAlwaysdata)>
                <SidebarMenuNavItemLink slot href="/">"Food Data"</SidebarMenuNavItemLink>
            </SidebarMenuNavItem>

            <SidebarMenuNavItem icon=Icon::from(BsIcon::BsClipboard2Data) label="Nested List">
                <SidebarMenuNavItemLink slot href="/page/blank">"Blank Page"</SidebarMenuNavItemLink>
                <SidebarMenuNavItemLink slot href="/page/empty">"Empty Page"</SidebarMenuNavItemLink>
            </SidebarMenuNavItem>

            <SidebarMenuNavItem icon=Icon::from(SiIcon::SiRust)>
                <SidebarMenuNavItemLink slot href="https://leptos.dev/" target="_blank">"Leptos Rust"</SidebarMenuNavItemLink>
            </SidebarMenuNavItem>
        </SidebarMenuNav>
    }
}
