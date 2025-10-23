use open_lark_core::core::{
use serde_json;    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
    constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
},
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 用户管理服务
pub struct UserService {
    config: Config,
}
impl UserService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 获取配置引用
