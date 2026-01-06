//! 延时更新消息卡片
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message-card/delay-update-message-card

use openlark_core::{api::ApiRequest, config::Config, error, http::Transport, SDKResult};

use crate::common::api_utils::serialize_params;

const INTERACTIVE_V1_CARD_UPDATE: &str = "/open-apis/interactive/v1/card/update";

/// 延时更新消息卡片请求
pub struct DelayUpdateMessageCardRequest {
    config: Config,
}

impl DelayUpdateMessageCardRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口响应体仅包含 code/msg（不含 data 字段），因此此处仅以 code==0 判断成功。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message-card/delay-update-message-card
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<()> {
        // url: POST:/open-apis/interactive/v1/card/update
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(INTERACTIVE_V1_CARD_UPDATE)
            .body(serialize_params(&body, "延时更新消息卡片")?);

        let resp: openlark_core::api::Response<serde_json::Value> =
            Transport::request(req, &self.config, None).await?;
        if resp.is_success() {
            Ok(())
        } else {
            Err(error::validation_error(
                format!("延时更新消息卡片失败: {}", resp.code()),
                resp.msg().to_string(),
            ))
        }
    }
}
