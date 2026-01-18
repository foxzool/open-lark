//! AILY (AI学习平台) 端点定义
//!
//! AILY 是飞书的 AI 学习平台，提供数据知识管理、AI 会话和技能调用等功能。
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_communication::endpoints::aily::*;
//!
//! let sessions_endpoint = AILY_V1_SESSIONS;
//! let skills_endpoint = AILY_V1_SKILLS;
//! ```

// ==================== AILY (AI学习平台) v1 ====================
// AILY AI学习平台 - 数据知识管理、AI会话和技能调用

/// AILY 会话管理 v1
pub const AILY_V1_SESSIONS: &str = "/open-apis/aily/v1/sessions";
pub const AILY_V1_SESSION: &str = "/open-apis/aily/v1/sessions/{session_id}";

/// AILY 消息管理 v1
pub const AILY_V1_MESSAGES: &str = "/open-apis/aily/v1/sessions/{session_id}/messages";

/// AILY 运行管理 v1
pub const AILY_V1_RUNS: &str = "/open-apis/aily/v1/sessions/{session_id}/runs";
pub const AILY_V1_RUN: &str = "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}";
pub const AILY_V1_RUN_CANCEL: &str =
    "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}/cancel";

/// AILY 数据资产管理 v1
pub const AILY_V1_DATA_ASSETS: &str = "/open-apis/aily/v1/apps/{app_id}/data_assets";
pub const AILY_V1_DATA_ASSET: &str = "/open-apis/aily/v1/apps/{app_id}/data_assets/{data_asset_id}";
pub const AILY_V1_DATA_ASSET_TAGS: &str = "/open-apis/aily/v1/apps/{app_id}/data_asset_tags";
pub const AILY_V1_UPLOAD_FILE: &str = "/open-apis/aily/v1/apps/{app_id}/data_assets/upload_file";

/// AILY 知识问答 v1
pub const AILY_V1_KNOWLEDGE_ASK: &str = "/open-apis/aily/v1/apps/{app_id}/knowledge/ask";

/// AILY 技能管理 v1
pub const AILY_V1_SKILLS: &str = "/open-apis/aily/v1/apps/{app_id}/skills";
pub const AILY_V1_SKILL: &str = "/open-apis/aily/v1/apps/{app_id}/skills/{skill_id}";
pub const AILY_V1_SKILL_START: &str = "/open-apis/aily/v1/apps/{app_id}/skills/{skill_id}/start";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aily_endpoints() {
        assert!(AILY_V1_SESSIONS.starts_with("/open-apis/aily/v1/"));
        assert!(AILY_V1_SESSION.contains("{session_id}"));
        assert!(AILY_V1_MESSAGES.contains("{session_id}"));
        assert!(AILY_V1_RUNS.contains("{session_id}"));
        assert!(AILY_V1_RUN.contains("{run_id}"));
        assert!(AILY_V1_DATA_ASSETS.contains("{app_id}"));
        assert!(AILY_V1_SKILLS.contains("{app_id}"));
    }
}
