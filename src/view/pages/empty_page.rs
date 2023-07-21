use leptos::*;

use super::particles::PageContent;

#[component]
pub fn EmptyPage(cx: Scope) -> impl IntoView {
    view! { cx,
        <PageContent>""</PageContent>
    }
}
