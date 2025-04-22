use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::data_stores::UserStore;

type UserStoreTraitObject = dyn UserStore + Send + Sync;
type UserStoreType        = Arc<RwLock<UserStoreTraitObject>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        AppState{user_store}
    }
}
