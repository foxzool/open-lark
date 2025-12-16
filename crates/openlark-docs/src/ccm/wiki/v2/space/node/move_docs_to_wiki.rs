//! 该接口允许移动云空间文档至知识空间，并挂载在指定位置
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/move_docs_to_wiki

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MoveDocsToWikiRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_wiki_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MoveDocsToWikiResponse {
    pub task_id: String,
    pub applied: bool,
}

impl ApiResponseTrait for MoveDocsToWikiResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct MoveDocsToWikiBuilder {
    api_req: ApiRequest<MoveDocsToWikiRequest>,
}

impl MoveDocsToWikiBuilder {
    pub fn new(space_id: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "wiki_v2_space_node_move_docs_to_wiki".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/wiki/v2/spaces/{}/nodes/move_docs_to_wiki",
            space_id.to_string()
        );
        builder.api_req.body = Some(MoveDocsToWikiRequest::default());
        builder
    }

    pub fn parent_wiki_token(mut self, parent_wiki_token: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.parent_wiki_token = Some(parent_wiki_token.to_string());
        }
        self
    }

    pub fn obj_type(mut self, obj_type: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.obj_type = Some(obj_type.to_string());
        }
        self
    }

    pub fn obj_token(mut self, obj_token: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.obj_token = Some(obj_token.to_string());
        }
        self
    }

    pub fn apply(mut self, apply: bool) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.apply = Some(apply);
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
