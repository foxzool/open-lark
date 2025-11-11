#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 电子表格获取 API
//!
//! 提供获取飞书电子表格详细信息的API，包括表格元数据、权限信息等。
//! 支持通过不同的用户ID类型进行查询。

use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::SDKResult;
use open_lark_core::error::LarkAPIError;

use crate::{
    core::{
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::SHEETS_V3_SPREADSHEETS,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 电子表格服务
pub struct SpreadsheetService {
    config: Config,
}

impl SpreadsheetService {
    /// 创建新的电子表格服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取电子表格信息
    ///
    /// 根据电子表格token获取表格的详细信息，包括标题、所有者、权限等。
    ///
    /// # 参数
    ///
    /// * `request` - 获取电子表格信息的请求参数
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回电子表格的完整信息，包括元数据和权限信息。
    ///
    /// # 示例
    ///
    /// ```rust
    /// let request = GetSpreadsheetRequest::builder()
    ///     .spreadsheet_token("shtcnmBA*****yGehy8".to_string())
    ///     .user_id_type("open_id".to_string())
    ///     .build();
    ///
    /// let response = client.sheets.v3.spreadsheet
    ///     .get(request, None)
    ///     .await?;
    /// ```
    pub async fn get(
        &self,
        request: GetSpreadsheetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetSpreadsheetResponseData>> {
        let mut api_req = request.api_request;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(SHEETS_V3_SPREADSHEETS.to_string());
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User
        ]);

        // 添加查询参数
        if let Some(user_id_type) = &request.user_id_type {
            api_req.query_params.insert("user_id_type", user_id_type.clone());
        }

        api_req.set_body(serde_json::to_vec(&request)?);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

/// 获取电子表格请求
///
/// 用于获取电子表格详细信息的请求参数。
#[derive(Debug, Clone, Serialize)]
pub struct GetSpreadsheetRequest {
    #[serde(skip)]
    api_request: crate::core::api_req::ApiRequest,
    /// 电子表格token
    ///
    /// 要查询的电子表格的唯一标识符
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 用户ID类型
    ///
    /// 指定返回的用户ID类型，影响响应中用户标识的格式
    ///
    /// # 可选值
    ///
    /// - `open_id`：用户在特定应用中的身份标识（默认）
    /// - `union_id`：用户在开发商下的统一身份标识
    /// - `user_id`：用户在租户内的身份标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetSpreadsheetRequest {
    /// 创建新的请求实例
    ///
    /// # 参数
    ///
    /// * `spreadsheet_token` - 电子表格的唯一标识符
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            api_request: Default::default(),
            spreadsheet_token: spreadsheet_token.into(),
            user_id_type: None,
        }
    }

    /// 创建构建器
    pub fn builder() -> GetSpreadsheetRequestBuilder {
        GetSpreadsheetRequestBuilder::default()
    }

    /// 验证请求参数
    ///
    /// 检查请求参数的有效性，确保符合API要求。
    pub fn validate(&self) -> SDKResult<()> {
        // 验证spreadsheet_token
        if self.spreadsheet_token.is_empty() {
            return Err(crate::core::error::LarkAPIError::illegal_param(
                "spreadsheet_token不能为空".to_string()
            ));
        }

        // 验证user_id_type格式
        if let Some(user_id_type) = &self.user_id_type {
            match user_id_type.as_str() {
                "open_id" | "union_id" | "user_id" => {
                    // 有效的值
                }
                _ => {
                    return Err(crate::core::error::LarkAPIError::illegal_param(
                        format!(
                            "user_id_type必须是以下值之一：open_id, union_id, user_id，当前值：{}",
                            user_id_type
                        )
                    ));
                }
            }
        }

        Ok(())
    }
}

/// 获取电子表格请求构建器
///
/// 提供流式API来构建获取电子表格信息的请求。
#[derive(Debug, Clone, Default)]
pub struct GetSpreadsheetRequestBuilder {
    request: GetSpreadsheetRequest,
}

impl GetSpreadsheetRequestBuilder {
    /// 设置电子表格token
    ///
    /// # 参数
    ///
    /// * `spreadsheet_token` - 电子表格的唯一标识符
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl Into<String>) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.into();
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    ///
    /// * `user_id_type` - 用户ID类型，可选值：open_id, union_id, user_id
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 构建请求对象
    ///
    /// 创建最终的请求实例，会自动验证参数。
    ///
    /// # 返回值
    ///
    /// 返回验证通过的请求对象
    ///
    /// # 错误
    ///
    /// 如果参数验证失败，会返回相应的错误
    pub fn build(self) -> SDKResult<GetSpreadsheetRequest> {
        self.request.validate()?;
        Ok(self.request)
    }
}

/// Trait实现，支持Builder模式
impl_executable_builder_owned!(
    GetSpreadsheetRequestBuilder,
    SpreadsheetService,
    GetSpreadsheetRequest,
    BaseResponse<GetSpreadsheetResponseData>,
    get,
);

/// 获取电子表格响应数据
///
/// 包含电子表格的详细信息。
#[derive(Debug, Clone, Deserialize)]
pub struct GetSpreadsheetResponseData {
    /// 电子表格信息
    pub spreadsheet: GetSpreadsheetResponse,
}

impl ApiResponseTrait for GetSpreadsheetResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取电子表格响应
///
/// 包含电子表格的完整信息。
#[derive(Debug, Clone, Deserialize)]
pub struct GetSpreadsheetResponse {
    /// 电子表格标题
    pub title: String,
    /// 电子表格所有者ID
    pub owner_id: String,
    /// 电子表格token
    pub token: String,
    /// 电子表格访问URL
    pub url: String,
    /// 电子表格创建时间
    #[serde(default)]
    pub create_time: Option<String>,
    /// 电子表格更新时间
    #[serde(default)]
    pub update_time: Option<String>,
    /// 工作表数量
    #[serde(default)]
    pub sheet_count: Option<i32>,
    /// 权限信息
    #[serde(default)]
    pub permissions: Option<SpreadsheetPermissions>,
}

/// 电子表格权限信息
#[derive(Debug, Clone, Deserialize)]
pub struct SpreadsheetPermissions {
    /// 是否可以编辑
    #[serde(default)]
    pub can_edit: Option<bool>,
    /// 是否可以分享
    #[serde(default)]
    pub can_share: Option<bool>,
    /// 是否可以评论
    #[serde(default)]
    pub can_comment: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_spreadsheet_request_builder() {
        let request = GetSpreadsheetRequest::builder()
            .spreadsheet_token("shtcnmBA*****yGehy8")
            .user_id_type("open_id")
            .build()
            .unwrap();

        assert_eq!(request.spreadsheet_token, "shtcnmBA*****yGehy8");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_spreadsheet_request_validation() {
        // 测试空spreadsheet_token验证
        let request = GetSpreadsheetRequest {
            api_request: Default::default(),
            spreadsheet_token: "".to_string(),
            user_id_type: None,
        };

        assert!(request.validate().is_err());

        // 测试无效user_id_type验证
        let request = GetSpreadsheetRequest {
            api_request: Default::default(),
            spreadsheet_token: "valid_token".to_string(),
            user_id_type: Some("invalid_type".to_string()),
        };

        assert!(request.validate().is_err());

        // 测试有效user_id_type
        let valid_types = ["open_id", "union_id", "user_id"];
        for user_type in &valid_types {
            let request = GetSpreadsheetRequest {
                api_request: Default::default(),
                spreadsheet_token: "valid_token".to_string(),
                user_id_type: Some(user_type.to_string()),
            };
            assert!(request.validate().is_ok());
        }
    }

    #[test]
    fn test_get_spreadsheet_response_deserialization() {
        let json = json!({
            "spreadsheet": {
                "title": "销售数据统计",
                "owner_id": "ou_1234567890",
                "token": "shtcnmBA*****yGehy8",
                "url": "https://example.feishu.cn/sheets/shtcnmBA*****yGehy8",
                "create_time": "2023-01-01T00:00:00Z",
                "update_time": "2023-01-02T12:00:00Z",
                "sheet_count": 3,
                "permissions": {
                    "can_edit": true,
                    "can_share": true,
                    "can_comment": false
                }
            }
        });

        let response: GetSpreadsheetResponseData = serde_json::from_value(json).unwrap();
        let spreadsheet = response.spreadsheet;

        assert_eq!(spreadsheet.title, "销售数据统计");
        assert_eq!(spreadsheet.owner_id, "ou_1234567890");
        assert_eq!(spreadsheet.token, "shtcnmBA*****yGehy8");
        assert_eq!(
            spreadsheet.url,
            "https://example.feishu.cn/sheets/shtcnmBA*****yGehy8"
        );
        assert_eq!(spreadsheet.create_time, Some("2023-01-01T00:00:00Z".to_string()));
        assert_eq!(spreadsheet.update_time, Some("2023-01-02T12:00:00Z".to_string()));
        assert_eq!(spreadsheet.sheet_count, Some(3));

        let permissions = spreadsheet.permissions.unwrap();
        assert_eq!(permissions.can_edit, Some(true));
        assert_eq!(permissions.can_share, Some(true));
        assert_eq!(permissions.can_comment, Some(false));
    }

    #[test]
    fn test_spreadsheet_permissions_partial_data() {
        let json = json!({
            "spreadsheet": {
                "title": "测试表格",
                "owner_id": "ou_123",
                "token": "sht_test",
                "url": "https://example.com",
                "permissions": {
                    "can_edit": false
                }
            }
        });

        let response: GetSpreadsheetResponseData = serde_json::from_value(json).unwrap();
        let spreadsheet = response.spreadsheet;

        assert_eq!(spreadsheet.title, "测试表格");

        let permissions = spreadsheet.permissions.unwrap();
        assert_eq!(permissions.can_edit, Some(false));
        assert_eq!(permissions.can_share, None); // 未提供的字段应该是None
        assert_eq!(permissions.can_comment, None); // 未提供的字段应该是None
    }
}