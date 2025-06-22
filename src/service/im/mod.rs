use std::sync::Arc;

use crate::{
    core::config::Config,
    service::im::{v1::V1, v2::V2},
};

pub mod v1;
pub mod v2;

pub struct ImService {
    pub v1: V1,
    pub v2: V2,
}

impl ImService {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            v1: V1::new((*config).clone()),
            v2: V2::new((*config).clone()),
        }
    }
}
