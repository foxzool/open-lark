//! 更新电子表格属性
//!
//! 根据 spreadsheetToken 更新电子表格的属性，如标题、语言、时区等。
//! API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v2/spreadsheet/update_spreadsheet_properties

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 更新电子表格属性请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetPropertiesParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 电子表格属性
    pub properties: SpreadsheetProperties,
}

/// 电子表格属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetProperties {
    /// 电子表格标题
    pub title: Option<String>,
    /// 语言设置
    #[serde(rename = "locale")]
    pub locale: Option<String>,
    /// 时区设置
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    /// 自动重计算设置
    #[serde(rename = "autoRecalc")]
    pub auto_recalc: Option<String>,
    /// 循环依赖设置
    #[serde(rename = "iterateCalculation")]
    pub iterate_calculation: Option<bool>,
    /// 循环计算阈值
    #[serde(rename = "iterationCount")]
    pub iteration_count: Option<i32>,
    /// 电子表格主题颜色
    #[serde(rename = "spreadsheetTheme")]
    pub spreadsheet_theme: Option<SpreadsheetTheme>,
}

/// 电子表格主题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetTheme {
    /// 主题名称
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// 主要字体族
    #[serde(rename = "fontFamily")]
    pub font_family: Option<String>,
}

/// 更新电子表格属性响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetPropertiesResponse {
    /// 操作结果
    pub data: Option<UpdateSpreadsheetPropertiesResult>,
}

/// 更新电子表格属性结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpreadsheetPropertiesResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 更新后的属性
    pub properties: Option<SpreadsheetProperties>,
}

impl ApiResponseTrait for UpdateSpreadsheetPropertiesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新电子表格属性请求
pub struct UpdateSpreadsheetPropertiesRequest {
    config: Config,
}

impl UpdateSpreadsheetPropertiesRequest {
    /// 创建更新电子表格属性请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v2/spreadsheet/update_spreadsheet_properties
    pub async fn execute(
        self,
        params: UpdateSpreadsheetPropertiesParams,
    ) -> SDKResult<UpdateSpreadsheetPropertiesResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::Properties(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<UpdateSpreadsheetPropertiesResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serde_json::to_value(params).map_err(|e| {
                    openlark_core::error::validation_error(
                        "参数序列化失败",
                        &format!("无法序列化请求参数: {}", e)
                    )
                })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
