use leptos::*;

use crate::providers::main_layout_provider::sidebar_width_slice;

#[component]
pub fn Content(cx: Scope, children: Children) -> impl IntoView {
    let (sidebar_width, _) = sidebar_width_slice(cx);

    let sidebar_classes = move || -> String {
        let mut classes = vec!["min-h-screen flex flex-col transition-margin".to_owned()];

        let sidebar_width = sidebar_width().to_string();

        let offset = sidebar_width.split("-").collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .unwrap();

        classes.push(format!("lg:ml-{}", offset));
        classes.join(" ")
    };

    view! { cx,
        <div class=move || sidebar_classes()>
            {children(cx)}
        </div>
    }
}
