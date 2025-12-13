/// 写入单个范围
///
/// 根据 spreadsheetToken 和 range 向单个范围写入数据，若范围内有数据，将被更新覆盖；单次写入不超过5000行，100列，每个格子不超过5万字符。
/// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-to-a-single-range
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 写入单个范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteSingleRangeParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 写入范围，如 "Sheet1!A1:C10"
    pub range: String,
    /// 值数据，二维数组
    pub values: Vec<Vec<serde_json::Value>>,
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

/// 写入单个范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteSingleRangeResponse {
    /// 写入结果
    pub data: Option<UpdateResult>,
}

/// 更新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
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
    /// 更新的范围
    #[serde(rename = "updatedRange")]
    pub updated_range: Option<String>,
}

impl ApiResponseTrait for WriteSingleRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 写入单个范围请求
pub struct WriteSingleRangeRequest {
    config: Config,
}

impl WriteSingleRangeRequest {
    /// 创建写入单个范围请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-data-to-a-single-range
    pub async fn execute(
        self,
        params: WriteSingleRangeParams,
    ) -> SDKResult<WriteSingleRangeResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");
        validate_required!(params.range, "写入范围不能为空");
        validate_required!(params.values, "写入数据不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::WriteSingleRange(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<WriteSingleRangeResponse> = ApiRequest::put(&format!(
            "/open-apis/sheets/v2/spreadsheets/{}/values/{}",
            params.spreadsheet_token, params.range
        ))
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
