use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
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
    #[serde(skip)]
    config: Config,
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
    /// * `config` - 配置
    /// * `token` - 文件token
    /// * `to_user_id` - 新所有者的用户ID
    /// * `to_user_type` - 新所有者的用户类型
    pub fn new(
        config: Config,
        token: impl Into<String>,
        to_user_id: impl Into<String>,
        to_user_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            token: token.into(),
            to_user_id: to_user_id.into(),
            to_user_type: to_user_type.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<Response<TransferOwnerResponse>> {
        let api_endpoint = DriveApi::TransferOwner(self.token.clone());

        let api_request = ApiRequest::<TransferOwnerResponse>::post(&api_endpoint.to_url())
            .body(serde_json::json!({
                "to_user_id": self.to_user_id,
                "to_user_type": self.to_user_type
            }));

        Transport::request(api_request, &self.config, None).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_owner_request_builder() {
        let config = Config::default();
        let request = TransferOwnerRequest::new(
            config,
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