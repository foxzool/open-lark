use crate::core::config::Config;
use crate::service::im::v1::V1;
use crate::service::im::v2::V2;

pub mod v1;
pub mod v2;

pub mod share;

pub struct ImService {
    pub v1: V1,
    pub v2: V2,
}

impl ImService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: V1::new(config.clone()),
            v2: V2::new(config),
        }
    }
}
