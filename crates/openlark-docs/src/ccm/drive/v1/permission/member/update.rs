use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 更新协作者权限
///
/// 更新文件或文件夹中指定协作者的权限
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/update
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 更新协作者权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionMemberRequest {
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
    /// * `token` - 文件token
    /// * `member_id` - 成员ID
    /// * `type` - 权限类型
    pub fn new(
        token: impl Into<String>,
        member_id: impl Into<String>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            token: token.into(),
            member_id: member_id.into(),
            r#type: r#type.into(),
        }
    }
}

/// 更新协作者权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePermissionMemberResponse {
    /// 更新后的协作者信息
    pub data: Option<PermissionMemberData>,
}

/// 权限成员数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionMemberData {
    /// 成员ID
    pub member_id: String,
    /// 用户ID
    pub user_id: String,
    /// 权限类型
    pub r#type: String,
    /// 更新时间
    pub update_time: i64,
}

impl ApiResponseTrait for UpdatePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新协作者权限
///
/// 更新文件或文件夹中指定协作者的权限
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/update
pub async fn update_permission_member(
    request: UpdatePermissionMemberRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<UpdatePermissionMemberResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::UpdatePermissionMember(request.token.clone(), request.member_id.clone());

    // 创建API请求
    let mut api_request: ApiRequest<UpdatePermissionMemberResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serde_json::json!({
            "type": request.r#type
        }));

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_permission_member_request_builder() {
        let request = UpdatePermissionMemberRequest::new(
            "file_token",
            "member_id",
            "editor",
        );

        assert_eq!(request.token, "file_token");
        assert_eq!(request.member_id, "member_id");
        assert_eq!(request.r#type, "editor");
    }

    #[test]
    fn test_permission_member_data_structure() {
        let permission_data = PermissionMemberData {
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