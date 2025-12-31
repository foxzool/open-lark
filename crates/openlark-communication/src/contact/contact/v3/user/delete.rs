//! 删除用户
//!
//! docPath: https://open.feishu.cn/document/server-docs/contact-v3/user/delete

use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    contact::contact::v3::user::models::UserIdType,
    endpoints::CONTACT_V3_USERS,
};

/// 删除用户请求
pub struct DeleteUserRequest {
    config: Config,
    user_id: String,
    user_id_type: Option<UserIdType>,
}

impl DeleteUserRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id: String::new(),
            user_id_type: None,
        }
    }

    /// 用户 ID（路径参数）
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/contact-v3/user/delete
    pub async fn execute(self) -> SDKResult<EmptyData> {
        validate_required!(self.user_id, "user_id 不能为空");

        // url: DELETE:/open-apis/contact/v3/users/:user_id
        let mut req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", CONTACT_V3_USERS, self.user_id));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "删除用户")
    }
}
