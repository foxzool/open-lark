use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 转移云文档所有者
///
/// 将文件或文件夹的所有者转移给其他用户
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/transfer_owner
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 转移所有者请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOwnerRequest {
    /// 文件token
    pub token: String,
    /// 新所有者的用户ID
    pub to_user_id: String,
    /// 新所有者的用户类型
    pub to_user_type: String,
}

impl TransferOwnerRequest {
    /// 创建转移所有者请求
    ///
    /// # 参数
    /// * `token` - 文件token
    /// * `to_user_id` - 新所有者的用户ID
    /// * `to_user_type` - 新所有者的用户类型
    pub fn new(
        token: impl Into<String>,
        to_user_id: impl Into<String>,
        to_user_type: impl Into<String>,
    ) -> Self {
        Self {
            token: token.into(),
            to_user_id: to_user_id.into(),
            to_user_type: to_user_type.into(),
        }
    }
}

/// 转移所有者响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOwnerResponse {
    /// 转移结果
    pub data: Option<TransferResult>,
}

/// 转移结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    /// 文件token
    pub token: String,
    /// 旧所有者
    pub from_user_id: String,
    /// 新所有者
    pub to_user_id: String,
    /// 转移时间
    pub transfer_time: i64,
}

impl ApiResponseTrait for TransferOwnerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 转移云文档所有者
///
/// 将文件或文件夹的所有者转移给其他用户
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/permission-member/transfer_owner
pub async fn transfer_owner(
    request: TransferOwnerRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<TransferOwnerResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::TransferOwner(request.token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<TransferOwnerResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::json!({
            "to_user_id": request.to_user_id,
            "to_user_type": request.to_user_type
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
    fn test_transfer_owner_request_builder() {
        let request = TransferOwnerRequest::new(
            "file_token",
            "new_user_id",
            "user",
        );

        assert_eq!(request.token, "file_token");
        assert_eq!(request.to_user_id, "new_user_id");
        assert_eq!(request.to_user_type, "user");
    }

    #[test]
    fn test_transfer_result_structure() {
        let transfer_result = TransferResult {
            token: "file_token".to_string(),
            from_user_id: "old_user_id".to_string(),
            to_user_id: "new_user_id".to_string(),
            transfer_time: 1640995200,
        };

        assert_eq!(transfer_result.token, "file_token");
        assert_eq!(transfer_result.from_user_id, "old_user_id");
        assert_eq!(transfer_result.to_user_id, "new_user_id");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            TransferOwnerResponse::data_format(),
            ResponseFormat::Data
        );
    }
}