use js_sys::Array;
use leptos::html::Button;
use leptos::*;
use wasm_bindgen::JsValue;

use crate::providers::main_layout_provider::{
    is_leaved_sidebar_slice, is_minimized_sidebar_slice, sidebar_width_slice, SidebarWidthEnum,
};
use crate::view::ui::aria::SrOnly;
use crate::view::ui::icons::IconBrand;
use crate::view::ui::icons::IconClose;
use crate::view::ui::icons::IconMenuClose;
use crate::view::ui::icons::IconMenuOpen;

#[component]
pub fn SidebarMenuTop(cx: Scope) -> impl IntoView {
    let button_element: NodeRef<Button> = create_node_ref(cx);

    let (is_minimized, set_is_minimized) = is_minimized_sidebar_slice(cx);
    let (is_leaved, _) = is_leaved_sidebar_slice(cx);
    let (_, set_sidebar_width) = sidebar_width_slice(cx);

    let toggle_sidebar_width = move |_| {
        let sidebar_width = if is_minimized() {
            SidebarWidthEnum::Expanded
        } else {
            SidebarWidthEnum::Minimized
        };
        set_sidebar_width(sidebar_width);
        set_is_minimized(!is_minimized());
    };

    let toggle_button_display = move |button_ref: &HtmlElement<Button>| {
        if is_minimized() && is_leaved() {
            button_ref
                .class_list()
                .add(&Array::of1(&JsValue::from_str("hidden")))
                .unwrap();
        } else {
            button_ref
                .class_list()
                .remove(&Array::of1(&JsValue::from_str("hidden")))
                .unwrap();
        }
    };

    create_effect(cx, move |_| {
        if let Some(button_ref) = button_element.get().as_ref() {
            toggle_button_display(button_ref);
        }
    });

    view! { cx,
        <div class="flex items-center justify-between flex-shrink-0 px-3">
            <a aria-current="page" href="#/" class="router-link-active router-link-exact-active inline-flex items-center gap-2">
                <SrOnly>"Food Nutrition"</SrOnly>
                <span class="p-2 bg-transparent dark:bg-[#ECECEC] rounded-full">
                    <IconBrand size=8 />
                </span>
            </a>
            <button
                type="button"
                class="inline-flex items-center transition-colors font-medium select-none disabled:opacity-50 disabled:cursor-not-allowed focus:outline-none focus:ring focus:ring-offset-2 focus:ring-offset-white dark:focus:ring-offset-dark-vague-1 p-2 bg-white text-gray-500 hover:bg-gray-100 focus:ring-brand-500 dark:text-gray-400 dark:bg-dark-vague-1 dark:hover:bg-dark-vague-1 dark:hover:text-gray-200 rounded-md"
                on:click=toggle_sidebar_width
                node_ref=button_element
            >
                <SrOnly>"Close sidebar"</SrOnly>
                <Show
                    when=move || { !is_minimized() }
                    fallback=move |cx| view! {cx,
                        <span
                            class="hidden lg:block w-6 h-6"
                            style=move || if is_leaved() { "display: none" } else { "" }
                        >
                            <IconMenuOpen />
                        </span>
                    }
                >
                    <span class="hidden lg:block w-6 h-6" /* style="display: none;" */>
                        <IconMenuClose />
                    </span>
                </Show>
                <span class="lg:hidden w-6 h-6">
                    <IconClose />
                </span>
            </button>
        </div>
    }
}
