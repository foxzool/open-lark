/// 文件订阅（subscription）相关模型
///
/// 注意：该文件仅存放模型结构，不计入 API 文件数量。
use openlark_core::api::ApiResponseTrait;
use serde::{Deserialize, Serialize};

/// 订阅信息（get/create/patch 的 data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Subscription {
    pub subscription_id: String,
    pub subscription_type: String,
    /// 文档响应字段拼写为 is_subcribe（注意不是 is_subscribe）
    #[serde(rename = "is_subcribe")]
    pub is_subscribe: bool,
    pub file_type: String,
}

impl ApiResponseTrait for Subscription {}

