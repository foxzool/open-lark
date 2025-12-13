/// 批量写入范围
///
/// 根据 spreadsheetToken 向多个范围批量写入数据，单次写入不超过10000个单元格。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/batch-write-data-to-multiple-ranges
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 批量写入范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchWriteRangesParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 写入数据列表
    pub data: Vec<WriteData>,
    /// 值输入选项
    #[serde(rename = "valueInputOption", skip_serializing_if = "Option::is_none")]
    pub value_input_option: Option<String>,
    /// 是否包含数据验证
    #[serde(
        rename = "includeDataValidation",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_data_validation: Option<bool>,
}

/// 写入数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteData {
    /// 写入范围，如 "Sheet1!A1:C10"
    pub range: String,
    /// 值数据，二维数组
    pub values: Vec<Vec<serde_json::Value>>,
}

/// 批量写入范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchWriteRangesResponse {
    /// 写入结果
    pub data: Option<BatchWriteResult>,
}

/// 批量写入结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchWriteResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 更新的行数
    #[serde(rename = "updatedRows")]
    pub updated_rows: Option<i32>,
    /// 更新的列数
    #[serde(rename = "updatedColumns")]
    pub updated_columns: Option<i32>,
    /// 更新的单元格数量
    #[serde(rename = "updatedCells")]
    pub updated_cells: Option<i32>,
    /// 写入结果列表
    #[serde(rename = "writeResults")]
    pub write_results: Option<Vec<WriteResult>>,
}

/// 写入结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteResult {
    /// 写入范围
    pub range: String,
    /// 更新的行数
    #[serde(rename = "updatedRows")]
    pub updated_rows: i32,
    /// 更新的列数
    #[serde(rename = "updatedColumns")]
    pub updated_columns: i32,
    /// 更新的单元格数量
    #[serde(rename = "updatedCells")]
    pub updated_cells: i32,
}

impl ApiResponseTrait for BatchWriteRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量写入范围请求
pub struct BatchWriteRangesRequest {
    config: Config,
}

impl BatchWriteRangesRequest {
    /// 创建批量写入范围请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/batch-write-data-to-multiple-ranges
    pub async fn execute(
        self,
        params: BatchWriteRangesParams,
    ) -> SDKResult<BatchWriteRangesResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");
        validate_required!(params.data, "写入数据不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ValuesBatchUpdate(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<BatchWriteRangesResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_value(params).map_err(|e| {
            openlark_core::error::validation_error(
                "参数序列化失败",
                &format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
