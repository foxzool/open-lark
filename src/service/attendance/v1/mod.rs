use crate::{
    core::config::Config,
    service::attendance::v1::{
        group::GroupService, shift::ShiftService, user_approval::UserApprovalService,
        user_daily_shift::UserDailyShiftService, user_setting::UserSettingService,
        user_stats_data::UserStatsDataService,
    },
};

pub mod group;
pub mod models;
pub mod p2_attendance_user_task_status_change_v1;
pub mod p2_attendance_user_task_updated_v1;
pub mod shift;
pub mod user_approval;
pub mod user_daily_shift;
pub mod user_setting;
pub mod user_stats_data;

pub struct V1 {
    pub shift: ShiftService,
    pub user_daily_shift: UserDailyShiftService,
    pub group: GroupService,
    pub user_setting: UserSettingService,
    pub user_stats_data: UserStatsDataService,
    pub user_approval: UserApprovalService,
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
            group: GroupService {
                config: config.clone(),
            },
            user_setting: UserSettingService {
                config: config.clone(),
            },
            user_stats_data: UserStatsDataService {
                config: config.clone(),
            },
            user_approval: UserApprovalService { config },
        }
    }
}
