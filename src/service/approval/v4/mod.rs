use crate::core::config::Config;

pub mod approval;
pub mod external_approval;
pub mod external_instance;
pub mod external_task;
pub mod file;
pub mod instance;
pub mod instance_comment;
pub mod message;
pub mod search;
pub mod task;

// Approval v4 事件模块
pub mod p2_approval_instance_approved_v4;
pub mod p2_approval_instance_created_v4;
pub mod p2_approval_instance_rejected_v4;

use approval::ApprovalService;
use external_approval::ExternalApprovalService;
use external_instance::ExternalInstanceService;
use external_task::ExternalTaskService;
use file::FileService;
use instance::InstanceService;
use instance_comment::InstanceCommentService;
use message::MessageService;
use search::SearchService;
use task::TaskService;

/// Approval v4服务
pub struct V4 {
    /// 原生审批定义服务
    pub approval: ApprovalService,
    /// 原生审批实例服务
    pub instance: InstanceService,
    /// 原生审批任务服务
    pub task: TaskService,
    /// 审批文件服务
    pub file: FileService,
    /// 审批评论服务
    pub instance_comment: InstanceCommentService,
    /// 三方审批定义服务
    pub external_approval: ExternalApprovalService,
    /// 三方审批实例服务
    pub external_instance: ExternalInstanceService,
    /// 三方审批任务服务
    pub external_task: ExternalTaskService,
    /// 审批消息服务
    pub message: MessageService,
    /// 审批查询服务
    pub search: SearchService,
}

impl V4 {
    pub fn new(config: Config) -> Self {
        Self {
            approval: ApprovalService::new(config.clone()),
            instance: InstanceService::new(config.clone()),
            task: TaskService::new(config.clone()),
            file: FileService::new(config.clone()),
            instance_comment: InstanceCommentService::new(config.clone()),
            external_approval: ExternalApprovalService::new(config.clone()),
            external_instance: ExternalInstanceService::new(config.clone()),
            external_task: ExternalTaskService::new(config.clone()),
            message: MessageService::new(config.clone()),
            search: SearchService::new(config),
        }
    }
}
