use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
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
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub token: String,
    /// 成员ID
    pub member_id: String,
}

impl DeletePermissionMemberRequest {
    /// 创建移除协作者权限请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `member_id` - 成员ID
    pub fn new(config: Config, token: impl Into<String>, member_id: impl Into<String>) -> Self {
        Self {
            config,
            token: token.into(),
            member_id: member_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<Response<DeletePermissionMemberResponse>> {
        let api_endpoint = DriveApi::DeletePermissionMember(self.token.clone(), self.member_id.clone());

        let api_request = ApiRequest::<DeletePermissionMemberResponse>::delete(&api_endpoint.to_url());

        Transport::request(api_request, &self.config, None).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_permission_member_request_builder() {
        let config = Config::default();
        let request = DeletePermissionMemberRequest::new(config, "file_token", "member_id");

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