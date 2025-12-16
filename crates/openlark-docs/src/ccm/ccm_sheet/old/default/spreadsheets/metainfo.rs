use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 获取表格元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetMetaRequest {
    /// 表格 token
    pub spreadsheet_token: String,
}

impl GetSpreadsheetMetaRequest {
    /// 创建新的 GetSpreadsheetMetaRequest
    pub fn new(spreadsheet_token: impl Into<String>) -> Self {
        Self {
            spreadsheet_token: spreadsheet_token.into(),
        }
    }
}

/// 获取表格元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpreadsheetMetaResponse {
    /// 表格标题
    pub title: String,
    /// 拥有者 ID
    pub owner_id: i64,
    /// 表格 token
    pub token: String,
    /// 表格 URL
    pub url: String,
    /// 表格版本号
    pub revision: i64,
}

impl ApiResponseTrait for GetSpreadsheetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取表格元数据
///
/// 获取表格的元数据，包括标题、拥有者、URL 等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet/get
pub async fn get_spreadsheet_meta(
    request: GetSpreadsheetMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetSpreadsheetMetaResponse>> {
    let api_endpoint = CcmSheetApiOld::Metainfo(request.spreadsheet_token.clone());
    let mut api_request: ApiRequest<GetSpreadsheetMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_spreadsheet_meta_request() {
        let request = GetSpreadsheetMetaRequest::new("spreadsheet_token");
        assert_eq!(request.spreadsheet_token, "spreadsheet_token");
    }
}
