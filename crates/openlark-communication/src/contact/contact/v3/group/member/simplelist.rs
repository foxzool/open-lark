//! 查询用户组成员列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/group-member/simplelist

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::group::member::models::SimpleListGroupMembersResponse,
    endpoints::CONTACT_V3_GROUP,
};

/// 查询用户组成员列表请求
pub struct SimpleListGroupMembersRequest {
    config: Config,
    group_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
    member_id_type: Option<String>,
    member_type: Option<String>,
}

impl SimpleListGroupMembersRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            group_id: String::new(),
            page_size: None,
            page_token: None,
            member_id_type: None,
            member_type: None,
        }
    }

    /// 用户组 ID（路径参数）
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = group_id.into();
        self
    }

    /// 分页大小（查询参数，可选，默认 50，最大 100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 成员 ID 类型（查询参数，可选）
    pub fn member_id_type(mut self, member_id_type: impl Into<String>) -> Self {
        self.member_id_type = Some(member_id_type.into());
        self
    }

    /// 成员类型（查询参数，可选）
    pub fn member_type(mut self, member_type: impl Into<String>) -> Self {
        self.member_type = Some(member_type.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/group-member/simplelist
    pub async fn execute(self) -> SDKResult<SimpleListGroupMembersResponse> {
        validate_required!(self.group_id, "group_id 不能为空");

        // url: GET:/open-apis/contact/v3/group/:group_id/member/simplelist
        let mut req: ApiRequest<SimpleListGroupMembersResponse> = ApiRequest::get(format!(
            "{}/{}/member/simplelist",
            CONTACT_V3_GROUP, self.group_id
        ));

        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(member_id_type) = self.member_id_type {
            req = req.query("member_id_type", member_id_type);
        }
        if let Some(member_type) = self.member_type {
            req = req.query("member_type", member_type);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询用户组成员列表")
    }
}
