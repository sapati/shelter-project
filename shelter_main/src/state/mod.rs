use crate::settings::Settings;
use arc_swap::ArcSwap;
use std::sync::Arc;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
}

impl ApplicationState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
        })
    }
}
