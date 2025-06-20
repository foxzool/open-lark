use crate::{
    core::config::Config,
    service::attendance::v1::{
        group::GroupService, shift::ShiftService, user_daily_shift::UserDailyShiftService,
    },
};

pub mod group;
pub mod models;
pub mod shift;
pub mod user_daily_shift;

pub struct V1 {
    pub shift: ShiftService,
    pub user_daily_shift: UserDailyShiftService,
    pub group: GroupService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            shift: ShiftService {
                config: config.clone(),
            },
            user_daily_shift: UserDailyShiftService {
                config: config.clone(),
            },
            group: GroupService { config },
        }
    }
}
