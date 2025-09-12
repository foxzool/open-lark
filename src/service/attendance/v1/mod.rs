use crate::{
    core::config::Config,
    service::attendance::v1::{
        archive_rule::ArchiveRuleService, group::GroupService,
        leave_accrual_record::LeaveAccrualRecordService,
        leave_employ_expire_record::LeaveEmployExpireRecordService, shift::ShiftService,
        user_approval::UserApprovalService, user_daily_shift::UserDailyShiftService,
        user_setting::UserSettingService, user_stats_data::UserStatsDataService,
        user_task::UserTaskService, user_task_remedy::UserTaskRemedyService,
    },
};

pub mod archive_rule;
pub mod group;
pub mod leave_accrual_record;
pub mod leave_employ_expire_record;
pub mod models;
pub mod p2_attendance_user_task_status_change_v1;
pub mod p2_attendance_user_task_updated_v1;
pub mod shift;
pub mod user_approval;
pub mod user_daily_shift;
pub mod user_setting;
pub mod user_stats_data;
pub mod user_task;
pub mod user_task_remedy;

pub struct V1 {
    pub shift: ShiftService,
    pub user_daily_shift: UserDailyShiftService,
    pub group: GroupService,
    pub user_setting: UserSettingService,
    pub user_stats_data: UserStatsDataService,
    pub user_approval: UserApprovalService,
    pub user_task: UserTaskService,
    pub user_task_remedy: UserTaskRemedyService,
    pub archive_rule: ArchiveRuleService,
    pub leave_employ_expire_record: LeaveEmployExpireRecordService,
    pub leave_accrual_record: LeaveAccrualRecordService,
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
            user_approval: UserApprovalService {
                config: config.clone(),
            },
            user_task: UserTaskService {
                config: config.clone(),
            },
            user_task_remedy: UserTaskRemedyService {
                config: config.clone(),
            },
            archive_rule: ArchiveRuleService {
                config: config.clone(),
            },
            leave_employ_expire_record: LeaveEmployExpireRecordService {
                config: config.clone(),
            },
            leave_accrual_record: LeaveAccrualRecordService {
                config: config.clone(),
            },
        }
    }
}
