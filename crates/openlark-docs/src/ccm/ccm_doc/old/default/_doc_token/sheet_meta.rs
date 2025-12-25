//! 获取旧版文档中的电子表格元数据
//!
//! docPath: /document/ukTMukTMukTM/uADOzUjLwgzM14CM4MTN
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/document/obtain-sheet-meta-info-in-doc

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocSheetMetaReq {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetDocSheetMetaResponse {
    /// sheet 文档 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// doc 下的 sheet 列表
    pub sheets: Vec<DocSheetMeta>,
}

/// doc 下的 sheet 元数据
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DocSheetMeta {
    /// sheet 的 id
    #[serde(rename = "sheetId")]
    pub sheet_id: String,
    /// sheet 的标题
    pub title: String,
    /// 该 sheet 的位置
    pub index: i32,
    /// 该 sheet 的行数
    #[serde(rename = "rowCount")]
    pub row_count: i32,
    /// 该 sheet 的列数
    #[serde(rename = "columnCount")]
    pub column_count: i32,
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
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取旧版文档中的电子表格元数据")
    }
}
