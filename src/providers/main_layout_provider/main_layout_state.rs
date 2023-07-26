use leptos::*;

#[derive(Copy, Clone, PartialEq, Default, strum_macros::Display, strum_macros::EnumString)]
pub enum SidebarWidthEnum {
    #[strum(serialize = "w-0")]
    Collapsed,
    #[strum(serialize = "w-20")]
    Minimized,
    #[strum(serialize = "w-64")]
    #[default]
    Expanded,
}

#[derive(Copy, Clone, Default)]
pub struct MainLayoutState {
    pub is_leaved: bool,
    pub is_minimized: bool,
    pub sidebar_width: SidebarWidthEnum,
}

pub fn is_minimized_sidebar_slice(cx: Scope) -> (Signal<bool>, SignalSetter<bool>) {
    create_slice(
        cx,
        expect_context::<RwSignal<MainLayoutState>>(cx),
        |state| state.is_minimized,
        |state, new_value| state.is_minimized = new_value,
    )
}

pub fn is_leaved_sidebar_slice(cx: Scope) -> (Signal<bool>, SignalSetter<bool>) {
    create_slice(
        cx,
        expect_context::<RwSignal<MainLayoutState>>(cx),
        |state| state.is_leaved,
        |state, new_value| state.is_leaved = new_value,
    )
}

pub fn sidebar_width_slice(
    cx: Scope,
) -> (Signal<SidebarWidthEnum>, SignalSetter<SidebarWidthEnum>) {
    create_slice(
        cx,
        expect_context::<RwSignal<MainLayoutState>>(cx),
        |state| state.sidebar_width,
        |state, new_value| state.sidebar_width = new_value,
    )
}
