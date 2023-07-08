mod services;
mod view;

use leptos::*;

use services::theme::ThemeService;
use view::widgets::application::Application;

fn main() {
    ThemeService::init();
    mount_to_body(|cx| view! { cx,  <Application /> })
}
