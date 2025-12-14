use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 添加协作者
///
/// 为文件或文件夹添加协作者权限
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/create
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 添加协作者请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionMemberRequest {
    /// 文件token
    pub token: String,
    /// 成员信息
    pub member: MemberInfo,
    /// 权限类型
    pub r#type: String,
}

/// 成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户类型
    pub user_type: String,
}

impl CreatePermissionMemberRequest {
    /// 创建添加协作者请求
    ///
    /// # 参数
    /// * `token` - 文件token
    /// * `member` - 成员信息
    /// * `type` - 权限类型
    pub fn new(
        token: impl Into<String>,
        member: MemberInfo,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            token: token.into(),
            member,
            r#type: r#type.into(),
        }
    }
}

/// 协作者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePermissionMemberResponse {
    /// 协作者信息
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
    /// 创建时间
    pub create_time: i64,
}

impl ApiResponseTrait for CreatePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加协作者
///
/// 为文件或文件夹添加协作者权限
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/create
pub async fn create_permission_member(
    request: CreatePermissionMemberRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreatePermissionMemberResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::CreatePermissionMember(request.token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<CreatePermissionMemberResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::json!({
            "member": request.member,
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
    fn test_create_permission_member_request_builder() {
        let member = MemberInfo {
            user_id: "user_id".to_string(),
            user_type: "user".to_string(),
        };

        let request = CreatePermissionMemberRequest::new(
            "file_token",
            member,
            "admin",
        );

        assert_eq!(request.token, "file_token");
        assert_eq!(request.r#type, "admin");
    }

    #[test]
    fn test_member_info_structure() {
        let member = MemberInfo {
            user_id: "user_id".to_string(),
            user_type: "user".to_string(),
        };

        assert_eq!(member.user_id, "user_id");
        assert_eq!(member.user_type, "user");
    }

    #[test]
    fn test_permission_member_data_structure() {
        let permission_data = PermissionMemberData {
            member_id: "member_id".to_string(),
            user_id: "user_id".to_string(),
            r#type: "admin".to_string(),
            create_time: 1640995200,
        };

        assert_eq!(permission_data.member_id, "member_id");
        assert_eq!(permission_data.user_id, "user_id");
        assert_eq!(permission_data.r#type, "admin");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreatePermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }
}