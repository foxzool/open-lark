pub mod v1;

pub struct BitableService {
    pub v1: v1::V1,
}

impl BitableService {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}
