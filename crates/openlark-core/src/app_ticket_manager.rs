//! 应用票据（app_ticket）相关逻辑
//!
//! 仅保留在请求遇到 app_ticket 失效时触发的“重新推送”能力。

use serde::{Deserialize, Serialize};

use crate::{config::Config, constants::APPLY_APP_TICKET_PATH, SDKResult};

/// 触发“重新推送 app_ticket”
///
/// 说明：该接口是一个副作用操作，且不返回业务数据。
pub async fn apply_app_ticket(config: &Config) -> SDKResult<()> {
    let url = format!("{}{}", config.base_url, APPLY_APP_TICKET_PATH);

    let body = ResendAppTicketReq {
        app_id: config.app_id.clone(),
        app_secret: config.app_secret.clone(),
    };

    let _response = config.http_client.post(&url).json(&body).send().await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct ResendAppTicketReq {
    app_id: String,
    app_secret: String,
}
