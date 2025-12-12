//! 合并单元格
//!
//! 根据 spreadsheetToken 和 mergeRequest 合并指定范围内的单元格。
//! API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/cells-format/merge-cells

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 合并单元格请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 合并请求
    pub merge: MergeRequest,
}

/// 合并请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequest {
    /// 合并类型，可选值：MERGE_ALL, MERGE_ROWS, MERGE_COLUMNS
    #[serde(rename = "mergeType")]
    pub merge_type: String,
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 起始行索引（从0开始）
    #[serde(rename = "startRowIndex")]
    pub start_row_index: i32,
    /// 结束行索引（不包含）
    #[serde(rename = "endRowIndex")]
    pub end_row_index: i32,
    /// 起始列索引（从0开始）
    #[serde(rename = "startColumnIndex")]
    pub start_column_index: i32,
    /// 结束列索引（不包含）
    #[serde(rename = "endColumnIndex")]
    pub end_column_index: i32,
}

/// 合并单元格响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeCellsResponse {
    /// 合并结果
    pub data: Option<MergeResult>,
}

/// 合并结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 合并的单元格范围
    #[serde(rename = "mergeRanges")]
    pub merge_ranges: Option<Vec<MergeRange>>,
}

/// 合并范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRange {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 起始行索引
    #[serde(rename = "startRowIndex")]
    pub start_row_index: i32,
    /// 结束行索引
    #[serde(rename = "endRowIndex")]
    pub end_row_index: i32,
    /// 起始列索引
    #[serde(rename = "startColumnIndex")]
    pub start_column_index: i32,
    /// 结束列索引
    #[serde(rename = "endColumnIndex")]
    pub end_column_index: i32,
    /// 合并类型
    #[serde(rename = "mergeType")]
    pub merge_type: String,
}

impl ApiResponseTrait for MergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 合并单元格请求
pub struct MergeCellsRequest {
    config: Config,
}

impl MergeCellsRequest {
    /// 创建合并单元格请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/cells-format/merge-cells
    pub async fn execute(
        self,
        params: MergeCellsParams,
    ) -> SDKResult<MergeCellsResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::MergeCells(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<MergeCellsResponse> =
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
