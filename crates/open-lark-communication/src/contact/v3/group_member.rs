use open_lark_core::core::{
    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config, constants::AccessTokenType, http::Transport,
},
use serde::{Deserialize, Serialize },
use serde_json;

// Import shared types from user module
use super::user::{GroupMember, GroupMemberInfo, GroupMemberResult};
/// 用户组成员服务
#[derive(Debug)]
pub struct GroupMemberService {
    config: Config,
}
impl GroupMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 添加用户组成员
    pub async fn add(
        &self,
        group_id: &str,
        req: &AddGroupMemberRequest,
