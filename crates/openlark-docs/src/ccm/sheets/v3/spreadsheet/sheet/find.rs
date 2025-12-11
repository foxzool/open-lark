//! 查找单元格
//!
//! 在指定范围内查找符合查找条件的单元格
//!
//! 文档链接: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/find

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
    FindCellsRequest, FindResult, SheetId, SpreadsheetToken, SheetsResponse
};

/// 查找单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindCellsResponse {
    /// 电子表格令牌
    pub spreadsheet_token: SpreadsheetToken,
    /// 工作表ID
    pub sheet_id: SheetId,
    /// 查找结果
    pub find_result: FindResult,
}

impl ApiResponseTrait for FindCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查找单元格构建器
pub struct FindCellsBuilder<'a> {
    config: &'a Config,
    spreadsheet_token: Option<SpreadsheetToken>,
    sheet_id: Option<SheetId>,
    request: FindCellsRequest,
}

impl<'a> FindCellsBuilder<'a> {
    /// 创建新的构建器实例
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            spreadsheet_token: None,
            sheet_id: None,
            request: FindCellsRequest {
                range: "A1:Z1000".to_string(), // 默认范围
                find: String::new(),
                match_case: None,
                match_entire_cell: None,
                use_regular_expressions: None,
            },
        }
    }

    /// 设置电子表格令牌
    pub fn spreadsheet_token(mut self, token: impl Into<String>) -> Self {
        self.spreadsheet_token = Some(token.into());
        self
    }

    /// 设置工作表ID
    pub fn sheet_id(mut self, sheet_id: impl Into<String>) -> Self {
        self.sheet_id = Some(sheet_id.into());
        self
    }

    /// 设置查找范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.request.range = range.into();
        self
    }

    /// 设置查找内容
    pub fn find(mut self, find: impl Into<String>) -> Self {
        self.request.find = find.into();
        self
    }

    /// 设置是否匹配大小写
    pub fn match_case(mut self, match_case: bool) -> Self {
        self.request.match_case = Some(match_case);
        self
    }

    /// 设置是否匹配整个单元格
    pub fn match_entire_cell(mut self, match_entire_cell: bool) -> Self {
        self.request.match_entire_cell = Some(match_entire_cell);
        self
    }

    /// 设置是否使用正则表达式
    pub fn use_regular_expressions(mut self, use_regular_expressions: bool) -> Self {
        self.request.use_regular_expressions = Some(use_regular_expressions);
        self
    }

    /// 执行查找单元格请求
    pub async fn execute(self) -> SDKResult<FindCellsResponse> {
        let spreadsheet_token = self.spreadsheet_token.ok_or_else(|| {
            LarkAPIError::IllegalParamError("电子表格令牌不能为空".to_string())
        })?;

        let sheet_id = self.sheet_id.ok_or_else(|| {
            LarkAPIError::IllegalParamError("工作表ID不能为空".to_string())
        })?;

        if self.request.find.is_empty() {
            return Err(LarkAPIError::IllegalParamError("查找内容不能为空".to_string()));
        }

        let url = format!(
            "{}/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/find",
            self.config.base_url, spreadsheet_token, sheet_id
        );

        let body = serde_json::to_value(&self.request).map_err(|e| {
            LarkAPIError::IllegalParamError(format!("请求体序列化失败: {}", e))
        })?;

        let mut api_request = ApiRequest::new(Method::POST, &url)
            .body(body)
            .bearer_auth(&self.config.tenant_access_token);

        let transport = Transport::new(self.config.clone());
        let response = transport.send_request(&mut api_request).await?;

        let api_response: SheetsResponse<FindCellsResponse> =
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

/// 查找单元格
///
/// # 参数
/// * `config` - 配置信息
/// * `spreadsheet_token` - 电子表格令牌
/// * `sheet_id` - 工作表ID
///
/// # 返回
/// 返回查找单元格的构建器
///
/// # 示例
/// ```rust
/// let response = find_cells(&config, "sheet_token_123", "sheet_id_456")
///     .range("A1:C10")
///     .find("关键词")
///     .match_case(false)
///     .execute()
///     .await?;
/// println!("找到 {} 个匹配的单元格", response.find_result.total_count);
/// ```
pub fn find_cells(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> FindCellsBuilder {
    FindCellsBuilder::new(config)
        .spreadsheet_token(spreadsheet_token)
        .sheet_id(sheet_id)
}

/// 查找单元格的便捷构建器
///
/// # 参数
/// * `config` - 配置信息
///
/// # 返回
/// 返回查找单元格的构建器
pub fn find_cells_builder(config: &Config) -> FindCellsBuilder {
    FindCellsBuilder::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cells_builder() {
        let config = Config::default();
        let builder = find_cells_builder(&config)
            .spreadsheet_token("sheet_token_123")
            .sheet_id("sheet_id_456")
            .range("A1:C10")
            .find("关键词")
            .match_case(false);

        assert_eq!(builder.spreadsheet_token, Some("sheet_token_123".to_string()));
        assert_eq!(builder.sheet_id, Some("sheet_id_456".to_string()));
        assert_eq!(builder.request.range, "A1:C10");
        assert_eq!(builder.request.find, "关键词");
        assert_eq!(builder.request.match_case, Some(false));
    }

    #[test]
    fn test_empty_find_content() {
        let config = Config::default();
        let builder = find_cells(&config, "sheet_token_123", "sheet_id_456")
            .range("A1:C10");

        assert_eq!(builder.request.find, "");
    }
}