use crate::core::config::Config;

pub mod v1;

pub struct DocxService {
    pub v1: v1::V1,
}

impl DocxService {
    pub fn new(config: Config) -> Self {
        DocxService {
            v1: v1::V1::new(config.clone()),
        }
    }
}
