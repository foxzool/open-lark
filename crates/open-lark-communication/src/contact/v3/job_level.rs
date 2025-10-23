use open_lark_core::core::{
    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config, constants::AccessTokenType, http::Transport,
},
use serde::{Deserialize, Serialize },
use serde_json;
use crate::contact::models::JobLevel;

/// 职级管理服务
pub struct JobLevelService {
    config: Config,
}
impl JobLevelService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 创建职级
    pub async fn create(
        &self,
