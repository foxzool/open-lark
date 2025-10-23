use open_lark_core::core::{
use serde_json;    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
},
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 单位管理服务
pub struct UnitService {
    config: Config,
}
impl UnitService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 创建单位
    pub async fn create(
        &self,
