/// 搜索云文档
///
/// 根据搜索条件进行文档搜索。
/// docPath: /document/ukTMukTMukTM/ugDM4UjL4ADO14COwgTN
/// doc: https://open.feishu.cn/document/ukTMukTMukTM/ugDM4UjL4ADO14COwgTN
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectReq {
    /// 指定搜索的关键字
    pub search_key: Option<String>,
    /// 指定搜索返回的文件数量。取值范围为 ` [0,50] `
    pub count: Option<i32>,
    /// 指定搜索的偏移量，最小为 0，且 offset + count < 200
    pub offset: Option<i32>,
    /// 文件所有者的 Open ID
    pub owner_ids: Option<Vec<String>>,
    /// 文件所在群的 ID
    pub chat_ids: Option<Vec<String>>,
    /// 文件类型（doc/sheet/slides/bitable/mindnote/file）
    pub docs_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchObjectResponse {
    #[serde(default)]
    pub docs_entities: Vec<DocsEntity>,
    #[serde(default)]
    pub total: i32,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsEntity {
    pub docs_token: String,
    pub docs_type: String,
    pub title: String,
    pub owner_id: String,
    pub create_time: i64,
}

impl ApiResponseTrait for SearchObjectResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索云文档请求（旧版）
pub struct SearchObjectRequest {
    config: Config,
    req: SearchObjectReq,
}

impl SearchObjectRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            req: SearchObjectReq {
                search_key: None,
                count: None,
                offset: None,
                owner_ids: None,
                chat_ids: None,
                docs_types: None,
            },
        }
    }

    pub fn search_key(mut self, search_key: impl Into<String>) -> Self {
        self.req.search_key = Some(search_key.into());
        self
    }

    pub fn count(mut self, count: i32) -> Self {
        self.req.count = Some(count);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.req.offset = Some(offset);
        self
    }

    pub fn owner_ids(mut self, owner_ids: Vec<String>) -> Self {
        self.req.owner_ids = Some(owner_ids);
        self
    }

    pub fn chat_ids(mut self, chat_ids: Vec<String>) -> Self {
        self.req.chat_ids = Some(chat_ids);
        self
    }

    pub fn docs_types(mut self, docs_types: Vec<String>) -> Self {
        self.req.docs_types = Some(docs_types);
        self
    }

    pub async fn send(self) -> SDKResult<SearchObjectResponse> {
        use crate::common::api_endpoints::CcmDocsApiOld;
        if self
            .req
            .search_key
            .as_deref()
            .unwrap_or_default()
            .is_empty()
        {
            return Err(openlark_core::error::validation_error(
                "search_key",
                "search_key 不能为空",
            ));
        }
        if let Some(count) = self.req.count {
            if !(0..=50).contains(&count) {
                return Err(openlark_core::error::validation_error(
                    "count",
                    "count 必须在 [0,50] 范围内",
                ));
            }
        }
        if let Some(offset) = self.req.offset {
            if offset < 0 {
                return Err(openlark_core::error::validation_error(
                    "offset",
                    "offset 必须 >= 0",
                ));
            }
        }
        if let (Some(offset), Some(count)) = (self.req.offset, self.req.count) {
            if offset + count >= 200 {
                return Err(openlark_core::error::validation_error(
                    "offset",
                    "要求 offset + count < 200",
                ));
            }
        }

        let api_request: ApiRequest<SearchObjectResponse> =
            ApiRequest::post(&CcmDocsApiOld::SearchObject.to_url())
                .body(serialize_params(&self.req, "搜索云文档")?);

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "搜索云文档")
    }
}
