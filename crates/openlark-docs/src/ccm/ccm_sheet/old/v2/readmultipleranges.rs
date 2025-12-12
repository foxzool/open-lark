//! 读取多个范围
//!
//! 根据 spreadsheetToken 和多个 ranges 批量读取表格多个范围的值，返回数据限制为10M。
//! API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-multiple-ranges

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 读取多个范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadMultipleRangesParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 范围列表，每个范围格式如 "Sheet1!A1:C10"
    pub ranges: Vec<String>,
    /// 是否包含格式信息
    #[serde(rename = "includeFormat", skip_serializing_if = "Option::is_none")]
    pub include_format: Option<bool>,
    /// 值渲染选项
    #[serde(rename = "valueRenderOption", skip_serializing_if = "Option::is_none")]
    pub value_render_option: Option<String>,
    /// 日期时间渲染选项
    #[serde(rename = "dateTimeRenderOption", skip_serializing_if = "Option::is_none")]
    pub date_time_render_option: Option<String>,
}

/// 读取多个范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadMultipleRangesResponse {
    /// 读取结果
    pub data: Option<MultipleRangeResult>,
}

/// 多个范围读取结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipleRangeResult {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 范围值列表
    #[serde(rename = "valueRanges")]
    pub value_ranges: Vec<ValueRange>,
}

/// 范围值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueRange {
    /// 范围
    pub range: String,
    /// 主要维度
    #[serde(rename = "majorDimension")]
    pub major_dimension: String,
    /// 值数据
    pub values: Vec<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for ReadMultipleRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 读取多个范围请求
pub struct ReadMultipleRangesRequest {
    config: Config,
}

impl ReadMultipleRangesRequest {
    /// 创建读取多个范围请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-multiple-ranges
    pub async fn execute(
        self,
        params: ReadMultipleRangesParams,
    ) -> SDKResult<ReadMultipleRangesResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");
        validate_required!(params.ranges, "读取范围不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint = CcmSheetApiOld::ValuesBatchGet(params.spreadsheet_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ReadMultipleRangesResponse> =
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