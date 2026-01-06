//! 更新用户 ID
//!
//! docPath: https://open.feishu.cn/document/contact-v3/user/update_user_id

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    common::models::EmptyData,
    contact::contact::v3::user::models::UserIdType,
    endpoints::CONTACT_V3_USERS,
};

/// 更新用户 ID 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserIdBody {
    pub new_user_id: String,
}

/// 更新用户 ID 请求
pub struct UpdateUserIdRequest {
    config: Config,
    user_id: String,
    user_id_type: Option<UserIdType>,
}

impl UpdateUserIdRequest {
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
    /// docPath: https://open.feishu.cn/document/contact-v3/user/update_user_id
    pub async fn execute(self, body: UpdateUserIdBody) -> SDKResult<EmptyData> {
        validate_required!(self.user_id, "user_id 不能为空");
        validate_required!(body.new_user_id, "new_user_id 不能为空");

        // url: PATCH:/open-apis/contact/v3/users/:user_id/update_user_id
        let mut req: ApiRequest<EmptyData> = ApiRequest::patch(format!(
            "{}/{}/update_user_id",
            CONTACT_V3_USERS, self.user_id
        ))
        .body(serialize_params(&body, "更新用户 ID")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新用户 ID")
    }
}
