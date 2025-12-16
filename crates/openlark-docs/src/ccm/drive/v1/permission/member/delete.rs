use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 移除云文档协作者权限
///
/// 移除文件或文件夹中指定协作者的权限
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/delete
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 移除协作者权限请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionMemberRequest {
    /// 文件token
    pub token: String,
    /// 成员ID
    pub member_id: String,
}

impl DeletePermissionMemberRequest {
    /// 创建移除协作者权限请求
    ///
    /// # 参数
    /// * `token` - 文件token
    /// * `member_id` - 成员ID
    pub fn new(token: impl Into<String>, member_id: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            member_id: member_id.into(),
        }
    }
}

/// 移除协作者权限响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePermissionMemberResponse {
    /// 操作结果
    pub data: Option<DeleteResult>,
}

/// 删除结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResult {
    /// 是否成功
    pub success: bool,
    /// 成员ID
    pub member_id: String,
}

impl ApiResponseTrait for DeletePermissionMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移除云文档协作者权限
///
/// 移除文件或文件夹中指定协作者的权限
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/delete
pub async fn delete_permission_member(
    request: DeletePermissionMemberRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<DeletePermissionMemberResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::DeletePermissionMember(request.token.clone(), request.member_id.clone());

    // 创建API请求
    let mut api_request: ApiRequest<DeletePermissionMemberResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

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
    fn test_delete_permission_member_request_builder() {
        let request = DeletePermissionMemberRequest::new("file_token", "member_id");

        assert_eq!(request.token, "file_token");
        assert_eq!(request.member_id, "member_id");
    }

    #[test]
    fn test_delete_result_structure() {
        let delete_result = DeleteResult {
            success: true,
            member_id: "member_id".to_string(),
        };

        assert!(delete_result.success);
        assert_eq!(delete_result.member_id, "member_id");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            DeletePermissionMemberResponse::data_format(),
            ResponseFormat::Data
        );
    }
}