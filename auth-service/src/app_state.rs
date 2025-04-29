use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{EmailClient, TokenStore};
use crate::domain::TwoFACodeStore;
use crate::domain::UserStore;

type EmailClientTraitObject        = dyn EmailClient     + Send + Sync;
type TokenStoreTraitObject         = dyn TokenStore      + Send + Sync;
type TwoFactorCodeStoreTraitObject = dyn TwoFACodeStore  + Send + Sync;
type UserStoreTraitObject          = dyn UserStore       + Send + Sync;
pub type EmailClientType           = Arc<RwLock<  EmailClientTraitObject>>;
pub type TokenStoreType            = Arc<RwLock<   TokenStoreTraitObject>>;
pub type TwoFactorCodeStoreType    = Arc<RwLock<TwoFactorCodeStoreTraitObject>>;
pub type UserStoreType             = Arc<RwLock<    UserStoreTraitObject>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store:        UserStoreType,
    pub banned_tokens:     TokenStoreType,
    pub two_fa_code_store: TwoFactorCodeStoreType,
    pub email_client:      EmailClientType,
}

impl AppState {
    pub fn new(
        user_store:        UserStoreType,
        banned_tokens:     TokenStoreType,
        two_fa_code_store: TwoFactorCodeStoreType,
        email_client:      EmailClientType,
        ) -> Self {
        AppState{user_store, banned_tokens, two_fa_code_store, email_client}
    }
}
