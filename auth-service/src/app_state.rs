use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::data_stores::UserStore;
use crate::domain::TokenStore;

type UserStoreTraitObject  = dyn UserStore  + Send + Sync;
type TokenStoreTraitObject = dyn TokenStore + Send + Sync;
pub type UserStoreType     = Arc<RwLock< UserStoreTraitObject>>;
pub type TokenStoreType    = Arc<RwLock<TokenStoreTraitObject>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store:    UserStoreType,
    pub banned_tokens: TokenStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType, banned_tokens: TokenStoreType) -> Self {
        AppState{user_store, banned_tokens}
    }
}
