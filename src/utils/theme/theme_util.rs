use js_sys::Array;
use std::str::FromStr;
use wasm_bindgen::JsValue;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, MediaQueryListEvent};

use crate::utils::media_query::MediaQueryUtil;
use crate::utils::storage::{StorageKeys, StorageUtil};

const PREFERS_COLOR_SCHEME_DARK: &'static str = "(prefers-color-scheme: dark)";

#[derive(Clone, PartialEq, strum_macros::Display, strum_macros::EnumString)]
pub enum ThemeMode {
    #[strum(serialize = "dark")]
    Dark,
    #[strum(serialize = "light")]
    Light,
}

pub struct ThemeUtil {}

impl ThemeUtil {
    pub fn init() {
        Self::apply_document_theme_mode(Self::get_preferred_color_schema());
        Self::_watch_preferred();
    }

    pub fn is_dark_mode() -> bool {
        Self::get_preferred_color_schema() == ThemeMode::Dark
    }

    pub fn get_preferred_color_schema() -> ThemeMode {
        let mut theme: ThemeMode;

        if MediaQueryUtil::get_media_query_list(PREFERS_COLOR_SCHEME_DARK).matches() {
            theme = ThemeMode::Dark;
        } else {
            theme = ThemeMode::Light;
        }

        if let Some(stored_theme) = StorageUtil::get_item(&StorageKeys::Theme.to_string()) {
            theme = ThemeMode::from_str(&stored_theme).unwrap();
        }

        return theme;
    }

    pub fn set_preferred_color_schema(theme_mode: ThemeMode) {
        let theme_mode_string = theme_mode.to_string();
        Self::apply_document_theme_mode(theme_mode);
        StorageUtil::set_item(&StorageKeys::Theme.to_string(), &theme_mode_string);
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
            let mode = if StorageUtil::get_item(&StorageKeys::Theme.to_string()).is_none()
                && ev.matches()
            {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            };
            StorageUtil::set_item(&StorageKeys::Theme.to_string(), &mode.to_string());
        }) as Box<dyn FnMut(_)>);

        MediaQueryUtil::watch_media_query_list(
            PREFERS_COLOR_SCHEME_DARK,
            "change",
            cb.as_ref().unchecked_ref(),
        );

        cb.forget();
    }
}
