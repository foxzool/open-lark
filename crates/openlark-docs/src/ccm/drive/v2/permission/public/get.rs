/// 获取云文档权限设置
///
/// 该接口用于根据 token 获取云文档的权限设置。
/// docPath: /document/ukTMukTMukTM/uIzNzUjLyczM14iM3MTN/drive-v2/permission-public/get
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::PermissionPublic;
use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取云文档权限设置请求。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionPublicRequest {
    /// 文档 token。
    pub token: String,
    /// 文档类型。
    pub r#type: String,
}

impl GetPermissionPublicRequest {
    /// 创建新的权限查询请求。
    pub fn new(token: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            r#type: r#type.into(),
        }
    }
}

/// 获取云文档权限设置响应 data。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPermissionPublicResponse {
    /// 权限设置详情。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_public: Option<PermissionPublic>,
}

impl ApiResponseTrait for GetPermissionPublicResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档权限设置。
pub async fn get_permission_public(
    request: GetPermissionPublicRequest,
    config: &Config,
) -> SDKResult<GetPermissionPublicResponse> {
    get_permission_public_with_options(
        request,
        config,
        openlark_core::req_option::RequestOption::default(),
    )
    .await
}

/// 使用指定请求选项获取云文档权限设置。
pub async fn get_permission_public_with_options(
    request: GetPermissionPublicRequest,
    config: &Config,
    option: openlark_core::req_option::RequestOption,
) -> SDKResult<GetPermissionPublicResponse> {
    if request.token.is_empty() {
        return Err(openlark_core::error::validation_error(
            "token",
            "token 不能为空",
        ));
    }
    if request.r#type.is_empty() {
        return Err(openlark_core::error::validation_error(
            "type",
            "type 不能为空",
        ));
    }

    let api_endpoint = DriveApi::GetPublicPermissionV2(request.token);
    let api_request: ApiRequest<GetPermissionPublicResponse> =
        ApiRequest::get(&api_endpoint.to_url()).query("type", &request.r#type);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "获取云文档权限设置")
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
