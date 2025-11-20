//! Sheets v2 保护范围管理服务
//!
//! 提供飞书电子表格v2版本的保护范围管理功能，支持：
//! - 批量创建保护范围
//! - 批量修改保护范围
//! - 批量获取保护范围信息
//! - 批量删除保护范围
//! - 权限验证和安全机制

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 保护范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtectedRange {
    /// 保护范围ID
    pub range_id: Option<String>,
    /// 保护范围（如 "Sheet1!A1:C10"）
    pub range: String,
    /// 描述信息
    pub description: Option<String>,
    /// 保护的工作表ID
    pub protected_sheet_id: Option<String>,
    /// 仅警告模式（用户编辑时显示警告但允许编辑）
    pub warning_only: Option<bool>,
    /// 保护范围的用户邮箱列表
    pub email_addresses: Option<Vec<String>>,
}

/// 创建保护范围请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateProtectedRangesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 保护范围列表
    pub protected_ranges: Vec<ProtectedRange>,
}

/// 创建保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProtectedRangesResponse {
    /// 响应数据
    pub data: CreateProtectedRangesData,
}

/// 创建保护范围数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProtectedRangesData {
    /// 创建结果列表
    pub protected_ranges: Vec<ProtectedRangeResult>,
}

/// 保护范围操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedRangeResult {
    /// 保护范围ID
    pub range_id: String,
    /// 请求ID
    pub request_id: String,
}

impl Default for CreateProtectedRangesResponse {
    fn default() -> Self {
        Self {
            data: CreateProtectedRangesData {
                protected_ranges: vec![],
            },
        }
    }
}

impl ApiResponseTrait for CreateProtectedRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新保护范围请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateProtectedRangesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 更新列表
    pub fields: String, // 指定更新的字段
    /// 保护范围列表
    pub protected_ranges: Vec<ProtectedRange>,
}

/// 批量更新保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProtectedRangesResponse {
    /// 响应数据
    pub data: UpdateProtectedRangesData,
}

/// 更新保护范围数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProtectedRangesData {
    /// 更新结果列表
    pub protected_ranges: Vec<ProtectedRangeResult>,
}

impl Default for UpdateProtectedRangesResponse {
    fn default() -> Self {
        Self {
            data: UpdateProtectedRangesData {
                protected_ranges: vec![],
            },
        }
    }
}

impl ApiResponseTrait for UpdateProtectedRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取保护范围请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetProtectedRangesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 保护范围ID列表
    pub range_ids: Vec<String>,
}

/// 批量获取保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProtectedRangesResponse {
    /// 响应数据
    pub data: GetProtectedRangesData,
}

/// 获取保护范围数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProtectedRangesData {
    /// 保护范围列表
    pub protected_ranges: Vec<ProtectedRange>,
}

impl Default for GetProtectedRangesResponse {
    fn default() -> Self {
        Self {
            data: GetProtectedRangesData {
                protected_ranges: vec![],
            },
        }
    }
}

impl ApiResponseTrait for GetProtectedRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除保护范围请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteProtectedRangesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 保护范围ID列表
    pub range_ids: Vec<String>,
}

/// 批量删除保护范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProtectedRangesResponse {
    /// 响应数据
    pub data: DeleteProtectedRangesData,
}

/// 删除保护范围数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProtectedRangesData {
    /// 删除结果列表
    pub protected_ranges: Vec<ProtectedRangeResult>,
}

impl Default for DeleteProtectedRangesResponse {
    fn default() -> Self {
        Self {
            data: DeleteProtectedRangesData {
                protected_ranges: vec![],
            },
        }
    }
}

impl ApiResponseTrait for DeleteProtectedRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 保护范围管理服务
#[derive(Clone, Debug)]
pub struct ProtectedRangesService {
    config: Config,
}

impl ProtectedRangesService {
    /// 创建新的保护范围管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量创建保护范围
    ///
    /// # 参数
    /// - `request`: 创建保护范围请求
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::protected_ranges::*;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = ProtectedRangesService::new(config);
    ///
    /// let protected_range = ProtectedRange {
    ///     range_id: None,
    ///     range: "Sheet1!A1:C10".to_string(),
    ///     description: Some("重要数据区域".to_string()),
    ///     protected_sheet_id: Some("sheet_id_123".to_string()),
    ///     warning_only: Some(false),
    ///     email_addresses: Some(vec!["user@example.com".to_string()]),
    /// };
    ///
    /// let request = CreateProtectedRangesRequest {
    ///     spreadsheet_token: "spreadsheet_token_123".to_string(),
    ///     protected_ranges: vec![protected_range],
    /// };
    ///
    /// let result = service.create(&request);
    /// ```
    pub async fn create(
        &self,
        request: &CreateProtectedRangesRequest,
    ) -> SDKResult<CreateProtectedRangesResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v2/spreadsheets/{}/protected_dimension",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let create_response: StandardResponse<CreateProtectedRangesResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if let Some(data) = create_response.data {
            Ok(data)
        } else {
            Err(LarkAPIError::InvalidResponse(
                "Missing data field".to_string(),
            ))
        }
    }

    /// 批量更新保护范围
    pub async fn update(
        &self,
        request: &UpdateProtectedRangesRequest,
    ) -> SDKResult<UpdateProtectedRangesResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v2/spreadsheets/{}/protected_range_batch_update",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let update_response: StandardResponse<UpdateProtectedRangesResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if let Some(data) = update_response.data {
            Ok(data)
        } else {
            Err(LarkAPIError::InvalidResponse(
                "Missing data field".to_string(),
            ))
        }
    }

    /// 批量获取保护范围
    pub async fn get(
        &self,
        request: &GetProtectedRangesRequest,
    ) -> SDKResult<GetProtectedRangesResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v2/spreadsheets/{}/protected_range_batch_get?range_ids={}",
            request.spreadsheet_token,
            request.range_ids.join(",")
        );

        let api_request = ApiRequest::with_method_and_path(Method::GET, &endpoint);
        let get_response: StandardResponse<GetProtectedRangesResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if let Some(data) = get_response.data {
            Ok(data)
        } else {
            Err(LarkAPIError::InvalidResponse(
                "Missing data field".to_string(),
            ))
        }
    }

    /// 批量删除保护范围
    pub async fn delete(
        &self,
        request: &DeleteProtectedRangesRequest,
    ) -> SDKResult<DeleteProtectedRangesResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v2/spreadsheets/{}/protected_range_batch_del",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::DELETE, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let delete_response: StandardResponse<DeleteProtectedRangesResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if let Some(data) = delete_response.data {
            Ok(data)
        } else {
            Err(LarkAPIError::InvalidResponse(
                "Missing data field".to_string(),
            ))
        }
    }
}

impl openlark_core::core::trait_system::Service for ProtectedRangesService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ProtectedRangesService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_protected_ranges_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = ProtectedRangesService::new(config);
        assert_eq!(service.service_name(), "ProtectedRangesService");
    }

    #[test]
    fn test_protected_range_default() {
        let protected_range = ProtectedRange {
            range_id: Some("range_123".to_string()),
            range: "Sheet1!A1:C10".to_string(),
            description: Some("测试保护范围".to_string()),
            protected_sheet_id: Some("sheet_456".to_string()),
            warning_only: Some(false),
            email_addresses: Some(vec!["user@example.com".to_string()]),
        };

        assert_eq!(protected_range.range_id, Some("range_123".to_string()));
        assert_eq!(protected_range.range, "Sheet1!A1:C10");
        assert_eq!(protected_range.warning_only, Some(false));
    }

    #[test]
    fn test_create_protected_ranges_request() {
        let protected_range = ProtectedRange {
            range_id: None,
            range: "Sheet1!A1:C10".to_string(),
            description: Some("重要数据".to_string()),
            protected_sheet_id: None,
            warning_only: Some(true),
            email_addresses: None,
        };

        let request = CreateProtectedRangesRequest {
            spreadsheet_token: "token_123".to_string(),
            protected_ranges: vec![protected_range],
        };

        assert_eq!(request.spreadsheet_token, "token_123");
        assert_eq!(request.protected_ranges.len(), 1);
    }

    #[test]
    fn test_api_response_traits() {
        assert_eq!(
            CreateProtectedRangesResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            UpdateProtectedRangesResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            GetProtectedRangesResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            DeleteProtectedRangesResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_protected_range_result() {
        let result = ProtectedRangeResult {
            range_id: "range_123".to_string(),
            request_id: "req_456".to_string(),
        };

        assert_eq!(result.range_id, "range_123");
        assert_eq!(result.request_id, "req_456");
    }

    #[test]
    fn test_serialization() {
        let protected_range = ProtectedRange {
            range_id: Some("test_id".to_string()),
            range: "Sheet1!A1:C10".to_string(),
            description: None,
            protected_sheet_id: None,
            warning_only: Some(false),
            email_addresses: None,
        };

        let json_str = serde_json::to_string(&protected_range);
        assert!(json_str.is_ok());

        let parsed: Result<ProtectedRange, _> = serde_json::from_str(&json_str.unwrap());
        assert!(parsed.is_ok());

        let parsed_range = parsed.unwrap();
        assert_eq!(parsed_range.range, "Sheet1!A1:C10");
        assert_eq!(parsed_range.warning_only, Some(false));
    }
}
