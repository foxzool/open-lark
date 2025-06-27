use crate::core::config::Config;

pub mod knowledge;
pub mod message;
pub mod models;
pub mod run;
pub mod session;
pub mod skill;

use knowledge::KnowledgeService;
use message::MessageService;
use run::RunService;
use session::SessionService;
use skill::SkillService;

/// 飞书智能伙伴创建平台服务
///
/// 提供飞书智能伙伴创建平台（aily）的完整功能，包括：
/// - 会话管理：创建、更新、查询、删除智能伙伴会话
/// - 消息管理：发送消息、获取消息、列出消息历史
/// - 运行管理：创建运行、查询运行状态、取消运行
/// - 技能管理：调用技能、获取技能信息、查询技能列表
/// - 知识问答：数据知识问答、知识库管理、文件上传处理
pub struct AilyService {
    /// 会话管理服务
    pub session: SessionService,
    /// 消息管理服务
    pub message: MessageService,
    /// 运行管理服务
    pub run: RunService,
    /// 技能管理服务
    pub skill: SkillService,
    /// 知识问答服务
    pub knowledge: KnowledgeService,
}

impl AilyService {
    pub fn new(config: Config) -> Self {
        Self {
            session: SessionService::new(config.clone()),
            message: MessageService::new(config.clone()),
            run: RunService::new(config.clone()),
            skill: SkillService::new(config.clone()),
            knowledge: KnowledgeService::new(config),
        }
    }
}
