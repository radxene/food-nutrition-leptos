use leptos::*;
use leptos_icons::{AiIcon, BiIcon, Icon, SiIcon};

use super::particles::SidebarMenuNav;
use super::particles::SidebarMenuNavItem;
use super::particles::SidebarMenuNavItemLink;
use super::particles::SidebarMenuTop;

#[component]
pub fn SidebarMenu(cx: Scope) -> impl IntoView {
    view! { cx,
        <SidebarMenuTop />

        <SidebarMenuNav>
            <SidebarMenuNavItem icon=Icon::from(AiIcon::AiDashboardOutlined)>
                <SidebarMenuNavItemLink slot href="/">"Dashboard"</SidebarMenuNavItemLink>
            </SidebarMenuNavItem>

            <SidebarMenuNavItem icon=Icon::from(BiIcon::BiFileBlankRegular) label="Pages">
                <SidebarMenuNavItemLink slot href="/page/blank">"Blank"</SidebarMenuNavItemLink>
                <SidebarMenuNavItemLink slot href="/page/empty">"Empty"</SidebarMenuNavItemLink>
            </SidebarMenuNavItem>

            <SidebarMenuNavItem icon=Icon::from(SiIcon::SiRust)>
                <SidebarMenuNavItemLink slot href="https://rustwasm.github.io/" target="_blank">"WasmPack"</SidebarMenuNavItemLink>
            </SidebarMenuNavItem>
        </SidebarMenuNav>
    }
}
