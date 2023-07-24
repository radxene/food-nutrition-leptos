use std::collections::HashMap;

use leptos::html::Tr;
use leptos::*;

use crate::utils::string::StringUtil;
use crate::view::ui::cards::CardBox;
use crate::view::ui::separators::LineHorizontal;

use super::data::{get_food_data, FoodNutrition, NutritionDetail};
use super::particles::PageContent;
use super::particles::PageHeader;
use super::particles::PageTitle;

#[component]
pub fn FoodDataPage(cx: Scope) -> impl IntoView {
    let categories = vec![
        HashMap::from([("label", "banana"), ("src", "/assets/img/f_banana.png")]),
        HashMap::from([("label", "apple"), ("src", "/assets/img/f_apple.png")]),
        HashMap::from([("label", "beef"), ("src", "/assets/img/f_beef.png")]),
        HashMap::from([("label", "chicken"), ("src", "/assets/img/f_chicken.png")]),
        HashMap::from([("label", "pork"), ("src", "/assets/img/f_pork.png")]),
    ];

    let food_data = get_food_data();

    let (active_label, set_active_label) = create_signal(cx, categories[0]["label"]);

    view! { cx,
        <PageHeader>
            <PageTitle>"Food Data Page"</PageTitle>
        </PageHeader>

        <PageContent>
            <div class="grid gap-6">
                // <section class="grid grid-cols-2 gap-6 md:grid-cols-3 lg:grid-cols-12">
                <section class="flex gap-6">
                    <h3 class="sr-only">"Food categories"</h3>
                    <For
                        each=move || categories.clone().into_iter()
                        key=move|item| item["label"]
                        view=move |cx, item| {
                            let label = item["label"];
                            view!{ cx,
                                <CardBox is_active=move || active_label() == label on:click=move |_| set_active_label(label)>
                                    <img class="dark:invert opacity-75" src=item["src"] alt=label />
                                    <p class="text-xs font-semibold leading-tight text-brand-500 dark:text-gray-200 hidden sm:block">
                                        {StringUtil::capitalize(label)}
                                    </p>
                                </CardBox>
                            }
                        }
                    />
                </section>
                <LineHorizontal />
                <div class="relative overflow-x-auto shadow-md sm:rounded-lg">
                    <table class="w-full text-sm text-left text-gray-500 dark:text-gray-400">
                        <thead class="text-xs text-gray-700 uppercase bg-gray-50 dark:bg-gray-700 dark:text-gray-400">
                            <tr>
                                <th scope="col" class="px-6 py-2 w-1/4">"Name"</th>
                                <th scope="col" class="px-6 py-2 w-1/4">"Amount"</th>
                                <th scope="col" class="px-6 py-2 w-1/4">"Unit"</th>
                                <th scope="col" class="px-6 py-2 w-1/4">"Derivation"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                move || {
                                    let active_nutrition = food_data.as_array().and_then(|value| value
                                        .into_iter()
                                        .find(|value| value.get("kind").unwrap() == active_label()),
                                    ).and_then(|value| value.get("nutrition"));

                                    let string_nutrition = active_nutrition.unwrap().to_string();

                                    let food_nutrition: FoodNutrition = serde_json::from_str(&string_nutrition).unwrap();

                                    view! { cx,
                                        {render_table_row(cx, "Water", food_nutrition.water.as_ref())}
                                        {render_table_row(cx, "Energy", food_nutrition.energy.as_ref())}
                                        {render_table_row(cx, "Calcium", food_nutrition.calcium.as_ref())}
                                        {render_table_row(cx, "Protein", food_nutrition.protein.as_ref())}
                                        {render_table_row(cx, "Lipid", food_nutrition.lipid.as_ref())}
                                        {render_table_row(cx, "Cholesterol", food_nutrition.cholesterol.as_ref())}
                                        {render_table_row(cx, "Sodium", food_nutrition.sodium.as_ref())}
                                        {render_table_row(cx, "Potassium", food_nutrition.potassium.as_ref())}
                                        {render_table_row(cx, "Carbohydrate", food_nutrition.carbohydrate.as_ref())}
                                        {render_table_row(cx, "Fiber", food_nutrition.fiber.as_ref())}
                                        {render_table_row(cx, "Sugars", food_nutrition.sugars.as_ref())}
                                        {render_table_row(cx, "Iron", food_nutrition.iron.as_ref())}
                                        {render_table_row(cx, "Magnesium", food_nutrition.magnesium.as_ref())}
                                        {render_table_row(cx, "Calcium", food_nutrition.calcium.as_ref())}
                                        {render_table_row(cx, "Vitamin C", food_nutrition.vitamin_c.as_ref())}
                                        {render_table_row(cx, "Vitamin D", food_nutrition.vitamin_d.as_ref())}
                                        {render_table_row(cx, "Vitamin B6", food_nutrition.vitamin_b6.as_ref())}
                                        {render_table_row(cx, "Vitamin B12", food_nutrition.vitamin_b12.as_ref())}
                                    }
                                }
                            }
                        </tbody>
                    </table>
                </div>
            </div>
        </PageContent>
    }
}

fn render_table_row(
    cx: Scope,
    label: &'static str,
    nutrition: Option<&NutritionDetail>,
) -> Option<HtmlElement<Tr>> {
    match nutrition {
        Some(nut) => Some({
            let build_label = |label: &'static str| -> String {
                if nut.alias.chars().count() > 0 {
                    format!("{}, ({})", label, nut.alias.clone())
                } else {
                    label.to_owned()
                }
            };
            view! { cx,
                <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600">
                    <th scope="row" class="px-6 py-2 font-medium text-gray-900 whitespace-nowrap dark:text-white">
                        {build_label(label)}
                    </th>
                    <td class="px-6 py-2">{nut.amount}</td>
                    <td class="px-6 py-2">{nut.unit.clone()}</td>
                    <td class="px-6 py-2">{nut.derivation.clone()}</td>
                </tr>
            }
        }),
        _ => None,
    }
}
