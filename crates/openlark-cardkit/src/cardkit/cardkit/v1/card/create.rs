//! 创建卡片实体
//!
//! docPath: https://open.feishu.cn/document/cardkit-v1/card/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::CARDKIT_V1_CARDS,
};

/// 创建卡片实体请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardBody {
    /// 卡片内容
    pub card_content: serde_json::Value,
    /// 卡片类型（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// 模板ID（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 临时卡片标记（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp: Option<bool>,
    /// 临时卡片过期时间（可选，秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_expire_time: Option<i32>,
}

impl CreateCardBody {
    pub fn validate(&self) -> Result<(), String> {
        if self.card_content.is_null() {
            return Err("card_content 不能为空".to_string());
        }
        if !self.card_content.is_object() {
            return Err("card_content 必须是 JSON 对象".to_string());
        }

        if let Some(temp_expire_time) = self.temp_expire_time {
            if temp_expire_time <= 0 || temp_expire_time > 86_400 {
                return Err("temp_expire_time 取值范围为 1~86400（秒）".to_string());
            }
        }

        Ok(())
    }
}

/// 创建卡片实体响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCardResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl openlark_core::api::ApiResponseTrait for CreateCardResponse {}

/// 创建卡片实体请求
pub struct CreateCardRequest {
    config: Config,
}

impl CreateCardRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/cardkit-v1/card/create
    pub async fn execute(self, body: CreateCardBody) -> SDKResult<CreateCardResponse> {
        body.validate()
            .map_err(|reason| openlark_core::error::validation_error("请求参数非法", reason))?;

        // url: POST:/open-apis/cardkit/v1/cards
        let req: ApiRequest<CreateCardResponse> =
            ApiRequest::post(CARDKIT_V1_CARDS).body(serialize_params(&body, "创建卡片实体")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建卡片实体")
    }
}
