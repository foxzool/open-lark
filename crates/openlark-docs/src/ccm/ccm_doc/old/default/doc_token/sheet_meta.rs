use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmDocApiOld;

/// 获取旧版文档中的电子表格元数据请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocSheetMetaRequest {
    /// 文档 token
    pub doc_token: String,
}

impl GetDocSheetMetaRequest {
    /// 创建新的 GetDocSheetMetaRequest
    pub fn new(doc_token: impl Into<String>) -> Self {
        Self {
            doc_token: doc_token.into(),
        }
    }
}

/// 获取旧版文档中的电子表格元数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDocSheetMetaResponse {
    /// 电子表格信息列表
    pub sheets: Vec<SheetInfo>,
}

/// 电子表格信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetInfo {
    /// 工作表 ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 行数
    pub row_count: i32,
    /// 列数
    pub column_count: i32,
}

impl ApiResponseTrait for GetDocSheetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档中的电子表格元数据
///
/// 根据 docToken 获取文档中的电子表格的元数据。
/// docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc
pub async fn get_doc_sheet_meta(
    request: GetDocSheetMetaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetDocSheetMetaResponse>> {
    let api_endpoint = CcmDocApiOld::SheetMeta(request.doc_token.clone());
    let mut api_request: ApiRequest<GetDocSheetMetaResponse> = ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_doc_sheet_meta_request() {
        let request = GetDocSheetMetaRequest::new("doc_token");
        assert_eq!(request.doc_token, "doc_token");
    }
}
