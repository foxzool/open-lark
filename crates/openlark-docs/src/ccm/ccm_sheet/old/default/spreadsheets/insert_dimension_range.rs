use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 插入行列请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDimensionRangeRequest {
    /// 表格 token
    #[serde(skip)]
    pub spreadsheet_token: String,
    /// 维度信息
    pub dimension: serde_json::Value,
    /// 继承样式，可选 "BEFORE" 或 "AFTER"
    pub inheritStyle: Option<String>,
}

impl InsertDimensionRangeRequest {
    /// 创建新的 InsertDimensionRangeRequest
    pub fn new(spreadsheet_token: impl Into<String>, dimension: serde_json::Value) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
            dimension,
            inheritStyle: None,
        }
    }
    
    /// 设置继承样式
    pub fn inherit_style(mut self, inherit_style: impl Into<String>) -> Self {
        self.inheritStyle = Some(inherit_style.into());
        self
    }
}

/// 插入行列响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertDimensionRangeResponse {
    /// 响应消息
    pub message: String,
}

impl ApiResponseTrait for InsertDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 插入行列
///
/// 插入行列。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-row-col-structure/insert-rows-or-columns
pub async fn insert_dimension_range(
    request: InsertDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<InsertDimensionRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::InsertDimensionRange(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<InsertDimensionRangeResponse> = ApiRequest::put(&api_endpoint.to_url())
        .json_body(&request);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_dimension_range_request() {
        let request = InsertDimensionRangeRequest::new("spreadsheet_token", serde_json::json!({}));
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
        assert!(request.inheritStyle.is_none());
    }
}
