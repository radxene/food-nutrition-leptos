use serde_json::Error;

use crate::{
    models::entities::{SotrageAuthUser, SotrageAuthUserData},
    utils::storage::{StorageKeys, StorageUtil},
};

pub struct AuthValidatorUtil;

impl AuthValidatorUtil {
    pub fn validate_storage_user_locally() -> bool {
        let auth_user = Self::_get_storage_auth_user();

        if auth_user.is_ok() {
            let SotrageAuthUser { email, password } = auth_user.unwrap();
            Self::_is_right_user_data(email, password)
        } else {
            false
        }
    }

    pub fn validate_storage_user_data(raw_email: &str, raw_password: &str) -> bool {
        let auth_user = Self::_get_storage_auth_user();

        if auth_user.is_ok() {
            let SotrageAuthUser { email, password } = auth_user.unwrap();
            raw_email == email && raw_password == password
        } else {
            Self::_is_right_user_data(raw_email, raw_password)
        }
    }

    fn _get_storage_auth_user() -> Result<SotrageAuthUser, Error> {
        let auth_user =
            StorageUtil::get_item(&StorageKeys::AuthUser.to_string()).unwrap_or_default();

        serde_json::from_str::<SotrageAuthUser>(Box::leak(auth_user.into_boxed_str()))
    }

    fn _is_right_user_data(email: &str, password: &str) -> bool {
        email == SotrageAuthUserData::Email.to_string()
            && password == SotrageAuthUserData::Password.to_string()
    }
}
