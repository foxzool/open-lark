use serde::{Deserialize, Serialize};

/// 分页响应基础结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    /// 数据项列表
    pub items: Vec<T>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// OKR 周期状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeriodStatus {
    /// 草稿状态
    Draft,
    /// 进行中
    Active,
    /// 已结束
    Ended,
    /// 已暂停
    Paused,
}

/// OKR 状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OkrStatus {
    /// 正常
    Normal,
    /// 已删除
    Deleted,
    /// 草稿
    Draft,
}

/// 进展记录类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProgressRecordType {
    /// 简单更新
    Simple,
    /// 详细更新
    Detail,
    /// 图片更新
    Image,
}

/// Key Result 类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyResultType {
    /// 数值型
    Numeric,
    /// 百分比型
    Percentage,
    /// 里程碑型
    Milestone,
}

/// 多语言文本
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct I18nText {
    /// 中文文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    /// 日文文本
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// OKR 周期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    /// 周期ID
    pub period_id: String,
    /// 周期名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 周期状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PeriodStatus>,
    /// 开始时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
}

/// OKR 周期规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodRule {
    /// 规则ID
    pub rule_id: String,
    /// 周期ID
    pub period_id: String,
    /// 规则类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<String>,
    /// 规则配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
}

/// Key Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyResult {
    /// Key Result ID
    pub kr_id: String,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<I18nText>,
    /// 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr_type: Option<KeyResultType>,
    /// 当前值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    /// 目标值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<f64>,
    /// 进度百分比
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<f64>,
    /// 完成状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
}

/// Objective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
    /// Objective ID
    pub objective_id: String,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<I18nText>,
    /// 进度百分比
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<f64>,
    /// Key Results 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_results: Option<Vec<KeyResult>>,
}

/// OKR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Okr {
    /// OKR ID
    pub okr_id: String,
    /// 用户ID
    pub user_id: String,
    /// 周期ID
    pub period_id: String,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OkrStatus>,
    /// Objectives 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectives: Option<Vec<Objective>>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
}

/// 进展记录附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressAttachment {
    /// 附件ID
    pub attachment_id: String,
    /// 附件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 附件URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 附件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// 进展记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressRecord {
    /// 进展记录ID
    pub progress_id: String,
    /// OKR ID
    pub okr_id: String,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 记录类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_type: Option<ProgressRecordType>,
    /// 进度百分比
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<f64>,
    /// 附件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<ProgressAttachment>>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<User>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<i64>,
}

/// OKR 复盘信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    /// 复盘ID
    pub review_id: String,
    /// OKR ID
    pub okr_id: String,
    /// 周期ID
    pub period_id: String,
    /// 复盘内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 复盘评分
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// 复盘者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<User>,
    /// 创建时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_period_status_enum() {
        assert_eq!(
            serde_json::to_string(&PeriodStatus::Draft).unwrap(),
            "\"draft\""
        );
        assert_eq!(
            serde_json::to_string(&PeriodStatus::Active).unwrap(),
            "\"active\""
        );
        assert_eq!(
            serde_json::to_string(&PeriodStatus::Ended).unwrap(),
            "\"ended\""
        );
        assert_eq!(
            serde_json::to_string(&PeriodStatus::Paused).unwrap(),
            "\"paused\""
        );
    }

    #[test]
    fn test_okr_status_enum() {
        assert_eq!(
            serde_json::to_string(&OkrStatus::Normal).unwrap(),
            "\"normal\""
        );
        assert_eq!(
            serde_json::to_string(&OkrStatus::Deleted).unwrap(),
            "\"deleted\""
        );
        assert_eq!(
            serde_json::to_string(&OkrStatus::Draft).unwrap(),
            "\"draft\""
        );
    }

    #[test]
    fn test_progress_record_type_enum() {
        assert_eq!(
            serde_json::to_string(&ProgressRecordType::Simple).unwrap(),
            "\"simple\""
        );
        assert_eq!(
            serde_json::to_string(&ProgressRecordType::Detail).unwrap(),
            "\"detail\""
        );
        assert_eq!(
            serde_json::to_string(&ProgressRecordType::Image).unwrap(),
            "\"image\""
        );
    }

    #[test]
    fn test_key_result_type_enum() {
        assert_eq!(
            serde_json::to_string(&KeyResultType::Numeric).unwrap(),
            "\"numeric\""
        );
        assert_eq!(
            serde_json::to_string(&KeyResultType::Percentage).unwrap(),
            "\"percentage\""
        );
        assert_eq!(
            serde_json::to_string(&KeyResultType::Milestone).unwrap(),
            "\"milestone\""
        );
    }

    #[test]
    fn test_i18n_text() {
        let text = I18nText {
            zh_cn: Some("中文内容".to_string()),
            en_us: Some("English content".to_string()),
            ja_jp: Some("日本語の内容".to_string()),
        };

        let json = serde_json::to_string(&text).unwrap();
        assert!(json.contains("中文内容"));
        assert!(json.contains("English content"));
        assert!(json.contains("日本語の内容"));
    }

    #[test]
    fn test_i18n_text_minimal() {
        let text = I18nText {
            zh_cn: Some("仅中文".to_string()),
            en_us: None,
            ja_jp: None,
        };

        let json = serde_json::to_string(&text).unwrap();
        assert!(json.contains("仅中文"));
        assert!(!json.contains("en_us"));
        assert!(!json.contains("ja_jp"));
    }

    #[test]
    fn test_user() {
        let name = I18nText {
            zh_cn: Some("张三".to_string()),
            en_us: Some("Zhang San".to_string()),
            ja_jp: None,
        };

        let user = User {
            user_id: "ou_user123".to_string(),
            name: Some(name),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("ou_user123"));
        assert!(json.contains("张三"));
        assert!(json.contains("Zhang San"));
        assert!(json.contains("https://example.com/avatar.jpg"));
    }

    #[test]
    fn test_user_minimal() {
        let user = User {
            user_id: "ou_minimal".to_string(),
            name: None,
            avatar: None,
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("ou_minimal"));
        assert!(!json.contains("name"));
        assert!(!json.contains("avatar"));
    }

    #[test]
    fn test_period() {
        let name = I18nText {
            zh_cn: Some("2024年第一季度".to_string()),
            en_us: Some("Q1 2024".to_string()),
            ja_jp: None,
        };

        let period = Period {
            period_id: "period123".to_string(),
            name: Some(name),
            status: Some(PeriodStatus::Active),
            start_time: Some(1704067200000), // 2024-01-01
            end_time: Some(1711900800000),   // 2024-03-31
            create_time: Some(1703980800000),
            modify_time: Some(1704067200000),
        };

        let json = serde_json::to_string(&period).unwrap();
        assert!(json.contains("period123"));
        assert!(json.contains("2024年第一季度"));
        assert!(json.contains("Q1 2024"));
        assert!(json.contains("\"active\""));
        assert!(json.contains("1704067200000"));
        assert!(json.contains("1711900800000"));
    }

    #[test]
    fn test_period_rule() {
        let config = serde_json::json!({
            "auto_close": true,
            "reminder_days": 7
        });

        let rule = PeriodRule {
            rule_id: "rule123".to_string(),
            period_id: "period123".to_string(),
            rule_type: Some("auto_close".to_string()),
            config: Some(config),
        };

        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("rule123"));
        assert!(json.contains("period123"));
        assert!(json.contains("auto_close"));
        assert!(json.contains("\"reminder_days\":7"));
    }

    #[test]
    fn test_key_result() {
        let content = I18nText {
            zh_cn: Some("提升用户活跃度到80%".to_string()),
            en_us: Some("Increase user activity to 80%".to_string()),
            ja_jp: None,
        };

        let kr = KeyResult {
            kr_id: "kr123".to_string(),
            content: Some(content),
            kr_type: Some(KeyResultType::Percentage),
            current_value: Some(65.5),
            target_value: Some(80.0),
            progress_rate: Some(81.875), // (65.5/80.0) * 100
            completed: Some(false),
        };

        let json = serde_json::to_string(&kr).unwrap();
        assert!(json.contains("kr123"));
        assert!(json.contains("提升用户活跃度到80%"));
        assert!(json.contains("Increase user activity to 80%"));
        assert!(json.contains("\"percentage\""));
        assert!(json.contains("65.5"));
        assert!(json.contains("80.0"));
        assert!(json.contains("81.875"));
        assert!(json.contains("false"));
    }

    #[test]
    fn test_key_result_milestone() {
        let content = I18nText {
            zh_cn: Some("完成产品MVP开发".to_string()),
            en_us: Some("Complete MVP development".to_string()),
            ja_jp: None,
        };

        let kr = KeyResult {
            kr_id: "kr456".to_string(),
            content: Some(content),
            kr_type: Some(KeyResultType::Milestone),
            current_value: None,
            target_value: None,
            progress_rate: Some(75.0),
            completed: Some(false),
        };

        let json = serde_json::to_string(&kr).unwrap();
        assert!(json.contains("kr456"));
        assert!(json.contains("完成产品MVP开发"));
        assert!(json.contains("\"milestone\""));
        assert!(json.contains("75.0"));
        assert!(!json.contains("current_value"));
        assert!(!json.contains("target_value"));
    }

    #[test]
    fn test_objective() {
        let obj_content = I18nText {
            zh_cn: Some("提升产品核心指标".to_string()),
            en_us: Some("Improve core product metrics".to_string()),
            ja_jp: None,
        };

        let kr_content = I18nText {
            zh_cn: Some("DAU增长到10万".to_string()),
            en_us: Some("Grow DAU to 100k".to_string()),
            ja_jp: None,
        };

        let kr = KeyResult {
            kr_id: "kr789".to_string(),
            content: Some(kr_content),
            kr_type: Some(KeyResultType::Numeric),
            current_value: Some(85000.0),
            target_value: Some(100000.0),
            progress_rate: Some(85.0),
            completed: Some(false),
        };

        let objective = Objective {
            objective_id: "obj123".to_string(),
            content: Some(obj_content),
            progress_rate: Some(85.0),
            key_results: Some(vec![kr]),
        };

        let json = serde_json::to_string(&objective).unwrap();
        assert!(json.contains("obj123"));
        assert!(json.contains("提升产品核心指标"));
        assert!(json.contains("Improve core product metrics"));
        assert!(json.contains("kr789"));
        assert!(json.contains("DAU增长到10万"));
        assert!(json.contains("\"numeric\""));
        assert!(json.contains("85000.0"));
        assert!(json.contains("100000.0"));
    }

    #[test]
    fn test_okr() {
        let obj_content = I18nText {
            zh_cn: Some("团队效率提升".to_string()),
            en_us: Some("Team efficiency improvement".to_string()),
            ja_jp: None,
        };

        let objective = Objective {
            objective_id: "obj456".to_string(),
            content: Some(obj_content),
            progress_rate: Some(60.0),
            key_results: Some(vec![]),
        };

        let okr = Okr {
            okr_id: "okr123".to_string(),
            user_id: "ou_user456".to_string(),
            period_id: "period456".to_string(),
            status: Some(OkrStatus::Normal),
            objectives: Some(vec![objective]),
            create_time: Some(1704067200000),
            modify_time: Some(1708646400000),
        };

        let json = serde_json::to_string(&okr).unwrap();
        assert!(json.contains("okr123"));
        assert!(json.contains("ou_user456"));
        assert!(json.contains("period456"));
        assert!(json.contains("\"normal\""));
        assert!(json.contains("obj456"));
        assert!(json.contains("团队效率提升"));
        assert!(json.contains("1704067200000"));
    }

    #[test]
    fn test_progress_attachment() {
        let attachment = ProgressAttachment {
            attachment_id: "att123".to_string(),
            name: Some("进度截图.png".to_string()),
            url: Some("https://example.com/progress.png".to_string()),
            file_type: Some("image/png".to_string()),
            size: Some(1024000),
        };

        let json = serde_json::to_string(&attachment).unwrap();
        assert!(json.contains("att123"));
        assert!(json.contains("进度截图.png"));
        assert!(json.contains("https://example.com/progress.png"));
        assert!(json.contains("image/png"));
        assert!(json.contains("1024000"));
    }

    #[test]
    fn test_progress_record() {
        let creator_name = I18nText {
            zh_cn: Some("李四".to_string()),
            en_us: Some("Li Si".to_string()),
            ja_jp: None,
        };

        let creator = User {
            user_id: "ou_creator123".to_string(),
            name: Some(creator_name),
            avatar: Some("https://example.com/creator.jpg".to_string()),
        };

        let attachment = ProgressAttachment {
            attachment_id: "att456".to_string(),
            name: Some("数据报表.xlsx".to_string()),
            url: Some("https://example.com/report.xlsx".to_string()),
            file_type: Some("application/xlsx".to_string()),
            size: Some(2048000),
        };

        let record = ProgressRecord {
            progress_id: "progress123".to_string(),
            okr_id: "okr456".to_string(),
            content: Some("本周完成了核心功能开发，用户反馈良好".to_string()),
            record_type: Some(ProgressRecordType::Detail),
            progress_rate: Some(75.0),
            attachments: Some(vec![attachment]),
            creator: Some(creator),
            create_time: Some(1708646400000),
            modify_time: Some(1708732800000),
        };

        let json = serde_json::to_string(&record).unwrap();
        assert!(json.contains("progress123"));
        assert!(json.contains("okr456"));
        assert!(json.contains("本周完成了核心功能开发，用户反馈良好"));
        assert!(json.contains("\"detail\""));
        assert!(json.contains("75.0"));
        assert!(json.contains("att456"));
        assert!(json.contains("数据报表.xlsx"));
        assert!(json.contains("ou_creator123"));
        assert!(json.contains("李四"));
    }

    #[test]
    fn test_review() {
        let reviewer_name = I18nText {
            zh_cn: Some("王五".to_string()),
            en_us: Some("Wang Wu".to_string()),
            ja_jp: None,
        };

        let reviewer = User {
            user_id: "ou_reviewer123".to_string(),
            name: Some(reviewer_name),
            avatar: None,
        };

        let review = Review {
            review_id: "review123".to_string(),
            okr_id: "okr789".to_string(),
            period_id: "period789".to_string(),
            content: Some(
                "本季度目标基本达成，团队协作有所提升，下季度需要重点关注效率优化".to_string(),
            ),
            score: Some(8.5),
            reviewer: Some(reviewer),
            create_time: Some(1711900800000),
        };

        let json = serde_json::to_string(&review).unwrap();
        assert!(json.contains("review123"));
        assert!(json.contains("okr789"));
        assert!(json.contains("period789"));
        assert!(json.contains("本季度目标基本达成，团队协作有所提升"));
        assert!(json.contains("8.5"));
        assert!(json.contains("ou_reviewer123"));
        assert!(json.contains("王五"));
        assert!(json.contains("1711900800000"));
    }

    #[test]
    fn test_page_response() {
        let kr = KeyResult {
            kr_id: "kr999".to_string(),
            content: None,
            kr_type: Some(KeyResultType::Numeric),
            current_value: Some(50.0),
            target_value: Some(100.0),
            progress_rate: Some(50.0),
            completed: Some(false),
        };

        let response: PageResponse<KeyResult> = PageResponse {
            items: vec![kr],
            page_token: Some("next_page_token".to_string()),
            has_more: Some(true),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("kr999"));
        assert!(json.contains("50.0"));
        assert!(json.contains("100.0"));
        assert!(json.contains("next_page_token"));
        assert!(json.contains("\"has_more\":true"));
    }

    #[test]
    fn test_minimal_structs() {
        let minimal_period = Period {
            period_id: "minimal_period".to_string(),
            name: None,
            status: None,
            start_time: None,
            end_time: None,
            create_time: None,
            modify_time: None,
        };

        let json = serde_json::to_string(&minimal_period).unwrap();
        assert!(json.contains("minimal_period"));
        assert!(!json.contains("name"));
        assert!(!json.contains("status"));
        assert!(!json.contains("start_time"));

        let minimal_okr = Okr {
            okr_id: "minimal_okr".to_string(),
            user_id: "minimal_user".to_string(),
            period_id: "minimal_period".to_string(),
            status: None,
            objectives: None,
            create_time: None,
            modify_time: None,
        };

        let okr_json = serde_json::to_string(&minimal_okr).unwrap();
        assert!(okr_json.contains("minimal_okr"));
        assert!(okr_json.contains("minimal_user"));
        assert!(okr_json.contains("minimal_period"));
        assert!(!okr_json.contains("status"));
        assert!(!okr_json.contains("objectives"));
    }
}
