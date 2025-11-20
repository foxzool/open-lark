//! Sheets v2 条件格式管理服务
//!
//! 提供飞书电子表格v2版本的条件格式管理功能，支持：
//! - 批量创建条件格式
//! - 批量更新条件格式
//! - 批量获取条件格式
//! - 批量删除条件格式
//! - 多种条件格式类型（数据条、色阶、图标集等）

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

/// 条件格式类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionalFormatType {
    /// 单元格值条件
    #[serde(rename = "CELL_IS")]
    CellIs,
    /// 公式条件
    #[serde(rename = "FORMULA")]
    Formula,
    /// 数据条
    #[serde(rename = "DATA_BAR")]
    DataBar,
    /// 色阶
    #[serde(rename = "COLOR_SCALE")]
    ColorScale,
    /// 图标集
    #[serde(rename = "ICON_SET")]
    IconSet,
}

/// 条件格式规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionalFormatRule {
    /// 规则ID
    pub rule_id: Option<String>,
    /// 条件格式类型
    pub format_type: ConditionalFormatType,
    /// 应用范围
    pub ranges: Vec<String>,
    /// 条件
    pub condition: Option<Condition>,
    /// 格式设置
    pub format: Option<CellFormat>,
    /// 数据条设置
    pub data_bar_format: Option<DataBarFormat>,
    /// 色阶设置
    pub color_scale_format: Option<ColorScaleFormat>,
    /// 图标集设置
    pub icon_set_format: Option<IconSetFormat>,
}

/// 条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    /// 条件操作符
    pub operator: String,
    /// 比较值1
    pub value1: Option<String>,
    /// 比较值2（用于BETWEEN操作符）
    pub value2: Option<String>,
    /// 公式（用于FORMULA类型）
    pub formula: Option<String>,
}

/// 单元格格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CellFormat {
    /// 背景色
    pub background_color: Option<String>,
    /// 前景色
    pub foreground_color: Option<String>,
    /// 字体加粗
    pub bold: Option<bool>,
    /// 字体倾斜
    pub italic: Option<bool>,
    /// 字体下划线
    pub underline: Option<bool>,
    /// 字体删除线
    pub strikethrough: Option<bool>,
}

/// 数据条格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataBarFormat {
    /// 最小值
    pub min_value: Option<f64>,
    /// 最大值
    pub max_value: Option<f64>,
    /// 最小值类型
    pub min_type: Option<String>,
    /// 最大值类型
    pub max_type: Option<String>,
    /// 颜色
    pub color: Option<String>,
}

/// 色阶格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColorScaleFormat {
    /// 最小值颜色
    pub min_color: Option<String>,
    /// 中值颜色
    pub mid_color: Option<String>,
    /// 最大值颜色
    pub max_color: Option<String>,
    /// 最小值
    pub min_value: Option<f64>,
    /// 中值
    pub mid_value: Option<f64>,
    /// 最大值
    pub max_value: Option<f64>,
}

/// 图标集格式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IconSetFormat {
    /// 图标集类型
    pub icon_set_type: Option<String>,
    /// 图标集规则
    pub icon_criteria: Option<Vec<IconCriterion>>,
}

/// 图标标准
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IconCriterion {
    /// 操作符
    pub operator: Option<String>,
    /// 值
    pub value: Option<f64>,
    /// 图标
    pub icon: Option<String>,
}

/// 批量创建条件格式请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateConditionalFormatsRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 条件格式规则列表
    pub conditional_formats: Vec<ConditionalFormatRule>,
}

/// 批量创建条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConditionalFormatsResponse {
    /// 响应数据
    pub data: CreateConditionalFormatsData,
}

/// 创建条件格式数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConditionalFormatsData {
    /// 创建结果列表
    pub conditional_formats: Vec<ConditionalFormatResult>,
}

/// 条件格式操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalFormatResult {
    /// 规则ID
    pub rule_id: String,
    /// 请求ID
    pub request_id: String,
}

impl Default for CreateConditionalFormatsResponse {
    fn default() -> Self {
        Self {
            data: CreateConditionalFormatsData {
                conditional_formats: vec![],
            },
        }
    }
}

impl ApiResponseTrait for CreateConditionalFormatsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新条件格式请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateConditionalFormatsRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 更新字段
    pub fields: String,
    /// 条件格式规则列表
    pub conditional_formats: Vec<ConditionalFormatRule>,
}

/// 批量更新条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConditionalFormatsResponse {
    /// 响应数据
    pub data: UpdateConditionalFormatsData,
}

/// 更新条件格式数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConditionalFormatsData {
    /// 更新结果列表
    pub conditional_formats: Vec<ConditionalFormatResult>,
}

impl Default for UpdateConditionalFormatsResponse {
    fn default() -> Self {
        Self {
            data: UpdateConditionalFormatsData {
                conditional_formats: vec![],
            },
        }
    }
}

impl ApiResponseTrait for UpdateConditionalFormatsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取条件格式请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetConditionalFormatsRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 工作表ID列表
    pub sheet_ids: Vec<String>,
}

/// 批量获取条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConditionalFormatsResponse {
    /// 响应数据
    pub data: GetConditionalFormatsData,
}

/// 获取条件格式数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConditionalFormatsData {
    /// 条件格式规则列表
    pub conditional_formats: Vec<ConditionalFormatRule>,
}

impl Default for GetConditionalFormatsResponse {
    fn default() -> Self {
        Self {
            data: GetConditionalFormatsData {
                conditional_formats: vec![],
            },
        }
    }
}

impl ApiResponseTrait for GetConditionalFormatsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除条件格式请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteConditionalFormatsRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 规则ID列表
    pub rule_ids: Vec<String>,
}

/// 批量删除条件格式响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConditionalFormatsResponse {
    /// 响应数据
    pub data: DeleteConditionalFormatsData,
}

/// 删除条件格式数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteConditionalFormatsData {
    /// 删除结果列表
    pub conditional_formats: Vec<ConditionalFormatResult>,
}

impl Default for DeleteConditionalFormatsResponse {
    fn default() -> Self {
        Self {
            data: DeleteConditionalFormatsData {
                conditional_formats: vec![],
            },
        }
    }
}

impl ApiResponseTrait for DeleteConditionalFormatsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 条件格式管理服务
#[derive(Clone, Debug)]
pub struct ConditionalFormatsService {
    config: Config,
}

impl ConditionalFormatsService {
    /// 创建新的条件格式管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 批量创建条件格式
    ///
    /// # 参数
    /// - `request`: 创建条件格式请求
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::sheets::v2::condition_formats::*;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = ConditionalFormatsService::new(config);
    ///
    /// let rule = ConditionalFormatRule {
    ///     rule_id: None,
    ///     format_type: ConditionalFormatType::CellIs,
    ///     ranges: vec!["Sheet1!A1:A10".to_string()],
    ///     condition: Some(Condition {
    ///         operator: "GREATER".to_string(),
    ///         value1: Some("100".to_string()),
    ///         value2: None,
    ///         formula: None,
    ///     }),
    ///     format: Some(CellFormat {
    ///         background_color: Some("#FF0000".to_string()),
    ///         bold: Some(true),
    ///         
    ///     }),
    ///     data_bar_format: None,
    ///     color_scale_format: None,
    ///     icon_set_format: None,
    /// };
    ///
    /// let request = CreateConditionalFormatsRequest {
    ///     spreadsheet_token: "token_123".to_string(),
    ///     conditional_formats: vec![rule],
    /// };
    ///
    /// let result = service.create(&request);
    /// ```
    pub async fn create(
        &self,
        request: &CreateConditionalFormatsRequest,
    ) -> SDKResult<CreateConditionalFormatsResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v2/spreadsheets/{}/condition_formats/batch_create",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let create_response: StandardResponse<CreateConditionalFormatsResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if let Some(data) = create_response.data {
            Ok(data)
        } else {
            Err(LarkAPIError::InvalidResponse(
                "Missing data field".to_string(),
            ))
        }
    }

    /// 批量更新条件格式
    pub async fn update(
        &self,
        request: &UpdateConditionalFormatsRequest,
    ) -> SDKResult<UpdateConditionalFormatsResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v2/spreadsheets/{}/condition_formats/batch_update",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::POST, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let update_response: StandardResponse<UpdateConditionalFormatsResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if let Some(data) = update_response.data {
            Ok(data)
        } else {
            Err(LarkAPIError::InvalidResponse(
                "Missing data field".to_string(),
            ))
        }
    }

    /// 批量获取条件格式
    pub async fn get(
        &self,
        request: &GetConditionalFormatsRequest,
    ) -> SDKResult<GetConditionalFormatsResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v2/spreadsheets/{}/condition_formats?sheet_ids={}",
            request.spreadsheet_token,
            request.sheet_ids.join(",")
        );

        let api_request = ApiRequest::with_method_and_path(Method::GET, &endpoint);
        let get_response: StandardResponse<GetConditionalFormatsResponse> =
            Transport::request(api_request, &self.config, None).await?;

        if let Some(data) = get_response.data {
            Ok(data)
        } else {
            Err(LarkAPIError::InvalidResponse(
                "Missing data field".to_string(),
            ))
        }
    }

    /// 批量删除条件格式
    pub async fn delete(
        &self,
        request: &DeleteConditionalFormatsRequest,
    ) -> SDKResult<DeleteConditionalFormatsResponse> {
        use openlark_core::{api::ApiRequest, http::Transport};
        use reqwest::Method;

        let endpoint = format!(
            "/open-apis/sheets/v2/spreadsheets/{}/condition_formats/batch_delete",
            request.spreadsheet_token
        );

        let mut api_request = ApiRequest::with_method_and_path(Method::DELETE, &endpoint);
        api_request.body = Some(openlark_core::api::RequestData::Json(request))?;

        let delete_response: StandardResponse<DeleteConditionalFormatsResponse> =
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

impl openlark_core::core::trait_system::Service for ConditionalFormatsService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "ConditionalFormatsService"
    }
}

impl Default for CellFormat {
    fn default() -> Self {
        Self {
            background_color: None,
            foreground_color: None,
            bold: None,
            italic: None,
            underline: None,
            strikethrough: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::Service;

    #[test]
    fn test_conditional_formats_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = ConditionalFormatsService::new(config);
        assert_eq!(service.service_name(), "ConditionalFormatsService");
    }

    #[test]
    fn test_conditional_format_type() {
        assert_eq!(ConditionalFormatType::CellIs, ConditionalFormatType::CellIs);
        assert_eq!(
            ConditionalFormatType::Formula,
            ConditionalFormatType::Formula
        );
        assert_eq!(
            ConditionalFormatType::DataBar,
            ConditionalFormatType::DataBar
        );
        assert_eq!(
            ConditionalFormatType::ColorScale,
            ConditionalFormatType::ColorScale
        );
        assert_eq!(
            ConditionalFormatType::IconSet,
            ConditionalFormatType::IconSet
        );
    }

    #[test]
    fn test_cell_format_default() {
        let format = CellFormat::default();
        assert!(format.background_color.is_none());
        assert!(format.foreground_color.is_none());
        assert!(format.bold.is_none());
    }

    #[test]
    fn test_conditional_format_rule() {
        let rule = ConditionalFormatRule {
            rule_id: Some("rule_123".to_string()),
            format_type: ConditionalFormatType::CellIs,
            ranges: vec!["Sheet1!A1:A10".to_string()],
            condition: Some(Condition {
                operator: "GREATER".to_string(),
                value1: Some("100".to_string()),
                value2: None,
                formula: None,
            }),
            format: Some(CellFormat {
                background_color: Some("#FF0000".to_string()),
                bold: Some(true),
                
            }),
            data_bar_format: None,
            color_scale_format: None,
            icon_set_format: None,
        };

        assert_eq!(rule.rule_id, Some("rule_123".to_string()));
        assert_eq!(rule.format_type, ConditionalFormatType::CellIs);
        assert_eq!(rule.ranges.len(), 1);
        assert!(rule.condition.is_some());
        assert!(rule.format.is_some());
    }

    #[test]
    fn test_create_conditional_formats_request() {
        let rule = ConditionalFormatRule {
            rule_id: None,
            format_type: ConditionalFormatType::DataBar,
            ranges: vec!["Sheet1!B1:B10".to_string()],
            condition: None,
            format: None,
            data_bar_format: Some(DataBarFormat {
                min_value: Some(0.0),
                max_value: Some(100.0),
                min_type: Some("MIN".to_string()),
                max_type: Some("MAX".to_string()),
                color: Some("#FF0000".to_string()),
            }),
            color_scale_format: None,
            icon_set_format: None,
        };

        let request = CreateConditionalFormatsRequest {
            spreadsheet_token: "token_123".to_string(),
            conditional_formats: vec![rule],
        };

        assert_eq!(request.spreadsheet_token, "token_123");
        assert_eq!(request.conditional_formats.len(), 1);
    }

    #[test]
    fn test_api_response_traits() {
        assert_eq!(
            CreateConditionalFormatsResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            UpdateConditionalFormatsResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            GetConditionalFormatsResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            DeleteConditionalFormatsResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_serialization() {
        let condition = Condition {
            operator: "BETWEEN".to_string(),
            value1: Some("10".to_string()),
            value2: Some("100".to_string()),
            formula: None,
        };

        let json_str = serde_json::to_string(&condition);
        assert!(json_str.is_ok());

        let parsed: Result<Condition, _> = serde_json::from_str(&json_str.unwrap());
        assert!(parsed.is_ok());

        let parsed_condition = parsed.unwrap();
        assert_eq!(parsed_condition.operator, "BETWEEN");
        assert_eq!(parsed_condition.value1, Some("10".to_string()));
        assert_eq!(parsed_condition.value2, Some("100".to_string()));
    }
}
