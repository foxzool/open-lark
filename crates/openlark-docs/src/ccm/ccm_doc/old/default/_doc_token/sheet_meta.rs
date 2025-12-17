//! 获取旧版文档中的电子表格元数据
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocSheetMetaReq {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocSheetMetaResponse {
    #[serde(rename = "sheetId")]
    pub sheet_id: i32,
    pub title: String,
    #[serde(rename = "rowCount")]
    pub row_count: i32,
    #[serde(rename = "colCount")]
    pub col_count: i32,
}

impl ApiResponseTrait for GetDocSheetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取旧版文档中的电子表格元数据请求
pub struct GetDocSheetMetaRequest {
    config: Config,
    doc_token: String,
}

impl GetDocSheetMetaRequest {
    pub fn new(config: Config, doc_token: impl Into<String>) -> Self {
        Self {
            config,
            doc_token: doc_token.into(),
        }
    }

    pub async fn send(self) -> SDKResult<GetDocSheetMetaResponse> {
        use crate::common::api_endpoints::CcmDocApiOld;

        let api_request: ApiRequest<GetDocSheetMetaResponse> =
            ApiRequest::get(&CcmDocApiOld::SheetMeta(self.doc_token).to_url());
        let response: Response<GetDocSheetMetaResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("response", "响应数据为空")
        })
    }
}
