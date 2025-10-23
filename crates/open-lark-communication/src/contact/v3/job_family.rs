use open_lark_core::core::{
    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config, constants::AccessTokenType, http::Transport,
},
use serde::{Deserialize, Serialize },
use serde_json;
use crate::contact::models::JobFamily;

/// 序列管理服务
#[derive(Debug)]
pub struct JobFamilyService {
    config: Config,
}
impl JobFamilyService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 创建序列
    pub async fn create(
        &self,
        req: &CreateJobFamilyRequest,
