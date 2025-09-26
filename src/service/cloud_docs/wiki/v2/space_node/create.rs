use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{cloud_docs::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 创建知识空间节点请求
#[derive(Debug, Serialize, Default)]
pub struct CreateSpaceNodeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id
    #[serde(skip)]
    space_id: String,
    /// 文档类型：doc(文档)、sheet(电子表格)、mindnote(思维笔记)、bitable(多维表格)
    obj_type: String,
    /// 父节点token，创建根节点时可以为空
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_node_token: Option<String>,
    /// 节点类型：origin(正常节点)、shortcut(快捷方式)
    #[serde(skip_serializing_if = "Option::is_none")]
    node_type: Option<String>,
    /// 文档标题
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl CreateSpaceNodeRequest {
    pub fn builder() -> CreateSpaceNodeRequestBuilder {
        CreateSpaceNodeRequestBuilder::default()
    }

    pub fn new(space_id: impl ToString, obj_type: impl ToString) -> Self {
        Self {
            space_id: space_id.to_string(),
            obj_type: obj_type.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CreateSpaceNodeRequestBuilder {
    request: CreateSpaceNodeRequest,
}

impl CreateSpaceNodeRequestBuilder {
    /// 知识空间id
    pub fn space_id(mut self, space_id: impl ToString) -> Self {
        self.request.space_id = space_id.to_string();
        self
    }

    /// 文档类型：doc(文档)、sheet(电子表格)、mindnote(思维笔记)、bitable(多维表格)
    pub fn obj_type(mut self, obj_type: impl ToString) -> Self {
        self.request.obj_type = obj_type.to_string();
        self
    }

    /// 创建文档类型节点
    pub fn with_doc_type(mut self) -> Self {
        self.request.obj_type = "doc".to_string();
        self
    }

    /// 创建电子表格类型节点
    pub fn with_sheet_type(mut self) -> Self {
        self.request.obj_type = "sheet".to_string();
        self
    }

    /// 创建思维笔记类型节点
    pub fn with_mindnote_type(mut self) -> Self {
        self.request.obj_type = "mindnote".to_string();
        self
    }

    /// 创建多维表格类型节点
    pub fn with_bitable_type(mut self) -> Self {
        self.request.obj_type = "bitable".to_string();
        self
    }

    /// 父节点token，创建根节点时可以为空
    pub fn parent_node_token(mut self, parent_node_token: impl ToString) -> Self {
        self.request.parent_node_token = Some(parent_node_token.to_string());
        self
    }

    /// 节点类型：origin(正常节点)、shortcut(快捷方式)
    pub fn node_type(mut self, node_type: impl ToString) -> Self {
        self.request.node_type = Some(node_type.to_string());
        self
    }

    /// 设置为正常节点
    pub fn with_origin_node(mut self) -> Self {
        self.request.node_type = Some("origin".to_string());
        self
    }

    /// 设置为快捷方式
    pub fn with_shortcut_node(mut self) -> Self {
        self.request.node_type = Some("shortcut".to_string());
        self
    }

    /// 文档标题
    pub fn title(mut self, title: impl ToString) -> Self {
        self.request.title = Some(title.to_string());
        self
    }

    pub fn build(mut self) -> CreateSpaceNodeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    CreateSpaceNodeRequestBuilder,
    crate::service::cloud_docs::wiki::v2::space_node::SpaceNodeService,
    CreateSpaceNodeRequest,
    CreateSpaceNodeResponse,
    create
);

/// 创建的节点信息
#[derive(Debug, Deserialize)]
pub struct CreatedNode {
    /// 知识空间id
    pub space_id: String,
    /// 节点token
    pub node_token: String,
    /// 文档类型
    pub obj_type: String,
    /// 父节点token
    pub parent_node_token: Option<String>,
    /// 节点类型
    pub node_type: Option<String>,
    /// 原始文档token
    pub obj_token: Option<String>,
    /// 文档标题
    pub title: Option<String>,
}

/// 创建知识空间节点响应
#[derive(Debug, Deserialize)]
pub struct CreateSpaceNodeResponse {
    /// 创建的节点信息
    pub node: CreatedNode,
}

impl ApiResponseTrait for CreateSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建知识空间节点
pub async fn create_space_node(
    request: CreateSpaceNodeRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreateSpaceNodeResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path =
        EndpointBuilder::replace_param(WIKI_V2_SPACE_NODES, "space_id", &request.space_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_create_space_node_request_builder() {
        let request = CreateSpaceNodeRequest::builder()
            .space_id("spcxxxxxx")
            .with_doc_type()
            .parent_node_token("wikcnxxxxxx")
            .with_origin_node()
            .title("我的文档")
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.parent_node_token, Some("wikcnxxxxxx".to_string()));
        assert_eq!(request.node_type, Some("origin".to_string()));
        assert_eq!(request.title, Some("我的文档".to_string()));
    }
}
