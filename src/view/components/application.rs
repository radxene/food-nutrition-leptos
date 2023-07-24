use leptos::*;
use leptos_router::*;

use crate::routes::PageRoute;
use crate::view::layouts::main_layout::MainLayout;
use crate::view::pages::{BlankPage, EmptyPage, FoodDataPage};

#[component]
pub fn Application(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <MainLayout>
                <Routes>
                    <Route path=PageRoute::Home.path() view=FoodDataPage />
                    <Route path=PageRoute::BlankPage.path() view=move |cx| view! { cx,  <BlankPage/> } />
                    <Route path=PageRoute::EmptyPage.path() view=move |cx| view! { cx,  <EmptyPage/> } />
                </Routes>
            </MainLayout>
        </Router>
    }
}
