use leptos::*;
use leptos_icons::Icon;
use leptos_router::use_location;

use crate::providers::main_layout_provider::{is_leaved_sidebar_slice, is_minimized_sidebar_slice};

#[slot]
pub struct SidebarMenuNavItemLink {
    href: &'static str,
    #[prop(optional)]
    target: &'static str,
    children: ChildrenFn,
}

#[component]
pub fn SidebarMenuNavItem(
    cx: Scope,
    #[prop(optional)] label: &'static str,
    #[prop(into)] icon: MaybeSignal<Icon>,
    #[prop(default = vec![])] sidebar_menu_nav_item_link: Vec<SidebarMenuNavItemLink>,
) -> impl IntoView {
    let (is_leaved, _) = is_leaved_sidebar_slice(cx);
    let (is_minimized, _) = is_minimized_sidebar_slice(cx);

    let location = use_location(cx);

    let toggle_minimized_content = move |view_fragment: Fragment| {
        if !is_minimized() && is_leaved() || !is_leaved() {
            view_fragment
        } else {
            Fragment::new(vec![])
        }
    };

    match sidebar_menu_nav_item_link.len() {
        1 => {
            let link_slot = sidebar_menu_nav_item_link.first().unwrap();
            view! {cx,
                <ItemLink
                    icon=icon
                    href=link_slot.href.clone()
                    target=link_slot.target.clone()
                    toggle_minimized_content=toggle_minimized_content
                >
                    {(sidebar_menu_nav_item_link.first().unwrap().children)(cx)}
                </ItemLink>
            }
        }
        num if num > 1 => {
            let hrefs = sidebar_menu_nav_item_link
                .iter()
                .map(|it| it.href)
                .collect::<Vec<_>>();

            view! {cx,
                <ItemLinks
                    icon=icon
                    label=label
                    hrefs=hrefs
                    toggle_minimized_content=toggle_minimized_content
                >
                    {
                        sidebar_menu_nav_item_link.iter().map(|slt| {
                            let href = slt.href.clone();
                            (slt.children)(cx).nodes.iter().map(|child| {
                                view! {cx,
                                    <li class="relative leading-8 m-0 pl-6 before:block before:w-4 before:h-0 before:absolute before:left-0 before:top-4 before:border-t-2 before:border-t-gray-200 before:-mt-0.5 last:before:bg-white last:before:h-auto last:before:top-4 last:before:bottom-0 dark:last:before:bg-dark-vague-1 dark:before:border-t-gray-600">
                                        <a
                                            href=href
                                            target=slt.target.clone()
                                            class=move || toggle_link_text_active_class(href == location.pathname.get())
                                        >
                                            <span class="text-base font-medium whitespace-nowrap">{child}</span>
                                        </a>
                                    </li>
                                }
                            }).collect::<Vec<_>>()
                        }).collect::<Vec<_>>()
                    }
                </ItemLinks>
            }
        }
        _ => ().into_view(cx),
    }
}

#[component]
fn ItemLink<F>(
    cx: Scope,
    children: ChildrenFn,
    href: &'static str,
    target: &'static str,
    toggle_minimized_content: F,
    #[prop(into)] icon: MaybeSignal<Icon>,
) -> impl IntoView
where
    F: Fn(Fragment) -> Fragment + Clone + 'static,
{
    let location = use_location(cx);

    view! { cx,
        <a
            href=href
            target=target
            class=move || toggle_link_active_class(href == location.pathname.get())
        >
            <Icon icon=icon class="w-6 h-6 min-w-[1.5rem] min-h-[1.5rem]" />
            {
                move || toggle_minimized_content(
                    Fragment::from(view! {cx,
                        <span class="text-base font-medium whitespace-nowrap">{children(cx)}</span>
                    }.into_view(cx))
                )
            }
        </a>
    }
}

#[component]
fn ItemLinks<F>(
    cx: Scope,
    label: &'static str,
    children: ChildrenFn,
    hrefs: Vec<&'static str>,
    toggle_minimized_content: F,
    #[prop(into)] icon: MaybeSignal<Icon>,
) -> impl IntoView
where
    F: Fn(Fragment) -> Fragment + Clone + 'static,
{
    let (opened, set_opened) = create_signal(cx, false);
    let location = use_location(cx);

    let contains_active = {
        let clone_hrefs = hrefs.clone();
        move || clone_hrefs.contains(&location.pathname.get().as_str())
    };

    view! { cx,
        <div class="relative">
            <button
                class={
                    let contains = contains_active.clone();
                    move || toggle_button_active_class(contains())
                }
                on:click=move |_| set_opened(!opened())
                type="button"
            >
                <Icon icon=icon class="w-6 h-6 min-w-[1.5rem] min-h-[1.5rem]" />
                {
                    let toggle_minimized_content = toggle_minimized_content.clone();
                    move || toggle_minimized_content(
                        view! {cx,
                            <span class="text-base font-medium whitespace-nowrap">{label}</span>
                            <span aria-hidden="true" class="relative block w-6 h-6 ml-auto">
                                {
                                    let contains = contains_active.clone();
                                    move || render_arrow_icon(cx, contains(), opened())
                                }
                            </span>
                        }
                    )
                }
            </button>
            {
                let toggle_minimized_content = toggle_minimized_content.clone();
                move || toggle_minimized_content(
                    Fragment::from(view! {cx,
                        <div class=move || toggle_button_list_class(opened())>
                            <ul class="relative px-0 pt-2 pb-0 ml-5 before:w-0 before:block before:absolute before:inset-y-0 before:left-0 before:border-l-2 before:border-l-gray-200 dark:before:border-l-gray-600">
                                {children(cx)}
                            </ul>
                        </div>
                    }.into_view(cx))
                )
            }
        </div>
    }
}

fn toggle_link_active_class(is_active: bool) -> String {
    let mut classes = vec![
        "flex",
        "flex-nowrap",
        "items-center",
        "p-3",
        "gap-2",
        "rounded-md",
        "transition-colors",
    ];
    if is_active {
        classes.extend_from_slice(&[
            "text-white",
            "bg-brand-500",
            "shadow-lg",
            "hover:bg-brand-600",
        ]);
    } else {
        classes.extend_from_slice(&[
            "text-gray-500",
            "hover:text-gray-700",
            "hover:bg-gray-100",
            "dark:hover:text-gray-300",
            "dark:hover:bg-dark-vague-2",
        ]);
    }
    classes.join(" ")
}

fn toggle_link_text_active_class(is_active: bool) -> String {
    let mut classes = vec!["transition-colors"];
    if is_active {
        classes.extend_from_slice(&["text-brand-500", "dark:text-white"]);
    } else {
        classes.extend_from_slice(&[
            "text-gray-500",
            "hover:text-gray-700",
            "dark:hover:text-gray-300",
        ]);
    }
    classes.join(" ")
}

fn toggle_button_active_class(is_active: bool) -> String {
    let mut classes = vec![
        "flex",
        "flex-nowrap",
        "items-center",
        "w-full",
        "p-3",
        "gap-2",
        "rounded-md",
        "transition-colors",
    ];
    if is_active {
        classes.extend_from_slice(&[
            "text-white",
            "bg-brand-500",
            "shadow-lg",
            "hover:bg-brand-600",
        ]);
    } else {
        classes.extend_from_slice(&[
            "text-gray-500",
            "hover:text-gray-700",
            "hover:bg-gray-100",
            "dark:hover:text-gray-300",
            "dark:hover:bg-dark-vague-2",
        ]);
    }
    classes.join(" ")
}

fn toggle_button_list_class(is_opened: bool) -> String {
    let mut classes = vec!["overflow-hidden", "transition-all", "duration-200"];
    if !is_opened {
        classes.extend_from_slice(&["max-h-0", "hidden"]);
    }
    classes.join(" ")
}

fn render_arrow_icon(cx: Scope, is_active: bool, is_opened: bool) -> Fragment {
    let classes = vec![
        "absolute",
        "mt-[-5px]",
        "h-2",
        "w-[2px]",
        "top-1/2",
        "transition-all",
        "duration-200",
    ];

    let mut right_classes = Vec::from([classes.clone(), vec!["right-[9px]"]].concat());
    let mut left_classes = Vec::from([classes.clone(), vec!["left-[9px]"]].concat());

    if is_active {
        let classes = &["bg-white"];
        right_classes.extend_from_slice(classes);
        left_classes.extend_from_slice(classes);
    } else {
        let classes = &["bg-gray-500"];
        right_classes.extend_from_slice(classes);
        left_classes.extend_from_slice(classes);
    }
    if is_opened {
        right_classes.extend_from_slice(&["-rotate-45"]);
        left_classes.extend_from_slice(&["rotate-45"]);
    } else {
        right_classes.extend_from_slice(&["rotate-45"]);
        left_classes.extend_from_slice(&["-rotate-45"]);
    }
    view! { cx,
        <span class=move || right_classes.join(" ")></span>
        <span class=move || left_classes.join(" ")></span>
    }
}
