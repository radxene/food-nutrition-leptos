use leptos::*;
use leptos_router::*;

use crate::utils::validators::AuthValidatorUtil;
use crate::view::layouts::main_layout::MainLayout;
use crate::view::pages::{AuthLoginPage, AuthLogoutPage, BlankPage, EmptyPage, FoodDataPage};

#[derive(Debug, Clone, Copy, Default)]
pub enum AppRoute {
    #[default]
    Root,
    BlankPage,
    EmptyPage,
    AuthLoginPage,
    AuthLogoutPage,
}

impl AppRoute {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Root => "/",
            Self::BlankPage => "/page/blank",
            Self::EmptyPage => "/page/empty",
            Self::AuthLoginPage => "/auth/login",
            Self::AuthLogoutPage => "/auth/logout",
        }
    }
}

#[component(transparent)]
pub fn MainRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <ProtectedRoute
            path=AppRoute::Root.path()
            redirect_path=AppRoute::AuthLoginPage.path()
            condition=move |_| AuthValidatorUtil::validate_storage_user_locally()
            view=move |cx| {
                view! { cx, <MainLayout><Outlet/></MainLayout> }
            }
        >
            <Route path=AppRoute::Root.path() view=FoodDataPage />
            <Route path=AppRoute::BlankPage.path() view=move |cx| view! { cx, <BlankPage /> } />
            <Route path=AppRoute::EmptyPage.path() view=move |cx| view! { cx, <EmptyPage /> } />
        </ProtectedRoute>
    }
}

#[component(transparent)]
pub fn AuthRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route path=AppRoute::Root.path() view=move |cx| { view! { cx, <Outlet/> } }>
            <Route path=AppRoute::AuthLogoutPage.path() view=AuthLogoutPage />
            <Route path=AppRoute::AuthLoginPage.path() view=AuthLoginPage />
        </Route>
    }
}
