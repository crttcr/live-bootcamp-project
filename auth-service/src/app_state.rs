use crate::domain::data_stores::UserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

type UserStoreTO   = dyn UserStore + Send + Sync;
type UserStoreType = Arc<RwLock<UserStoreTO>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        AppState { user_store }
    }
}
