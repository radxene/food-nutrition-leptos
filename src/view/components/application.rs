use leptos::*;
use leptos_router::*;

use crate::routes::{AuthRoutes, MainRoutes};

#[component]
pub fn Application(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes>
                <MainRoutes />
                <AuthRoutes />
            </Routes>
        </Router>
    }
}
