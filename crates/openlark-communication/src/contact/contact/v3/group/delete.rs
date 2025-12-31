//! 删除用户组
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/group/delete

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::CONTACT_V3_GROUP,
};

/// 删除用户组请求
pub struct DeleteGroupRequest {
    config: Config,
    group_id: String,
}

impl DeleteGroupRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/group/delete
    pub async fn execute(self) -> SDKResult<EmptyData> {
        validate_required!(self.group_id, "group_id 不能为空");

        // url: DELETE:/open-apis/contact/v3/group/:group_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", CONTACT_V3_GROUP, self.group_id));
        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除用户组")
    }
}

