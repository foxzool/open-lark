//! 添加用户组成员
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/group-member/add

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    common::models::EmptyData,
    contact::contact::v3::user::models::UserIdType,
    endpoints::CONTACT_V3_GROUP,
};

/// 添加用户组成员请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddGroupMemberBody {
    /// 用户组成员类型（目前仅支持 user）
    pub member_type: String,
    /// 用户 ID 类型
    pub member_id_type: String,
    /// 用户 ID
    pub member_id: String,
}

/// 添加用户组成员请求
pub struct AddGroupMemberRequest {
    config: Config,
    group_id: String,
}

impl AddGroupMemberRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            group_id: String::new(),
        }
    }

    /// 用户组 ID（路径参数）
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = group_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口目前仅支持 `member_type=user`。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/group-member/add
    pub async fn execute(
        self,
        member_id_type: UserIdType,
        member_id: impl Into<String>,
    ) -> SDKResult<EmptyData> {
        validate_required!(self.group_id, "group_id 不能为空");
        let member_id = member_id.into();
        validate_required!(member_id, "member_id 不能为空");

        // url: POST:/open-apis/contact/v3/group/:group_id/member/add
        let body = AddGroupMemberBody {
            member_type: "user".to_string(),
            member_id_type: member_id_type.as_str().to_string(),
            member_id,
        };
        let req: ApiRequest<EmptyData> =
            ApiRequest::post(format!("{}/{}/member/add", CONTACT_V3_GROUP, self.group_id))
                .body(serialize_params(&body, "添加用户组成员")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "添加用户组成员")
    }
}
