use open_lark_core::core::{
use serde_json;    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config, constants::AccessTokenType,
    endpoints::EndpointBuilder, http::Transport,
},
use serde::{Deserialize, Serialize },

/// 角色成员服务
pub struct FunctionalRoleMemberService {
    config: Config,
}
impl FunctionalRoleMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 添加角色成员
    pub async fn create(
        &self,
        role_id: &str,
        req: &CreateRoleMemberRequest,
