//! Hire 业务域通用响应模型。
//!
//! 用于承接第一批 typed response 收敛中反复出现的 i18n、分页、引用对象、
//! 金额与附件元数据等结构，避免在每个 API 文件里重复定义。

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 国际化文本。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct I18nText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 常见的 ID + 名称引用对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct IdNameObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 带 code + name 的区域对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CodeNameObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 通用分页响应壳。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PaginatedResponse<T> {
    #[serde(default)]
    pub items: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 奖励金额。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BonusAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_bonus: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_bonus: Option<Vec<CashAmount>>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 现金奖励金额。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CashAmount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 附件元数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AttachmentMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}

/// 分数信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ScoreInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_score: Option<f64>,
    #[serde(default, flatten)]
    pub extra: HashMap<String, Value>,
}
