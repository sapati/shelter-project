use crate::settings::Settings;
use arc_swap::ArcSwap;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct ApplicationState {
    pub db_conn: ArcSwap<DatabaseConnection>,
    pub settings: ArcSwap<Settings>,
}

impl ApplicationState {
    pub fn new(settings: &Settings, db_conn: DatabaseConnection) -> anyhow::Result<Self> {
        Ok(Self {
            db_conn: ArcSwap::new(Arc::new(db_conn)),
            settings: ArcSwap::new(Arc::new((*settings).clone())),
        })
    }
}
