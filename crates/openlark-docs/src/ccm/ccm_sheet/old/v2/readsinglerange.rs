/// 读取单个范围
///
/// 根据 spreadsheetToken 和 range 读取表格单个范围的值，返回数据限制为10M。
/// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-a-single-range
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 读取单个范围请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadSingleRangeParams {
    /// 电子表格的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 读取范围，如 "Sheet1!A1:C10"
    pub range: String,
    /// 是否包含格式信息
    #[serde(rename = "includeFormat", skip_serializing_if = "Option::is_none")]
    pub include_format: Option<bool>,
    /// 值渲染选项
    #[serde(rename = "valueRenderOption", skip_serializing_if = "Option::is_none")]
    pub value_render_option: Option<String>,
    /// 日期时间渲染选项
    #[serde(
        rename = "dateTimeRenderOption",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_time_render_option: Option<String>,
}

/// 读取单个范围响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadSingleRangeResponse {
    /// 读取结果
    pub data: Option<RangeValue>,
}

/// 范围值数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeValue {
    /// 范围
    pub range: String,
    /// 主要维度范围
    #[serde(rename = "majorDimension")]
    pub major_dimension: String,
    /// 值数据
    pub values: Option<Vec<Vec<serde_json::Value>>>,
}

impl ApiResponseTrait for ReadSingleRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 读取单个范围请求
pub struct ReadSingleRangeRequest {
    config: Config,
}

impl ReadSingleRangeRequest {
    /// 创建读取单个范围请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-a-single-range
    pub async fn execute(
        self,
        params: ReadSingleRangeParams,
    ) -> SDKResult<ReadSingleRangeResponse> {
        // 验证必填字段
        validate_required!(params.spreadsheet_token, "电子表格token不能为空");
        validate_required!(params.range, "读取范围不能为空");

        // 使用enum+builder系统生成API端点
        let api_endpoint =
            CcmSheetApiOld::ReadSingleRange(params.spreadsheet_token.clone());

        // 构建查询参数
        let mut query_params = Vec::new();
        if let Some(include_format) = params.include_format {
            query_params.push(("includeFormat".to_string(), include_format.to_string()));
        }
        if let Some(ref render_option) = params.value_render_option {
            query_params.push(("valueRenderOption".to_string(), render_option.clone()));
        }
        if let Some(ref time_option) = params.date_time_render_option {
            query_params.push(("dateTimeRenderOption".to_string(), time_option.clone()));
        }

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ReadSingleRangeResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 添加查询参数
        for (key, value) in query_params {
            api_request = api_request.query_param(&key, &value);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
