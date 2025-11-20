//! Sheets v2 表格属性管理服务
//!
//! 提供飞书电子表格v2版本的表格属性管理功能，支持：
//! - 更新表格标题
//! - 修改表格描述
//! - 更新表格其他属性
//! - 属性变更同步

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

/// 更新表格属性请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdatePropertiesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 表格属性
    pub properties: SpreadsheetProperties,
}

/// 表格属性
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpreadsheetProperties {
    /// 表格标题
    pub title: Option<String>,
    /// 表格描述
    pub description: Option<String>,
    /// 表格URL（只读）
    pub url: Option<String>,
    /// 创建时间（只读）
    pub create_time: Option<String>,
    /// 更新时间（只读）
    pub update_time: Option<String>,
    /// 创建者信息（只读）
    pub creator: Option<Creator>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Creator {
    /// 用户ID
    pub user_id: Option<String>,
    /// 用户邮箱
    pub email: Option<String>,
    /// 用户名称
    pub name: Option<String>,
}

/// 更新表格属性响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePropertiesResponse {
    /// 响应数据
    pub data: UpdatePropertiesData,
}

/// 更新属性数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePropertiesData {
    /// 表格属性
    pub properties: SpreadsheetProperties,
}

impl Default for UpdatePropertiesResponse {
    fn default() -> Self {
        Self {
            data: UpdatePropertiesData {
                properties: SpreadsheetProperties {
                    title: None,
                    description: None,
                    url: None,
                    create_time: None,
                    update_time: None,
                    creator: None,
                },
            },
        }
    }
}

impl ApiResponseTrait for UpdatePropertiesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 表格属性管理服务
#[derive(Clone, Debug)]
pub struct PropertiesService {
    config: Config,
}

impl PropertiesService {
    /// 创建新的表格属性管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 更新表格属性
    ///
    /// # 参数
    /// - `request`: 更新属性请求
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::properties::*;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = PropertiesService::new(config);
    ///
    /// let properties = SpreadsheetProperties {
    ///     title: Some("新的表格标题".to_string()),
    ///     description: Some("表格描述信息".to_string()),
    ///     url: None,
    ///     create_time: None,
    ///     update_time: None,
    ///     creator: None,
    /// };
    ///
    /// let request = UpdatePropertiesRequest {
    ///     spreadsheet_token: "spreadsheet_token_123".to_string(),
    ///     properties: properties,
    /// };
    ///
    /// let result = service.update(&request);
    /// ```
    pub async fn update(
        &self,
        request: &UpdatePropertiesRequest,
    ) -> SDKResult<UpdatePropertiesResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v2/spreadsheets/{}/properties",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::PUT, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let update_response: StandardResponse<UpdatePropertiesResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if let Some(data) = update_response.data {
            Ok(data)
        } else {
            Err(LarkAPIError::InvalidResponse(
                "Missing data field".to_string(),
            ))
        }
    }

    /// 更新表格标题（便捷方法）
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `title`: 新标题
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::properties::PropertiesService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = PropertiesService::new(config);
    ///
    /// let result = service.update_title("token_123", "新标题");
    /// ```
    pub async fn update_title(
        &self,
        spreadsheet_token: &str,
        title: &str,
    ) -> SDKResult<UpdatePropertiesResponse> {
        let properties = SpreadsheetProperties {
            title: Some(title.to_string()),
            description: None,
            url: None,
            create_time: None,
            update_time: None,
            creator: None,
        };

        let request = UpdatePropertiesRequest {
            spreadsheet_token: spreadsheet_token.to_string(),
            properties,
        };

        self.update(&request).await
    }

    /// 更新表格描述（便捷方法）
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `description`: 新描述
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::properties::PropertiesService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = PropertiesService::new(config);
    ///
    /// let result = service.update_description("token_123", "新的描述信息");
    /// ```
    pub async fn update_description(
        &self,
        spreadsheet_token: &str,
        description: &str,
    ) -> SDKResult<UpdatePropertiesResponse> {
        let properties = SpreadsheetProperties {
            title: None,
            description: Some(description.to_string()),
            url: None,
            create_time: None,
            update_time: None,
            creator: None,
        };

        let request = UpdatePropertiesRequest {
            spreadsheet_token: spreadsheet_token.to_string(),
            properties,
        };

        self.update(&request).await
    }

    /// 同时更新标题和描述（便捷方法）
    ///
    /// # 参数
    /// - `spreadsheet_token`: 电子表格token
    /// - `title`: 新标题
    /// - `description`: 新描述
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::properties::PropertiesService;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = PropertiesService::new(config);
    ///
    /// let result = service.update_title_and_description(
    ///     "token_123",
    ///     "新标题",
    ///     "新的描述信息"
    /// );
    /// ```
    pub async fn update_title_and_description(
        &self,
        spreadsheet_token: &str,
        title: &str,
        description: &str,
    ) -> SDKResult<UpdatePropertiesResponse> {
        let properties = SpreadsheetProperties {
            title: Some(title.to_string()),
            description: Some(description.to_string()),
            url: None,
            create_time: None,
            update_time: None,
            creator: None,
        };

        let request = UpdatePropertiesRequest {
            spreadsheet_token: spreadsheet_token.to_string(),
            properties,
        };

        self.update(&request).await
    }
}

impl openlark_core::core::trait_system::Service for PropertiesService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "PropertiesService"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_properties_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = PropertiesService::new(config);
        assert_eq!(service.service_name(), "PropertiesService");
    }

    #[test]
    fn test_spreadsheet_properties() {
        let creator = Creator {
            user_id: Some("user_123".to_string()),
            email: Some("user@example.com".to_string()),
            name: Some("用户名".to_string()),
        };

        let properties = SpreadsheetProperties {
            title: Some("测试表格".to_string()),
            description: Some("这是一个测试表格".to_string()),
            url: Some("https://example.com".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
            creator: Some(creator),
        };

        assert_eq!(properties.title, Some("测试表格".to_string()));
        assert_eq!(properties.description, Some("这是一个测试表格".to_string()));
        assert!(properties.creator.is_some());

        let creator = properties.creator.unwrap();
        assert_eq!(creator.user_id, Some("user_123".to_string()));
        assert_eq!(creator.email, Some("user@example.com".to_string()));
    }

    #[test]
    fn test_update_properties_request() {
        let properties = SpreadsheetProperties {
            title: Some("新标题".to_string()),
            description: None,
            url: None,
            create_time: None,
            update_time: None,
            creator: None,
        };

        let request = UpdatePropertiesRequest {
            spreadsheet_token: "token_123".to_string(),
            properties,
        };

        assert_eq!(request.spreadsheet_token, "token_123");
        assert_eq!(request.properties.title, Some("新标题".to_string()));
        assert!(request.properties.description.is_none());
    }

    #[test]
    fn test_creator() {
        let creator = Creator {
            user_id: Some("user_456".to_string()),
            email: Some("test@example.com".to_string()),
            name: Some("测试用户".to_string()),
        };

        assert_eq!(creator.user_id, Some("user_456".to_string()));
        assert_eq!(creator.email, Some("test@example.com".to_string()));
        assert_eq!(creator.name, Some("测试用户".to_string()));
    }

    #[test]
    fn test_api_response_trait() {
        assert_eq!(
            UpdatePropertiesResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_serialization() {
        let properties = SpreadsheetProperties {
            title: Some("序列化测试".to_string()),
            description: Some("测试序列化功能".to_string()),
            url: None,
            create_time: None,
            update_time: None,
            creator: None,
        };

        let json_str = serde_json::to_string(&properties);
        assert!(json_str.is_ok());

        let parsed: Result<SpreadsheetProperties, _> = serde_json::from_str(&json_str.unwrap());
        assert!(parsed.is_ok());

        let parsed_properties = parsed.unwrap();
        assert_eq!(parsed_properties.title, Some("序列化测试".to_string()));
        assert_eq!(
            parsed_properties.description,
            Some("测试序列化功能".to_string())
        );
    }

    #[test]
    fn test_update_properties_response_default() {
        let response = UpdatePropertiesResponse::default();
        assert!(response.data.properties.title.is_none());
        assert!(response.data.properties.description.is_none());
        assert!(response.data.properties.creator.is_none());
    }
}
