use std::collections::HashMap;

use leptos::*;

use crate::utils::string::StringUtil;
use crate::view::ui::cards::CardBox;
use crate::view::ui::separators::LineHorizontal;

use super::particles::PageContent;
use super::particles::PageHeader;
use super::particles::PageTitle;

#[component]
pub fn FoodDataPage(cx: Scope) -> impl IntoView {
    let categories = vec![
        HashMap::from([("label", "beef"), ("src", "/assets/img/f_beef.png")]),
        HashMap::from([("label", "chicken"), ("src", "/assets/img/f_chicken.png")]),
        HashMap::from([("label", "pork"), ("src", "/assets/img/f_pork.png")]),
        HashMap::from([("label", "apple"), ("src", "/assets/img/f_apple.png")]),
        HashMap::from([("label", "banana"), ("src", "/assets/img/f_banana.png")]),
    ];

    view! { cx,
        <PageHeader>
            <PageTitle>"Food Data Page"</PageTitle>
        </PageHeader>
        <PageContent>
            <div class="grid gap-6">
                <section class="grid grid-cols-2 gap-6 md:grid-cols-3 lg:grid-cols-12">
                    <h3 class="sr-only">"Food categories"</h3>
                    <For
                        each=move || categories.clone().into_iter()
                        key=|item| item["label"]
                        view=|cx, item| {
                            let label = item["label"];
                            view!{ cx,
                                <CardBox>
                                    <img class="dark:invert opacity-75" src=item["src"] alt=label />
                                    <p class="text-xs font-semibold leading-tight text-brand-500 dark:text-gray-200">
                                        {StringUtil::capitalize(label)}
                                    </p>
                                </CardBox>
                            }
                        }
                    />
                </section>
                <LineHorizontal />
            </div>
        </PageContent>
    }
}
