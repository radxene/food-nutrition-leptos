use leptos::*;
use leptos_router::*;

use crate::view::layouts::main_layout::MainLayout;
use crate::view::pages::{AuthLoginPage, AuthLogoutPage, BlankPage, EmptyPage, FoodDataPage};

#[derive(Debug, Clone, Copy, Default)]
pub enum PageRoute {
    #[default]
    Root,
    BlankPage,
    EmptyPage,
    AuthLoginPage,
    AuthLogoutPage,
}

impl PageRoute {
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
        <Route path=PageRoute::Root.path() view=move |cx| {
            view! { cx, <MainLayout><Outlet/></MainLayout> }
        }>
            <Route path=PageRoute::Root.path() view=FoodDataPage />
            <Route path=PageRoute::BlankPage.path() view=move |cx| view! { cx,  <BlankPage /> } />
            <Route path=PageRoute::EmptyPage.path() view=move |cx| view! { cx,  <EmptyPage /> } />
        </Route>
    }
}

#[component(transparent)]
pub fn AuthRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Route path=PageRoute::Root.path() view=move |cx| { view! { cx, <Outlet/> } }>
            <Route path=PageRoute::AuthLogoutPage.path() view=AuthLogoutPage />
            <Route path=PageRoute::AuthLoginPage.path() view=AuthLoginPage />
        </Route>
    }
}
