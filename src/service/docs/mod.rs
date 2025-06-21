use crate::core::config::Config;

pub mod v1;

pub struct DocsService {
    pub v1: v1::V1,
}

impl DocsService {
    pub(crate) fn new(config: Config) -> Self {
        DocsService {
            v1: v1::V1::new(config),
        }
    }
}
