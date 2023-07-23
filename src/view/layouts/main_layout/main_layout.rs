use leptos::*;

use super::particles::Content;
use super::particles::Footer;
use super::particles::Navigation;
use super::particles::Sidebar;

#[component]
pub fn MainLayout(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="min-h-full bg-gray-100 dark:bg-dark-bg">
            <Sidebar />
            <Content>
                <Navigation />
                {children(cx)}
                <Footer />
            </Content>
        </div>
    }
}
