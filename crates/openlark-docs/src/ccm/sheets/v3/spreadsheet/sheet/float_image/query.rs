//! 查询浮动图片列表
//!
//! 查询工作表中的所有浮动图片
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/query

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    error::LarkAPIError,
    http::Transport,
    SDKResult,
};

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::ccm::sheets::v3::models::{
    FloatImageInfo, SpreadsheetToken, SheetId, SheetsResponse
};

/// 查询浮动图片列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFloatImagesResponse {
    /// 浮动图片列表
    pub float_images: Vec<FloatImageInfo>,
    /// 总数
    pub total: i32,
}

impl ApiResponseTrait for QueryFloatImagesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询浮动图片列表构建器
pub struct QueryFloatImagesBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl<'a> QueryFloatImagesBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置电子表格令牌
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, id: impl Into<String>) -> Self {
        self.sheet_id = Some(id.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, token: impl Into<String>) -> Self {
        self.page_token = Some(token.into());
        self
    }

    /// 执行查询浮动图片列表请求
    pub async fn execute(self) -> SDKResult<QueryFloatImagesResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let mut url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/query",
            self.config.base_url, spreadsheet_token, sheet_id
        );

        // 添加查询参数
        let mut params = Vec::new();
        if let Some(page_size) = self.page_size {
            params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = self.page_token {
            params.push(format!("page_token={}", page_token));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let mut api_request = ApiRequest::new(Method::GET, &url)
            .bearer_auth(&self.config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<QueryFloatImagesResponse> =
            serde_json::from_str(&response).map_err(|e| {
                LarkAPIError::JsonParseError(format!("响应解析失败: {}", e))
            })?;

        if let Some(data) = api_response.data {
            Ok(data)
        } else if let Some(error) = api_response.error {
            Err(LarkAPIError::APIError(error.to_string()))
        } else {
            Err(LarkAPIError::UnknownError("未知错误".to_string()))
        }
    }
}

/// 查询浮动图片列表
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
///
/// # 返回
/// 返回工作表中的所有浮动图片
///
/// # 示例
/// ```rust
/// let response = query_float_images(&config, "sheet_token_123", "sheet_id_456")
///     .await?;
/// println!("找到 {} 个浮动图片", response.total);
/// for image in response.float_images {
///     println!("图片ID: {}", image.float_image_id);
/// }
/// ```
pub async fn query_float_images(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<QueryFloatImagesResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/query",
        config.base_url, spreadsheet_token, sheet_id
    );

    let mut api_request = ApiRequest::new(Method::GET, &url)
        .bearer_auth(&config.tenant_access_token);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<QueryFloatImagesResponse> =
        serde_json::from_str(&response).map_err(|e| {
            LarkAPIError::JsonParseError(format!("响应解析失败: {}", e))
        })?;

    if let Some(data) = api_response.data {
        Ok(data)
    } else if let Some(error) = api_response.error {
        Err(LarkAPIError::APIError(error.to_string()))
    } else {
        Err(LarkAPIError::UnknownError("未知错误".to_string()))
    }
}

/// 查询浮动图片列表的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回查询浮动图片列表的构建器
pub fn query_float_images_builder(config: &Config) -> QueryFloatImagesBuilder {
    QueryFloatImagesBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_float_images_builder() {
        let config = Config::default();
        let builder = query_float_images_builder(&config)
            .spreadsheet_token("sheet_token_123")
            .sheet_id("sheet_id_456")
            .page_size(20);

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.page_size, Some(20));
    }

    #[test]
    fn test_missing_parameters() {
        let config = Config::default();
        let builder = query_float_images_builder(&config);

        assert!(builder.spreadsheet_token.is_none());
        assert!(builder.sheet_id.is_none());
        assert!(builder.page_size.is_none());
        assert!(builder.page_token.is_none());
    }
}