use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

/// 查询导入结果请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportResultRequest {
    /// 任务票据
    pub ticket: String,
}

impl GetImportResultRequest {
    /// 创建新的 GetImportResultRequest
    pub fn new(ticket: impl Into<String>) -> Self {
        Self {
            ticket: ticket.into(),
        }
    }
}

/// 查询导入结果响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetImportResultResponse {
    /// 结果状态，0：成功
    pub code: i32,
    /// 导入后的表格 token
    pub url: Option<String>,
    /// 错误信息
    pub msg: Option<String>,
}

impl ApiResponseTrait for GetImportResultResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询导入结果
///
/// 查询导入结果。
/// docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/import-export/query-import-result
pub async fn get_import_result(
    request: GetImportResultRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetImportResultResponse>> {
    let api_endpoint = CcmSheetApiOld::ImportResult;
    let mut api_request: ApiRequest<GetImportResultResponse> = ApiRequest::get(&api_endpoint.to_url())
        .query("ticket", &request.ticket);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_import_result_request() {
        let request = GetImportResultRequest::new("ticket");
        assert_eq!(request.ticket, "ticket");
    }
}
