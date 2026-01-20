pub mod get;
pub mod models;

use std::sync::Arc;
use openlark_core::config::Config;

#[derive(Clone)]
pub struct App {
    config: Arc<Config>,
}

impl App {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn get(&self) -> get::GetAppRequest {
        get::GetAppRequest::new(self.config.clone())
    }
}

pub use get::GetAppRequest;
pub use models::*;
