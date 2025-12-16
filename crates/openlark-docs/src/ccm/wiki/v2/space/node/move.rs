//! 此方法用于在Wiki内移动节点，支持跨知识空间移动。如果有子节点，会携带子节点一起移动。
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/move

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MoveNodeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_parent_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_space_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MoveNodeResponse {
    pub node_token: String,
}

impl ApiResponseTrait for MoveNodeResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct MoveNodeBuilder {
    api_req: ApiRequest<MoveNodeRequest>,
}

impl MoveNodeBuilder {
    pub fn new(space_id: impl ToString, node_token: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "wiki_v2_space_node_move".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = format!(
            "https://open.feishu.cn/open-apis/wiki/v2/spaces/{}/nodes/{}/move",
            space_id.to_string(),
            node_token.to_string()
        );
        builder.api_req.body = Some(MoveNodeRequest::default());
        builder
    }

    pub fn target_parent_token(mut self, target_parent_token: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.target_parent_token = Some(target_parent_token.to_string());
        }
        self
    }

    pub fn target_space_id(mut self, target_space_id: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.target_space_id = Some(target_space_id.to_string());
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
