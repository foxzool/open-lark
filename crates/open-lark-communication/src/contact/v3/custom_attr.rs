use open_lark_core::core::{
    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
},
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 自定义用户字段服务
pub struct CustomAttrService {
    config: Config,
}
impl CustomAttrService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 获取企业自定义用户字段
    pub async fn list(
        &self,
