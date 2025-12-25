use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 更新协作者权限
///
/// 更新文件或文件夹中指定协作者的权限
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/permission-member/update
/// doc: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/update
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 更新协作者权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionMemberRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub token: String,
    /// 成员ID
    pub member_id: String,
    /// 权限类型
    pub r#type: String,
}

impl UpdatePermissionMemberRequest {
    /// 创建更新协作者权限请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `member_id` - 成员ID
    /// * `type` - 权限类型
    pub fn new(
        config: Config,
        token: impl Into<String>,
        member_id: impl Into<String>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            token: token.into(),
            member_id: member_id.into(),
            r#type: r#type.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UpdatePermissionMemberResponse> {
        if self.token.is_empty() {
            return Err(openlark_core::error::validation_error("token", "token 不能为空"));
        }
        if self.member_id.is_empty() {
            return Err(openlark_core::error::validation_error(
                "member_id",
                "member_id 不能为空",
            ));
        }
        if self.r#type.is_empty() {
            return Err(openlark_core::error::validation_error("type", "type 不能为空"));
        }

        let api_endpoint =
            DriveApi::UpdatePermissionMember(self.token.clone(), self.member_id.clone());

        #[derive(Serialize)]
        struct UpdatePermissionMemberBody {
            #[serde(rename = "type")]
            r#type: String,
        }

        let api_request: ApiRequest<UpdatePermissionMemberResponse> =
            ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(
                &UpdatePermissionMemberBody { r#type: self.r#type },
                "更新协作者权限",
            )?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "更新协作者权限")
    }
}

/// 更新协作者权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionMemberResponse {
    /// 成员ID
    pub member_id: String,
    /// 用户ID
    pub user_id: String,
    /// 权限类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 更新时间
    pub update_time: i64,
}

impl ApiResponseTrait for UpdatePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_permission_member_request_builder() {
        let config = Config::default();
        let request =
            UpdatePermissionMemberRequest::new(config, "file_token", "member_id", "editor");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.member_id, "member_id");
        assert_eq!(request.r#type, "editor");
    }

    #[test]
    fn test_permission_member_data_structure() {
        let permission_data = UpdatePermissionMemberResponse {
            member_id: "member_id".to_string(),
            user_id: "user_id".to_string(),
            r#type: "editor".to_string(),
            update_time: 1640995200,
        };

        assert_eq!(permission_data.member_id, "member_id");
        assert_eq!(permission_data.user_id, "user_id");
        assert_eq!(permission_data.r#type, "editor");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            UpdatePermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
