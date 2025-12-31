//! 批量移除用户组成员
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/group-member/batch_remove

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    common::models::EmptyData,
    endpoints::CONTACT_V3_GROUP,
};

/// 批量移除成员信息（请求体 members[] 元素）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberListItem {
    pub member_id: String,
    pub member_id_type: String,
    pub member_type: String,
}

/// 批量移除用户组成员请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRemoveGroupMembersBody {
    pub members: Vec<MemberListItem>,
}

/// 批量移除用户组成员请求
pub struct BatchRemoveGroupMembersRequest {
    config: Config,
    group_id: String,
}

impl BatchRemoveGroupMembersRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/group-member/batch_remove
    pub async fn execute(self, body: BatchRemoveGroupMembersBody) -> SDKResult<EmptyData> {
        validate_required!(self.group_id, "group_id 不能为空");
        if body.members.is_empty() {
            return Err(openlark_core::error::validation_error(
                "members 不能为空".to_string(),
                "members 至少需要包含 1 个成员".to_string(),
            ));
        }

        // url: POST:/open-apis/contact/v3/group/:group_id/member/batch_remove
        let req: ApiRequest<EmptyData> = ApiRequest::post(format!(
            "{}/{}/member/batch_remove",
            CONTACT_V3_GROUP, self.group_id
        ))
        .body(serialize_params(&body, "批量移除用户组成员")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "批量移除用户组成员")
    }
}
