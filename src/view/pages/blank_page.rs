use leptos::*;

use super::particles::PageContent;
use super::particles::PageHeader;
use super::particles::PageTitle;

#[component]
pub fn BlankPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <PageHeader>
            <PageTitle>"Blank Page"</PageTitle>
        </PageHeader>
        <PageContent>""</PageContent>
    }
}
