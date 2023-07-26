mod models;
mod providers;
mod routes;
mod utils;
mod view;

use leptos::*;

use providers::Providers;
use utils::theme::ThemeUtil;
use view::components::Application;

fn main() {
    ThemeUtil::init();

    mount_to_body(|cx| {
        Providers::init(cx);

        view! { cx,  <Application /> }
    })
}
