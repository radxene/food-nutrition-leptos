use js_sys::Array;
use std::str::FromStr;
use wasm_bindgen::JsValue;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, MediaQueryListEvent};

use crate::services::media_query::MediaQueryService;
use crate::services::storage::StorageService;

const PREFERS_COLOR_SCHEME_DARK: &'static str = "(prefers-color-scheme: dark)";

#[derive(Clone, PartialEq, strum_macros::Display, strum_macros::EnumString)]
pub enum ThemeMode {
    #[strum(serialize = "dark")]
    Dark,
    #[strum(serialize = "light")]
    Light,
}

pub struct ThemeService {}

impl ThemeService {
    pub fn init() {
        Self::apply_document_theme_mode(Self::get_preferred_color_schema());
        Self::_watch_preferred();
    }

    pub fn is_dark_mode() -> bool {
        ThemeService::get_preferred_color_schema() == ThemeMode::Dark
    }

    pub fn get_preferred_color_schema() -> ThemeMode {
        let mut theme: ThemeMode;

        if MediaQueryService::get_media_query_list(PREFERS_COLOR_SCHEME_DARK).matches() {
            theme = ThemeMode::Dark;
        } else {
            theme = ThemeMode::Light;
        }

        if let Some(stored_theme) = StorageService::get_item("theme") {
            theme = ThemeMode::from_str(&stored_theme).unwrap();
        }

        return theme;
    }

    pub fn set_preferred_color_schema(theme_mode: ThemeMode) {
        let theme_mode_string = theme_mode.to_string();
        Self::apply_document_theme_mode(theme_mode);
        StorageService::set_item("theme", &theme_mode_string);
    }

    pub fn apply_document_theme_mode(theme_mode: ThemeMode) {
        let element = window().unwrap().document().unwrap().document_element();

        for item in [ThemeMode::Dark, ThemeMode::Light].iter() {
            element
                .as_ref()
                .unwrap()
                .class_list()
                .remove(&Array::of1(&JsValue::from_str(&item.to_string())))
                .unwrap();
        }

        element
            .unwrap()
            .class_list()
            .add(&Array::of1(&JsValue::from_str(&theme_mode.to_string())))
            .unwrap();
    }

    fn _watch_preferred() {
        let cb = Closure::wrap(Box::new(|ev: MediaQueryListEvent| {
            let mode = if StorageService::get_item("theme").is_none() && ev.matches() {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            };
            StorageService::set_item("theme", &mode.to_string());
        }) as Box<dyn FnMut(_)>);

        MediaQueryService::watch_media_query_list(
            PREFERS_COLOR_SCHEME_DARK,
            "change",
            cb.as_ref().unchecked_ref(),
        );

        cb.forget();
    }
}
