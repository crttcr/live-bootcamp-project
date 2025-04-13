use std::sync::Arc;
use tokio::sync::RwLock;

use crate::services::hashmap_user_store::HashmapUserStore;

type UserStoreType = Arc<RwLock<HashmapUserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        AppState { user_store }
    }
}
