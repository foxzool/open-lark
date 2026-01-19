//! 获取旧版文档中的电子表格元数据
//!
//! docPath: /document/ukTMukTMukTM/uADOzUjLwgzM14CM4MTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uADOzUjLwgzM14CM4MTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
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
    #[serde(default)]
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

    pub async fn execute(self) -> SDKResult<GetDocSheetMetaResponse> {
            self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
        }

        pub async fn execute_with_options(
            self,
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<GetDocSheetMetaResponse> {

        use crate::common::api_endpoints::CcmDocApiOld;
        validate_required!(self.doc_token, "doc_token 不能为空");

        let api_request: ApiRequest<GetDocSheetMetaResponse> =
            ApiRequest::get(&CcmDocApiOld::SheetMeta(self.doc_token).to_url());
        
            let response = Transport::request(api_request, &self.config, Some(option)).await?;
            extract_response_data(response, "获取文档表格元数据")
    }
}
