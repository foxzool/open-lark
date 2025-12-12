//! 插入数据
//!
//! 根据 spreadsheetToken 和 valuesPrependRequest 在指定位置插入数据。
//! API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/values/insert-values

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 插入数据请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertValuesParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 插入数据请求
    pub values: ValuesPrependRequest,
}

/// 插入数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesPrependRequest {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 插入位置：A1表示法
    #[serde(rename = "insertionPoint")]
    pub insertion_point: String,
    /// 插入方向：ROWS或COLUMNS
    pub dimension: String,
    /// 要插入的数据
    pub values: Vec<Vec<String>>,
}

/// 插入数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertValuesResponse {
    /// 操作结果
    pub data: Option<InsertValuesResult>,
}

/// 插入数据操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertValuesResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 操作是否成功
    pub success: bool,
    /// 插入的范围
    #[serde(rename = "insertedRange")]
    pub inserted_range: Option<GridRange>,
    /// 更新的表格ID
    #[serde(rename = "updatedSheetId")]
    pub updated_sheet_id: Option<i64>,
}

/// 网格范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridRange {
    /// 工作表ID
    #[serde(rename = "sheetId")]
    pub sheet_id: i64,
    /// 起始行索引（从0开始）
    #[serde(rename = "startRowIndex")]
    pub start_row_index: Option<i32>,
    /// 结束行索引（不包含）
    #[serde(rename = "endRowIndex")]
    pub end_row_index: Option<i32>,
    /// 起始列索引（从0开始）
    #[serde(rename = "startColumnIndex")]
    pub start_column_index: Option<i32>,
    /// 结束列索引（不包含）
    #[serde(rename = "endColumnIndex")]
    pub end_column_index: Option<i32>,
}

impl ApiResponseTrait for InsertValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 插入数据请求
pub struct InsertValuesRequest {
    config: Config,
}

impl InsertValuesRequest {
    /// 创建插入数据请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/values/insert-values
    pub async fn execute(
        self,
        params: InsertValuesParams,
    ) -> SDKResult<InsertValuesResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ValuesPrepend(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<InsertValuesResponse> =
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