use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 移动云空间文档至知识空间请求
#[derive(Debug, Serialize, Default)]
pub struct MoveDocsToWikiRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id
    space_id: String,
    /// 移动的云空间文档token列表
    obj_tokens: Vec<String>,
    /// 目标父节点token，不填时为根节点
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_node_token: Option<String>,
}

impl MoveDocsToWikiRequest {
    pub fn builder() -> MoveDocsToWikiRequestBuilder {
        MoveDocsToWikiRequestBuilder::default()
    }

    pub fn new(space_id: impl ToString, obj_tokens: Vec<String>) -> Self {
        Self {
            space_id: space_id.to_string(),
            obj_tokens,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct MoveDocsToWikiRequestBuilder {
    request: MoveDocsToWikiRequest,
}

impl MoveDocsToWikiRequestBuilder {
    /// 知识空间id
    pub fn space_id(mut self, space_id: impl ToString) -> Self {
        self.request.space_id = space_id.to_string();
        self
    }

    /// 移动的云空间文档token列表
    pub fn obj_tokens(mut self, obj_tokens: Vec<String>) -> Self {
        self.request.obj_tokens = obj_tokens;
        self
    }

    /// 添加单个文档token
    pub fn add_obj_token(mut self, obj_token: impl ToString) -> Self {
        self.request.obj_tokens.push(obj_token.to_string());
        self
    }

    /// 批量添加文档tokens
    pub fn add_obj_tokens(mut self, tokens: Vec<impl ToString>) -> Self {
        for token in tokens {
            self.request.obj_tokens.push(token.to_string());
        }
        self
    }

    /// 目标父节点token
    pub fn parent_node_token(mut self, parent_node_token: impl ToString) -> Self {
        self.request.parent_node_token = Some(parent_node_token.to_string());
        self
    }

    /// 移动到根节点
    pub fn move_to_root(mut self) -> Self {
        self.request.parent_node_token = None;
        self
    }

    pub fn build(mut self) -> MoveDocsToWikiRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 移动任务信息
#[derive(Debug, Deserialize)]
pub struct MoveTask {
    /// 任务id
    pub task_id: String,
}

/// 移动云空间文档至知识空间响应
#[derive(Debug, Deserialize)]
pub struct MoveDocsToWikiResponse {
    /// 任务信息
    pub task: MoveTask,
}

impl ApiResponseTrait for MoveDocsToWikiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动云空间文档至知识空间
pub async fn move_docs_to_wiki(
    request: MoveDocsToWikiRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<MoveDocsToWikiResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = WIKI_V2_TASKS_MOVE_DOCS_TO_WIKI.to_string();
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_move_docs_to_wiki_request_builder() {
        let request = MoveDocsToWikiRequest::builder()
            .space_id("spcxxxxxx")
            .add_obj_token("doccnxxxxxx")
            .add_obj_token("shtcnxxxxxx")
            .parent_node_token("wikcnxxxxxx")
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.obj_tokens.len(), 2);
        assert_eq!(request.obj_tokens[0], "doccnxxxxxx");
        assert_eq!(request.obj_tokens[1], "shtcnxxxxxx");
        assert_eq!(request.parent_node_token, Some("wikcnxxxxxx".to_string()));
    }

    #[test]
    fn test_move_to_root() {
        let request = MoveDocsToWikiRequest::builder()
            .space_id("spcxxxxxx")
            .obj_tokens(vec!["doccnxxxxxx".to_string(), "shtcnxxxxxx".to_string()])
            .move_to_root()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.obj_tokens.len(), 2);
        assert_eq!(request.parent_node_token, None);
    }
}
