//! OpenLark AI 服务端点定义
//!
//! 此模块包含AI和智能服务相关的所有API端点常量，从 openlark-core 迁移而来。
//! 包含文档AI识别、AI嵌入、AI工作流、Aily智能助手等完整功能。
//!
//! # 服务模块包含
//!
//! - **document_ai**: 文档AI识别（简历、身份证、驾驶证、银行卡等）
//! - **ai_embedding**: AI嵌入服务
//! - **ai_workflow**: AI工作流服务
//! - **aily**: Aily智能助手（会话、运行、知识库、消息、技能管理）
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_ai::endpoints::*;
//!
//! // 文档AI识别
//! let resume_parse_endpoint = DOCUMENT_AI_RESUME_PARSE;
//! let id_card_endpoint = DOCUMENT_AI_ID_CARD_RECOGNIZE;
//!
//! // Aily智能助手
//! let session_endpoint = AILY_V1_SESSIONS;
//! let knowledge_endpoint = AILY_V1_DATA_KNOWLEDGE_ASK;
//!
//! // AI工作流
//! let workflow_endpoint = AI_V1_WORKFLOWS;
//! ```

// 导入核心端点（auth, application等基础端点）
pub use openlark_core::endpoints::{auth, application, apass, platform_integration};
pub use openlark_core::endpoints::{Endpoints, EndpointBuilder};

// ===== 文档AI识别端点 (Document AI) =====

/// 简历解析
pub const DOCUMENT_AI_RESUME_PARSE: &str = "/open-apis/document_ai/v1/resume_parse";

/// 身份证识别
pub const DOCUMENT_AI_ID_CARD_RECOGNIZE: &str = "/open-apis/document_ai/v1/id_card_recognize";

/// 驾驶证识别
pub const DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE: &str = "/open-apis/document_ai/v1/driving_license_recognize";

/// 银行卡识别
pub const DOCUMENT_AI_BANK_CARD_RECOGNIZE: &str = "/open-apis/document_ai/v1/bank_card_recognize";

/// 名片识别
pub const DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE: &str = "/open-apis/document_ai/v1/business_card_recognize";

/// 营业执照识别
pub const DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE: &str = "/open-apis/document_ai/v1/business_license_recognize";

/// 中国护照识别
pub const DOCUMENT_AI_CHINESE_PASSPORT_RECOGNIZE: &str = "/open-apis/document_ai/v1/chinese_passport_recognize";

/// 合同字段提取
pub const DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION: &str = "/open-apis/document_ai/v1/contract_field_extraction";

/// 食品经营许可证识别
pub const DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE: &str = "/open-apis/document_ai/v1/food_manage_license_recognize";

/// 食品生产许可证识别
pub const DOCUMENT_AI_FOOD_PRODUCE_LICENSE_RECOGNIZE: &str = "/open-apis/document_ai/v1/food_produce_license_recognize";

/// 健康证识别
pub const DOCUMENT_AI_HEALTH_CERTIFICATE_RECOGNIZE: &str = "/open-apis/document_ai/v1/health_certificate_recognize";

/// 港澳通行证识别
pub const DOCUMENT_AI_HONG_KONG_MACAU_PASS_RECOGNIZE: &str = "/open-apis/document_ai/v1/hong_kong_macau_pass_recognize";

/// 户口本识别
pub const DOCUMENT_AI_HOUSEHOLD_REGISTER_RECOGNIZE: &str = "/open-apis/document_ai/v1/household_register_recognize";

/// 出生医学证明识别
pub const DOCUMENT_AI_BIRTH_CERTIFICATE_RECOGNIZE: &str = "/open-apis/document_ai/v1/birth_certificate_recognize";

/// 表格识别
pub const DOCUMENT_AI_FORM_RECOGNIZE: &str = "/open-apis/document_ai/v1/form_recognize";

/// 发票识别
pub const DOCUMENT_AI_INVOICE_RECOGNIZE: &str = "/open-apis/document_ai/v1/invoice_recognize";

/// 税务登记证识别
pub const DOCUMENT_AI_TAX_REGISTRATION_CERT_RECOGNIZE: &str = "/open-apis/document_ai/v1/tax_registration_cert_recognize";

/// 社会信用代码识别
pub const DOCUMENT_AI_SOCIAL_CREDIT_CODE_RECOGNIZE: &str = "/open-apis/document_ai/v1/social_credit_code_recognize";

/// 学历证书识别
pub const DOCUMENT_AI_EDUCATION_CERTIFICATE_RECOGNIZE: &str = "/open-apis/document_ai/v1/education_certificate_recognize";

/// 车辆识别
pub const DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE: &str = "/open-apis/document_ai/v1/vehicle_license_recognize";

/// 房产证识别
pub const DOCUMENT_AI_REAL_ESTATE_CERT_RECOGNIZE: &str = "/open-apis/document_ai/v1/real_estate_cert_recognize";

/// 企业信息识别
pub const DOCUMENT_AI_ENTERPRISE_INFO_RECOGNIZE: &str = "/open-apis/document_ai/v1/enterprise_info_recognize";

/// 表格提取
pub const DOCUMENT_AI_TABLE_EXTRACTION: &str = "/open-apis/document_ai/v1/table_extraction";

/// 文档分类
pub const DOCUMENT_AI_DOCUMENT_CLASSIFICATION: &str = "/open-apis/document_ai/v1/document_classification";

/// OCR识别
pub const DOCUMENT_AI_OCR_RECOGNIZE: &str = "/open-apis/document_ai/v1/ocr_recognize";

// ===== AI嵌入服务端点 (AI Embedding) =====

/// 文本嵌入
pub const AI_EMBEDDING_V1_TEXT_EMBEDDING: &str = "/open-apis/ai/v1/text_embedding";

/// 批量文本嵌入
pub const AI_EMBEDDING_V1_TEXT_EMBEDDING_BATCH: &str = "/open-apis/ai/v1/text_embedding/batch";

/// 对话嵌入
pub const AI_EMBEDDING_V1_CHAT_EMBEDDING: &str = "/open-apis/ai/v1/chat_embedding";

/// 图像嵌入
pub const AI_EMBEDDING_V1_IMAGE_EMBEDDING: &str = "/open-apis/ai/v1/image_embedding";

/// 向量相似度计算
pub const AI_EMBEDDING_V1_SIMILARITY: &str = "/open-apis/ai/v1/similarity";

// ===== AI工作流端点 (AI Workflow) =====

/// 工作流管理
pub const AI_V1_WORKFLOWS: &str = "/open-apis/ai/v1/workflows";

/// 工作流操作
pub const AI_V1_WORKFLOW_OPERATION: &str = "/open-apis/ai/v1/workflows/{workflow_id}";

/// 工作流运行
pub const AI_V1_WORKFLOW_RUNS: &str = "/open-apis/ai/v1/workflows/{workflow_id}/runs";

/// 工作流运行操作
pub const AI_V1_WORKFLOW_RUN_OPERATION: &str = "/open-apis/ai/v1/workflows/{workflow_id}/runs/{run_id}";

/// 工作流节点
pub const AI_V1_WORKFLOW_NODES: &str = "/open-apis/ai/v1/workflows/{workflow_id}/nodes";

/// 工作流节点操作
pub const AI_V1_WORKFLOW_NODE_OPERATION: &str = "/open-apis/ai/v1/workflows/{workflow_id}/nodes/{node_id}";

// ===== Aily智能助手端点 =====

/// 会话管理
pub const AILY_V1_SESSIONS: &str = "/open-apis/aily/v1/sessions";

/// 会话操作（获取/更新/删除）
pub const AILY_V1_SESSION_OPERATION: &str = "/open-apis/aily/v1/sessions/{session_id}";

/// 运行列表/创建
pub const AILY_V1_RUNS: &str = "/open-apis/aily/v1/sessions/{session_id}/runs";

/// 运行操作（获取）
pub const AILY_V1_RUN_GET: &str = "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}";

/// 取消运行
pub const AILY_V1_RUN_CANCEL: &str = "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}/cancel";

/// 知识库问答
pub const AILY_V1_DATA_KNOWLEDGE_ASK: &str = "/open-apis/aily/v1/data_knowledge/ask";

/// 知识库文件上传
pub const AILY_V1_DATA_KNOWLEDGE_UPLOAD_FILE: &str = "/open-apis/aily/v1/data_knowledge/upload_file";

/// 知识库操作（创建/列表）
pub const AILY_V1_DATA_KNOWLEDGE: &str = "/open-apis/aily/v1/data_knowledge";

/// 知识库操作（获取/删除）
pub const AILY_V1_DATA_KNOWLEDGE_OPERATION: &str = "/open-apis/aily/v1/data_knowledge/{knowledge_id}";

/// 知识库分类
pub const AILY_V1_DATA_KNOWLEDGE_CATEGORIES: &str = "/open-apis/aily/v1/data_knowledge/categories";

/// 消息操作（创建/列表）
pub const AILY_V1_MESSAGES: &str = "/open-apis/aily/v1/sessions/{session_id}/messages";

/// 消息获取
pub const AILY_V1_MESSAGE_GET: &str = "/open-apis/aily/v1/sessions/{session_id}/messages/{message_id}";

/// 技能启动
pub const AILY_V1_SKILL_START: &str = "/open-apis/aily/v1/skills/{skill_id}/start";

/// 技能获取
pub const AILY_V1_SKILL_GET: &str = "/open-apis/aily/v1/skills/{skill_id}";

/// 技能列表
pub const AILY_V1_SKILLS: &str = "/open-apis/aily/v1/skills";

// ===== 兼容性别名 =====

/// 为保持向后兼容性，提供一些简短的别名
/// 简历解析别名
pub const RESUME_PARSE: &str = DOCUMENT_AI_RESUME_PARSE;

/// 身份证识别别名
pub const ID_CARD: &str = DOCUMENT_AI_ID_CARD_RECOGNIZE;

/// Aily会话管理别名
pub const SESSIONS: &str = AILY_V1_SESSIONS;

/// 文本嵌入别名
pub const TEXT_EMBEDDING: &str = AI_EMBEDDING_V1_TEXT_EMBEDDING;

/// AI工作流别名
pub const WORKFLOWS: &str = AI_V1_WORKFLOWS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_ai_endpoints() {
        // 验证文档AI端点
        assert!(DOCUMENT_AI_RESUME_PARSE.starts_with("/open-apis/document_ai/v1/"));
        assert!(DOCUMENT_AI_ID_CARD_RECOGNIZE.contains("id_card"));
        assert!(DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE.contains("driving_license"));
        assert!(DOCUMENT_AI_INVOICE_RECOGNIZE.contains("invoice"));
    }

    #[test]
    fn test_ai_embedding_endpoints() {
        // 验证AI嵌入端点
        assert!(AI_EMBEDDING_V1_TEXT_EMBEDDING.starts_with("/open-apis/ai/v1/"));
        assert!(AI_EMBEDDING_V1_TEXT_EMBEDDING.contains("text_embedding"));
        assert!(AI_EMBEDDING_V1_IMAGE_EMBEDDING.contains("image_embedding"));
        assert!(AI_EMBEDDING_V1_SIMILARITY.contains("similarity"));
    }

    #[test]
    fn test_ai_workflow_endpoints() {
        // 验证AI工作流端点
        assert!(AI_V1_WORKFLOWS.starts_with("/open-apis/ai/v1/"));
        assert!(AI_V1_WORKFLOW_OPERATION.contains("{workflow_id}"));
        assert!(AI_V1_WORKFLOW_RUNS.contains("runs"));
        assert!(AI_V1_WORKFLOW_NODES.contains("nodes"));
    }

    #[test]
    fn test_aily_endpoints() {
        // 验证Aily端点
        assert!(AILY_V1_SESSIONS.starts_with("/open-apis/aily/v1/"));
        assert!(AILY_V1_SESSION_OPERATION.contains("{session_id}"));
        assert!(AILY_V1_DATA_KNOWLEDGE_ASK.contains("data_knowledge"));
        assert!(AILY_V1_SKILLS.contains("skills"));
    }

    #[test]
    fn test_backward_compatibility() {
        // 验证兼容性别名
        assert_eq!(RESUME_PARSE, DOCUMENT_AI_RESUME_PARSE);
        assert_eq!(ID_CARD, DOCUMENT_AI_ID_CARD_RECOGNIZE);
        assert_eq!(SESSIONS, AILY_V1_SESSIONS);
        assert_eq!(TEXT_EMBEDDING, AI_EMBEDDING_V1_TEXT_EMBEDDING);
        assert_eq!(WORKFLOWS, AI_V1_WORKFLOWS);
    }
}