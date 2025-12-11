//! 移动行列
//!
//! 该接口用于移动行列，行列被移动到目标位置后，原本在目标位置的行列会对应右移或下移。
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/move_dimension

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
use serde_json::Value;

use crate::ccm::sheets::v3::models::{
    MoveDimensionRequest, SheetId, SpreadsheetToken, SheetsResponse
};

/// 移动行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDimensionResponse {
    /// 电子表格令牌
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    pub sheet_id: SheetId,
}

impl ApiResponseTrait for MoveDimensionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动行列
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
/// * `request` - 移动行列请求
///
/// # 返回
/// 返回移动行列的操作结果
///
/// # 示例
/// ```rust
/// use openlark_docs::ccm::sheets::v3::spreadsheet::sheet::move_dimension::*;
///
/// let request = MoveDimensionRequest {
///     dimension: "rows".to_string(),
///     source: "A1:A5".to_string(),
///     destination_index: 10,
/// };
///
/// let response = move_dimension(&config, "sheet_token_123", "sheet_id_456", request)
///     .await?;
/// println!("行列移动成功: {}", response.sheet_id);
/// ```
pub async fn move_dimension(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    request: MoveDimensionRequest,
) -> SDKResult<MoveDimensionResponse> {
    let url = format!(
        "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/move_dimension",
        config.base_url, spreadsheet_token, sheet_id
    );

    let mut api_request = ApiRequest::new(Method::POST, &url)
        .bearer_auth(&config.tenant_access_token)
        .body(serde_json::to_value(&request)?);

    let transport = Transport::new(config.clone());
    let response = transport.send_request(&mut api_request).await?;

    let api_response: SheetsResponse<MoveDimensionResponse> =
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

/// 移动行列构建器
pub struct MoveDimensionBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    dimension: Option<String>,
    source: Option<String>,
    destination_index: Option<i32>,
}

impl<'a> MoveDimensionBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            dimension: None,
            source: None,
            destination_index: None,
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

    /// 设置维度（rows/columns）
    pub fn dimension(mut self, dimension: impl Into<String>) -> Self {
        self.dimension = Some(dimension.into());
        self
    }

    /// 设置源范围
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// 设置目标索引
    pub fn destination_index(mut self, index: i32) -> Self {
        self.destination_index = Some(index);
        self
    }

    /// 执行移动行列请求
    pub async fn execute(self) -> SDKResult<MoveDimensionResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        let dimension = self.dimension.ok_or_else(|| {
            LarkAPIError::IllegalParamError("维度不能为空".to_string())
        })?;

        let source = self.source.ok_or_else(|| {
            LarkAPIError::IllegalParamError("源范围不能为空".to_string())
        })?;

        let destination_index = self.destination_index.ok_or_else(|| {
            LarkAPIError::IllegalParamError("目标索引不能为空".to_string())
        })?;

        let request = MoveDimensionRequest {
            dimension,
            source,
            destination_index,
        };

        move_dimension(self.config, &spreadsheet_token, &sheet_id, request).await
    }
}

/// 移动行列的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回移动行列的构建器
pub fn move_dimension_builder(config: &Config) -> MoveDimensionBuilder {
    MoveDimensionBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_dimension_builder() {
        let config = Config::default();
        let builder = move_dimension_builder(&config)
            .spreadsheet_token("sheet_token_123")
            .sheet_id("sheet_id_456")
            .dimension("rows")
            .source("A1:A5")
            .destination_index(10);

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.dimension, Some("rows".to_string()));
        assert_eq!(builder.source, Some("A1:A5".to_string()));
        assert_eq!(builder.destination_index, Some(10));
    }

    #[test]
    fn test_missing_parameters() {
        let config = Config::default();
        let builder = move_dimension_builder(&config);

        assert!(builder.spreadsheet_token.is_none());
        assert!(builder.sheet_id.is_none());
        assert!(builder.dimension.is_none());
        assert!(builder.source.is_none());
        assert!(builder.destination_index.is_none());
    }
}