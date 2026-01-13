//! 转换 ID
//!
//! docPath: https://open.feishu.cn/document/historic-version/id_convert

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CARDKIT_V1_CARD_ID_CONVERT,
};

/// 转换 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertCardIdBody {
    /// 源 ID 类型
    pub source_id_type: String,
    /// 目标 ID 类型
    pub target_id_type: String,
    /// 卡片 ID 列表
    pub card_ids: Vec<String>,
}

/// 转换 ID 请求
pub struct ConvertCardIdRequest {
    config: Config,
}

impl ConvertCardIdRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/historic-version/id_convert
    pub async fn execute(self, body: ConvertCardIdBody) -> SDKResult<serde_json::Value> {
        if body.source_id_type.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "source_id_type 不能为空",
                "source_id_type 不能为空",
            ));
        }
        if body.target_id_type.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "target_id_type 不能为空",
                "target_id_type 不能为空",
            ));
        }
        if body.card_ids.is_empty() {
            return Err(openlark_core::error::validation_error(
                "card_ids 不能为空",
                "card_ids 不能为空",
            ));
        }

        // url: POST:/open-apis/cardkit/v1/cards/id_convert
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(CARDKIT_V1_CARD_ID_CONVERT).body(serialize_params(&body, "转换 ID")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "转换 ID")
    }
}
