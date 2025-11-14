//! Aily 服务端点常量定义

// ===== 会话管理 =====

/// 会话管理
pub const AILY_V1_SESSIONS: &str = "/open-apis/aily/v1/sessions";

/// 会话操作（获取/更新/删除）
pub const AILY_V1_SESSION_OPERATION: &str = "/open-apis/aily/v1/sessions/{session_id}";

// ===== 运行管理 =====

/// 运行列表/创建
pub const AILY_V1_RUNS: &str = "/open-apis/aily/v1/sessions/{session_id}/runs";

/// 运行操作（获取）
pub const AILY_V1_RUN_GET: &str = "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}";

/// 取消运行
pub const AILY_V1_RUN_CANCEL: &str =
    "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}/cancel";

// ===== 知识库管理 =====

/// 知识库问答
pub const AILY_V1_DATA_KNOWLEDGE_ASK: &str = "/open-apis/aily/v1/data_knowledge/ask";

/// 知识库文件上传
pub const AILY_V1_DATA_KNOWLEDGE_UPLOAD_FILE: &str =
    "/open-apis/aily/v1/data_knowledge/upload_file";

/// 知识库操作（创建/列表）
pub const AILY_V1_DATA_KNOWLEDGE: &str = "/open-apis/aily/v1/data_knowledge";

/// 知识库操作（获取/删除）
pub const AILY_V1_DATA_KNOWLEDGE_OPERATION: &str =
    "/open-apis/aily/v1/data_knowledge/{knowledge_id}";

/// 知识库分类
pub const AILY_V1_DATA_KNOWLEDGE_CATEGORIES: &str = "/open-apis/aily/v1/data_knowledge/categories";

// ===== 消息管理 =====

/// 消息操作（创建/列表）
pub const AILY_V1_MESSAGES: &str = "/open-apis/aily/v1/sessions/{session_id}/messages";

/// 消息获取
pub const AILY_V1_MESSAGE_GET: &str =
    "/open-apis/aily/v1/sessions/{session_id}/messages/{message_id}";

// ===== 技能管理 =====

/// 技能启动
pub const AILY_V1_SKILL_START: &str = "/open-apis/aily/v1/skills/{skill_id}/start";

/// 技能获取
pub const AILY_V1_SKILL_GET: &str = "/open-apis/aily/v1/skills/{skill_id}";

/// 技能列表
pub const AILY_V1_SKILLS: &str = "/open-apis/aily/v1/skills";
