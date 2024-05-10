use serde::{Deserialize, Serialize};

use crate::core::model::BaseResponseTrait;

/// access_token 响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    /// 错误码，非 0 取值表示失败
    pub code: i32,
    /// 错误描述
    pub msg: String,
    /// 租户访问凭证
    pub tenant_access_token: String,
    /// tenant_access_token 的过期时间，单位为秒
    pub expire: i32,
}

impl BaseResponseTrait for AccessTokenResponse {
    fn code(&self) -> u16 {
        self.code as u16
    }

    fn msg(&self) -> &str {
        self.msg.as_str()
    }
}
