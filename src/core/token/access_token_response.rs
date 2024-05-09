use serde::Deserialize;
use crate::core::model::BaseResponseTrait;

/// access_token 响应体
#[derive(Debug, Deserialize)]
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
    fn success(&self) -> bool {
        self.code == 0
    }

   
}