use web_sys::{window, Storage};

pub struct StorageService {}

impl StorageService {
    pub fn get_item(key: &str) -> Option<String> {
        Self::get_storage().get_item(key).unwrap()
    }

    pub fn set_item(key: &str, val: &str) {
        Self::get_storage().set_item(key, val).unwrap();
    }

    fn get_storage() -> Storage {
        window()
            .unwrap()
            .local_storage()
            .expect("LocalStorage not exported from web-sys.")
            .unwrap()
    }
}
