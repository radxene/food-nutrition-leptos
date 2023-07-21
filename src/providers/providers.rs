use leptos::*;

use super::main_layout_provider::MainLayoutState;

pub struct Providers;

impl Providers {
    pub fn init(cx: Scope) {
        Self::main_layout_provider(cx);
    }

    pub fn main_layout_provider(cx: Scope) {
        provide_context(cx, create_rw_signal(cx, MainLayoutState::default()));
    }
}
