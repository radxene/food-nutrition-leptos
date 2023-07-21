use leptos::*;

use super::particles::PageContent;
use super::particles::PageHeader;
use super::particles::PageTitle;

#[component]
pub fn DashboardPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <PageHeader>
            <PageTitle>"Dashboard Page"</PageTitle>
        </PageHeader>
        <PageContent>""</PageContent>
    }
}
