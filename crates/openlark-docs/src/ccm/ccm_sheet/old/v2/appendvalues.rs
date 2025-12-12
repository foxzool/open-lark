//! 追加数据
//!
//! 向电子表格的指定范围追加数据。
//! API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/append-values

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 追加数据请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendValuesParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 工作表ID或工作表名称
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// 写入范围，如 "A1:C10"
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// 值数据，二维数组
    pub values: Vec<Vec<serde_json::Value>>,
    /// 值输入选项
    #[serde(rename = "valueRenderOption", skip_serializing_if = "Option::is_none")]
    pub value_render_option: Option<String>,
    /// 日期时间渲染选项
    #[serde(rename = "dateTimeRenderOption", skip_serializing_if = "Option::is_none")]
    pub date_time_render_option: Option<String>,
    /// 插入数据选项
    #[serde(rename = "insertDataOption", skip_serializing_if = "Option::is_none")]
    pub insert_data_option: Option<String>,
}

/// 追加数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendValuesResponse {
    /// 操作结果
    pub data: Option<AppendValuesResult>,
}

/// 追加数据结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendValuesResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 更新的范围
    #[serde(rename = "updatesRange")]
    pub updates_range: Option<String>,
    /// 更新的行数
    #[serde(rename = "updatedRows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updatedColumns")]
    pub updated_columns: i32,
    /// 更新的单元格数
    #[serde(rename = "updatedCells")]
    pub updated_cells: i32,
    /// 追加的数据
    pub values: Option<Vec<Vec<serde_json::Value>>>,
}

impl ApiResponseTrait for AppendValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 追加数据请求
pub struct AppendValuesRequest {
    config: Config,
}

impl AppendValuesRequest {
    /// 创建追加数据请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v2/data-operation/append-values
    pub async fn execute(
        self,
        params: AppendValuesParams,
    ) -> SDKResult<AppendValuesResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");
        validate_required!(params.sheet_id, "工作表ID不能为空");
        validate_required!(params.values, "值数据不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ValuesAppend(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<AppendValuesResponse> =
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
