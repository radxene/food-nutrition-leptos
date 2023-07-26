use serde::{Deserialize, Serialize};

use crate::utils::{
    storage::{StorageKeys, StorageUtil},
    validators::AuthValidatorUtil,
};

#[derive(Copy, Clone, PartialEq, strum_macros::Display, strum_macros::EnumString)]
pub enum SotrageAuthUserData {
    #[strum(serialize = "barrow@occurs.rs")]
    Email,
    #[strum(serialize = ")))")]
    Password,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub struct SotrageAuthUser {
    pub email: &'static str,
    pub password: &'static str,
}

impl SotrageAuthUser {
    pub fn login(email: String, password: String) {
        if AuthValidatorUtil::validate_storage_user_data(&email, &password) {
            StorageUtil::set_item(
                &StorageKeys::AuthUser.to_string(),
                &serde_json::to_string(&Self {
                    email: Box::leak(email.into_boxed_str()),
                    password: Box::leak(password.into_boxed_str()),
                })
                .unwrap(),
            )
        }
    }

    pub fn logout() {
        StorageUtil::remove_item(&StorageKeys::AuthUser.to_string())
    }
}
