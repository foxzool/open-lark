pub mod attachment;
pub mod comment;
pub mod custom_field;
pub mod custom_field_option;
pub mod section;
pub mod task;
pub mod task_subtask;
pub mod tasklist;
pub mod tasklist_activity_subscription;

use crate::core::config::Config;

pub use attachment::AttachmentService;
pub use comment::CommentService;
pub use custom_field::CustomFieldService;
pub use custom_field_option::CustomFieldOptionService;
pub use section::SectionService;
pub use task::TaskService;
pub use task_subtask::TaskSubtaskService;
pub use tasklist::TasklistService;
pub use tasklist_activity_subscription::TasklistActivitySubscriptionService;

/// 任务模块服务 (v2)
pub struct TaskV2Service {
    /// 任务服务
    pub task: TaskService,
    /// 子任务服务
    pub task_subtask: TaskSubtaskService,
    /// 清单服务
    pub tasklist: TasklistService,
    /// 清单活动订阅服务
    pub tasklist_activity_subscription: TasklistActivitySubscriptionService,
    /// 评论服务
    pub comment: CommentService,
    /// 附件服务
    pub attachment: AttachmentService,
    /// 自定义分组服务
    pub section: SectionService,
    /// 自定义字段服务
    pub custom_field: CustomFieldService,
    /// 自定义字段选项服务
    pub custom_field_option: CustomFieldOptionService,
}

impl TaskV2Service {
    pub fn new(config: Config) -> Self {
        Self {
            task: TaskService::new(config.clone()),
            task_subtask: TaskSubtaskService::new(config.clone()),
            tasklist: TasklistService::new(config.clone()),
            tasklist_activity_subscription: TasklistActivitySubscriptionService::new(
                config.clone(),
            ),
            comment: CommentService::new(config.clone()),
            attachment: AttachmentService::new(config.clone()),
            section: SectionService::new(config.clone()),
            custom_field: CustomFieldService::new(config.clone()),
            custom_field_option: CustomFieldOptionService::new(config),
        }
    }
}
