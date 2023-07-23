use leptos::*;
use leptos_router::*;

use crate::view::layouts::main_layout::MainLayout;
use crate::view::pages::{BlankPage, EmptyPage, FoodDataPage};

#[component]
pub fn Application(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <MainLayout>
                <Routes>
                    <Route path="/" view=FoodDataPage />
                    <Route path="/page/blank" view=move |cx| view! { cx,  <BlankPage/> } />
                    <Route path="/page/empty" view=move |cx| view! { cx,  <EmptyPage/> } />
                </Routes>
            </MainLayout>
        </Router>
    }
}
