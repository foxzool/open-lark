pub mod v3;

pub struct SheetsService {
    pub v3: v3::V3,
}

impl SheetsService {
    pub fn new(config: crate::core::config::Config) -> Self {
        Self {
            v3: v3::V3::new(config),
        }
    }
}
