use crate::{core::config::Config, service::attendance::v1::shift::ShiftService};

pub mod models;
pub mod shift;

pub struct V1 {
    pub shift: ShiftService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            shift: ShiftService { config },
        }
    }
}
