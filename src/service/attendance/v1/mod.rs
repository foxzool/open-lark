use crate::{
    core::config::Config,
    service::attendance::v1::{shift::ShiftService, user_daily_shift::UserDailyShiftService},
};

pub mod models;
pub mod shift;
pub mod user_daily_shift;

pub struct V1 {
    pub shift: ShiftService,
    pub user_daily_shift: UserDailyShiftService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            shift: ShiftService {
                config: config.clone(),
            },
            user_daily_shift: UserDailyShiftService { config },
        }
    }
}
