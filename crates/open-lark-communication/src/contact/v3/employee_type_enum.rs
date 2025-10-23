use open_lark_core::core::{
use serde_json;    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
},
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 人员类型服务
pub struct EmployeeTypeEnumService {
    config: Config,
}
impl EmployeeTypeEnumService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 新增人员类型
    pub async fn create(
        &self,
