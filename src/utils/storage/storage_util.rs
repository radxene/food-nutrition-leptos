use web_sys::{window, Storage};

#[derive(Copy, Clone, PartialEq, strum_macros::Display, strum_macros::EnumString)]
pub enum StorageKeys {
    #[strum(serialize = "auth_user")]
    AuthUser,
    #[strum(serialize = "theme")]
    Theme,
}

pub struct StorageUtil {}

impl StorageUtil {
    pub fn get_item(key: &str) -> Option<String> {
        Self::get_storage().get_item(key).unwrap()
    }

    pub fn set_item(key: &str, val: &str) {
        Self::get_storage().set_item(key, val).unwrap();
    }

    pub fn remove_item(key: &str) {
        Self::get_storage().remove_item(key).unwrap()
    }

    fn get_storage() -> Storage {
        window()
            .unwrap()
            .local_storage()
            .expect("LocalStorage not exported from web-sys.")
            .unwrap()
    }
}
