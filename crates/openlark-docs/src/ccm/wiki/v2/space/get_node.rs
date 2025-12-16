/// 获取知识空间节点信息
///
/// 获取知识空间节点信息
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/get_node
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use crate::ccm::wiki::v2::space::node::NodeInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNodeParams {
    pub token: String,
    pub obj_type: Option<String>,
}

impl GetNodeParams {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            obj_type: None,
        }
    }

    pub fn obj_type(mut self, obj_type: impl Into<String>) -> Self {
        self.obj_type = Some(obj_type.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNodeResponse {
    pub node: Option<NodeInfo>,
}

impl ApiResponseTrait for GetNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn get_node(
    params: GetNodeParams,
    config: &Config,
) -> SDKResult<Response<GetNodeResponse>> {
    let mut api_request: ApiRequest<GetNodeResponse> = ApiRequest::get("/open-apis/wiki/v2/spaces/get_node");
    
    api_request = api_request.query_param("token", &params.token);
    if let Some(obj_type) = params.obj_type {
        api_request = api_request.query_param("obj_type", &obj_type);
    }

    Transport::request(api_request, config, None).await
}
