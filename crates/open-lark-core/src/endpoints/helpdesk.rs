//! helpdesk 服务端点常量定义

/// 客服管理
pub const HELPDESK_V1_AGENT_GET: &str = "/open-apis/helpdesk/v1/agents/{agent_id}";
pub const HELPDESK_V1_AGENT_EMAIL: &str = "/open-apis/helpdesk/v1/agents/{agent_id}/email";

/// 客服排班管理
pub const HELPDESK_V1_AGENT_SCHEDULES: &str = "/open-apis/helpdesk/v1/agent_schedules";
pub const HELPDESK_V1_AGENT_SCHEDULE_DELETE: &str =
    "/open-apis/helpdesk/v1/agent_schedules/{agent_schedule_id}";

/// 客服技能管理
pub const HELPDESK_V1_AGENT_SKILLS: &str = "/open-apis/helpdesk/v1/agent_skills";
pub const HELPDESK_V1_AGENT_SKILL_CREATE: &str = "/open-apis/helpdesk/v1/agent_skills";
pub const HELPDESK_V1_AGENT_SKILL_GET: &str =
    "/open-apis/helpdesk/v1/agent_skills/{agent_skill_id}";
pub const HELPDESK_V1_AGENT_SKILL_DELETE: &str =
    "/open-apis/helpdesk/v1/agent_skills/{agent_skill_id}";

/// 客服技能规则管理
pub const HELPDESK_V1_AGENT_SKILL_RULES: &str = "/open-apis/helpdesk/v1/agent_skill_rules";
pub const HELPDESK_V1_AGENT_SKILL_RULES_OPERATOR_OPTIONS: &str =
    "/open-apis/helpdesk/v1/agent_skill_rules/operator_options";

/// 分类管理
pub const HELPDESK_V1_CATEGORIES: &str = "/open-apis/helpdesk/v1/categories";
pub const HELPDESK_V1_CATEGORY_GET: &str = "/open-apis/helpdesk/v1/categories/{category_id}";

/// 事件订阅管理
pub const HELPDESK_V1_EVENTS_SUBSCRIBE: &str = "/open-apis/helpdesk/v1/events/subscribe";
pub const HELPDESK_V1_EVENTS_UNSUBSCRIBE: &str = "/open-apis/helpdesk/v1/events/unsubscribe";

/// FAQ管理
pub const HELPDESK_V1_FAQS: &str = "/open-apis/helpdesk/v1/faqs";
pub const HELPDESK_V1_FAQS_SEARCH: &str = "/open-apis/helpdesk/v1/faqs/search";
pub const HELPDESK_V1_FAQ_CREATE: &str = "/open-apis/helpdesk/v1/faqs";
pub const HELPDESK_V1_FAQ_GET: &str = "/open-apis/helpdesk/v1/faqs/{faq_id}";
pub const HELPDESK_V1_FAQ_DELETE: &str = "/open-apis/helpdesk/v1/faqs/{faq_id}";
pub const HELPDESK_V1_FAQ_UPDATE: &str = "/open-apis/helpdesk/v1/faqs/{faq_id}";
pub const HELPDESK_V1_FAQ_IMAGE: &str = "/open-apis/helpdesk/v1/faq_images";

/// 通知管理
pub const HELPDESK_V1_NOTIFICATIONS: &str = "/open-apis/helpdesk/v1/notifications";
pub const HELPDESK_V1_NOTIFICATION_GET: &str =
    "/open-apis/helpdesk/v1/notifications/{notification_id}";
pub const HELPDESK_V1_NOTIFICATION_UPDATE: &str =
    "/open-apis/helpdesk/v1/notifications/{notification_id}";
pub const HELPDESK_V1_NOTIFICATION_PREVIEW: &str =
    "/open-apis/helpdesk/v1/notifications/{notification_id}/preview";
pub const HELPDESK_V1_NOTIFICATION_SUBMIT_APPROVE: &str =
    "/open-apis/helpdesk/v1/notifications/{notification_id}/submit_approve";
pub const HELPDESK_V1_NOTIFICATION_CANCEL_APPROVE: &str =
    "/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_approve";
pub const HELPDESK_V1_NOTIFICATION_EXECUTE_SEND: &str =
    "/open-apis/helpdesk/v1/notifications/{notification_id}/execute_send";
pub const HELPDESK_V1_NOTIFICATION_CANCEL_SEND: &str =
    "/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_send";

/// 服务启动
pub const HELPDESK_V1_START_SERVICE: &str = "/open-apis/helpdesk/v1/start_service";

/// 工单管理
pub const HELPDESK_V1_TICKETS: &str = "/open-apis/helpdesk/v1/tickets";
pub const HELPDESK_V1_TICKET_GET: &str = "/open-apis/helpdesk/v1/tickets/{ticket_id}";
pub const HELPDESK_V1_TICKET_MESSAGES: &str = "/open-apis/helpdesk/v1/tickets/{ticket_id}/messages";
pub const HELPDESK_V1_TICKET_BOT_MESSAGES: &str =
    "/open-apis/helpdesk/v1/tickets/{ticket_id}/bot_messages";

/// 工单自定义字段
pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELDS: &str =
    "/open-apis/helpdesk/v1/ticket_customized_fields";
pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_GET: &str =
    "/open-apis/helpdesk/v1/ticket_customized_fields/{ticket_customized_field_id}";
pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_UPDATE: &str =
    "/open-apis/helpdesk/v1/ticket_customized_fields/{ticket_customized_field_id}";
pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_DELETE: &str =
    "/open-apis/helpdesk/v1/ticket_customized_fields/{ticket_customized_field_id}";
