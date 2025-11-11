#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 电子表格创建 API
//!
//! 提供创建新的飞书电子表格的功能，支持设置标题和指定文件夹位置。
//! 这是操作电子表格的基础入口点。

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

    /// 创建电子表格
    ///
    /// 创建一个新的飞书电子表格，可以指定标题和目标文件夹。
    ///
    /// # 参数
    ///
    /// * `request` - 创建电子表格的请求参数
    /// * `option` - 可选的请求配置
    ///
    /// # 返回值
    ///
    /// 返回创建的电子表格信息，包含表格token和访问链接。
    ///
    /// # 示例
    ///
    /// ```rust
    /// let request = CreateSpreadsheetRequest::builder()
    ///     .title("销售数据统计".to_string())
    ///     .folder_token("folder_token".to_string())
    ///     .build();
    ///
    /// let response = client.sheets.v3.spreadsheet
    ///     .create(request, None)
    ///     .await?;
    /// ```
    pub async fn create(
        &self,
        request: CreateSpreadsheetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateSpreadsheetResponseData>> {
        let mut api_req = request.api_request;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(SHEETS_V3_SPREADSHEETS.to_string());
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User
        ]);
        api_req.set_body(serde_json::to_vec(&request)?);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}

/// 创建电子表格请求
///
/// 用于创建新的电子表格的请求参数。
#[derive(Debug, Clone, Serialize)]
pub struct CreateSpreadsheetRequest {
    #[serde(skip)]
    api_request: crate::core::api_req::ApiRequest,
    /// 表格标题
    ///
    /// 电子表格的显示名称，长度范围：0-255字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 文件夹token
    ///
    /// 目标文件夹的标识符，用于指定电子表格的存储位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

impl CreateSpreadsheetRequest {
    /// 创建新的请求实例
    pub fn new() -> Self {
        Self {
            api_request: Default::default(),
            title: None,
            folder_token: None,
        }
    }

    /// 创建构建器
    pub fn builder() -> CreateSpreadsheetRequestBuilder {
        CreateSpreadsheetRequestBuilder::default()
    }

    /// 验证请求参数
    ///
    /// 检查请求参数的有效性，确保符合API要求。
    pub fn validate(&self) -> SDKResult<()> {
        // 验证标题长度
        if let Some(title) = &self.title {
            if title.len() > 255 {
                return Err(crate::core::error::LarkAPIError::illegal_param(
                    format!("标题长度不能超过255个字符，当前长度：{}", title.len())
                ));
            }
        }

        // 如果提供了folder_token，验证其格式
        if let Some(folder_token) = &self.folder_token {
            if folder_token.is_empty() {
                return Err(crate::core::error::LarkAPIError::illegal_param(
                    "folder_token不能为空".to_string()
                ));
            }
        }

        Ok(())
    }
}

/// 创建电子表格请求构建器
///
/// 提供流式API来构建创建电子表格的请求。
#[derive(Debug, Clone, Default)]
pub struct CreateSpreadsheetRequestBuilder {
    request: CreateSpreadsheetRequest,
}

impl CreateSpreadsheetRequestBuilder {
    /// 设置表格标题
    ///
    /// # 参数
    ///
    /// * `title` - 电子表格的标题，长度不超过255字符
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.request.title = Some(title.into());
        self
    }

    /// 设置文件夹token
    ///
    /// # 参数
    ///
    /// * `folder_token` - 目标文件夹的标识符
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.request.folder_token = Some(folder_token.into());
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
    pub fn build(self) -> SDKResult<CreateSpreadsheetRequest> {
        self.request.validate()?;
        Ok(self.request)
    }
}

/// Trait实现，支持Builder模式
impl_executable_builder_owned!(
    CreateSpreadsheetRequestBuilder,
    SpreadsheetService,
    CreateSpreadsheetRequest,
    BaseResponse<CreateSpreadsheetResponseData>,
    create,
);

/// 创建电子表格响应数据
///
/// 包含创建成功的电子表格信息。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSpreadsheetResponseData {
    /// 创建的电子表格信息
    pub spreadsheet: CreateSpreadsheetResponse,
}

impl ApiResponseTrait for CreateSpreadsheetResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建电子表格响应
///
/// 包含新创建的电子表格的详细信息。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateSpreadsheetResponse {
    /// 表格标题
    pub title: String,
    /// 文件夹token
    pub folder_token: String,
    /// 文档访问URL
    pub url: String,
    /// 电子表格token
    ///
    /// 用于后续操作电子表格的唯一标识符
    pub spreadsheet_token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_spreadsheet_request_builder() {
        let request = CreateSpreadsheetRequest::builder()
            .title("测试表格")
            .folder_token("test_folder_token")
            .build()
            .unwrap();

        assert_eq!(request.title, Some("测试表格".to_string()));
        assert_eq!(request.folder_token, Some("test_folder_token".to_string()));
    }

    #[test]
    fn test_create_spreadsheet_request_validation() {
        // 测试标题长度验证
        let long_title = "a".repeat(256);
        let request = CreateSpreadsheetRequest {
            api_request: Default::default(),
            title: Some(long_title.clone()),
            folder_token: None,
        };

        assert!(request.validate().is_err());

        // 测试空folder_token验证
        let request = CreateSpreadsheetRequest {
            api_request: Default::default(),
            title: None,
            folder_token: Some("".to_string()),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_create_spreadsheet_response_deserialization() {
        let json = json!({
            "spreadsheet": {
                "title": "销售数据统计",
                "folder_token": "fldcnMsNb*****hIW9IjG1LVswg",
                "url": "https://example.feishu.cn/sheets/shtcnmBA*****yGehy8",
                "spreadsheet_token": "shtcnmBA*****yGehy8"
            }
        });

        let response: CreateSpreadsheetResponseData = serde_json::from_value(json).unwrap();
        let spreadsheet = response.spreadsheet;

        assert_eq!(spreadsheet.title, "销售数据统计");
        assert_eq!(spreadsheet.folder_token, "fldcnMsNb*****hIW9IjG1LVswg");
        assert_eq!(
            spreadsheet.url,
            "https://example.feishu.cn/sheets/shtcnmBA*****yGehy8"
        );
        assert_eq!(spreadsheet.spreadsheet_token, "shtcnmBA*****yGehy8");
    }

    #[test]
    fn test_empty_request_validation() {
        let request = CreateSpreadsheetRequest::new();
        assert!(request.validate().is_ok()); // 空请求应该有效
    }
}