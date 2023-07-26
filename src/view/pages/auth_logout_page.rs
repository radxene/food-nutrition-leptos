use leptos::*;
use leptos_router::*;

use crate::{models::entities::SotrageAuthUser, routes::AppRoute};

#[component]
pub fn AuthLogoutPage(cx: Scope) -> impl IntoView {
    SotrageAuthUser::logout();

    view! { cx,
        <Redirect path=AppRoute::AuthLoginPage.path() />
    }
}
