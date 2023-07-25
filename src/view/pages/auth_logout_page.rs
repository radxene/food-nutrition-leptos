use leptos::*;
use leptos_router::*;

use crate::routes::PageRoute;

#[component]
pub fn AuthLogoutPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Redirect path=PageRoute::AuthLoginPage.path() />
    }
}
