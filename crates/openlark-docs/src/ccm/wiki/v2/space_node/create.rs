#![allow(unused_variables, unused_unsafe)]
use SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::{
    core::{
        api::ApiRequest,
        api::{ApiResponseTrait, BaseResponse, ResponseFormat},
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
///
/// 用于在指定的知识空间中创建新节点，支持多种文档类型和节点类型。
/// 可以创建文档、电子表格、思维笔记、多维表格等不同类型的节点。
///
/// # 支持的文档类型
/// - `doc`: 文档
/// - `sheet`: 电子表格
/// - `mindnote`: 思维笔记
/// - `bitable`: 多维表格
///
/// # 支持的节点类型
/// - `origin`: 正常节点
/// - `shortcut`: 快捷方式
///
/// # 使用示例
///
/// ```rust,no_run
/// use open_lark::prelude::*;
/// use open_lark::service::cloud_docs::wiki::v2::space_node::create::*;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = LarkClient::builder()
///         .app_id("your_app_id")
///         .app_secret("your_app_secret")
///         .build()?;
///
///     // 创建一个文档节点
///     let response = client.cloud_docs.wiki.v2.space_node
///         .create_space_node_builder()
///         .space_id("your_space_id")
///         .with_doc_type()
///         .parent_node_token("parent_node_token")
///         .with_origin_node()
///         .title("我的新文档")
///         .execute(&client.cloud_docs.wiki.v2.space_node)
///         .await?;
///
///     println!("创建成功，节点token: {}", response.data.node.node_token);
///     Ok(())
/// }
/// ```
#[derive(Debug, Serialize, Default)]
pub struct CreateSpaceNodeRequest {
    #[serde(skip)]
    api_request: ApiRequest,

    /// 知识空间ID
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
    /// 创建新的创建知识空间节点请求构建器
    pub fn builder() -> CreateSpaceNodeRequestBuilder {
        CreateSpaceNodeRequestBuilder::default()
    }

    /// 创建新的请求实例（已弃用，请使用builder()方法）
    #[deprecated(note = "请使用 builder() 方法替代")]
    pub fn new(space_id: impl ToString, obj_type: impl ToString) -> Self {
        Self {
            space_id: space_id.to_string(),
            obj_type: obj_type.to_string(),
            
        }
    }
}

/// 创建知识空间节点请求构建器
///
/// 提供流畅的API来构建创建知识空间节点的请求。
/// 支持链式调用，方便设置各种参数。
#[derive(Default)]
pub struct CreateSpaceNodeRequestBuilder {
    request: CreateSpaceNodeRequest,
}

impl CreateSpaceNodeRequestBuilder {
    /// 设置知识空间ID
    ///
    /// # 参数
    /// - `space_id`: 知识空间的唯一标识符
    pub fn space_id(mut self, space_id: impl ToString) -> Self {
        self.request.space_id = space_id.to_string();
        self
    }

    /// 设置文档类型为文档（doc）
    pub fn with_doc_type(mut self) -> Self {
        self.request.obj_type = "doc".to_string();
        self
    }

    /// 设置文档类型为电子表格（sheet）
    pub fn with_sheet_type(mut self) -> Self {
        self.request.obj_type = "sheet".to_string();
        self
    }

    /// 设置文档类型为思维笔记（mindnote）
    pub fn with_mindnote_type(mut self) -> Self {
        self.request.obj_type = "mindnote".to_string();
        self
    }

    /// 设置文档类型为多维表格（bitable）
    pub fn with_bitable_type(mut self) -> Self {
        self.request.obj_type = "bitable".to_string();
        self
    }

    /// 自定义文档类型
    ///
    /// # 参数
    /// - `obj_type`: 文档类型字符串
    pub fn obj_type(mut self, obj_type: impl ToString) -> Self {
        self.request.obj_type = obj_type.to_string();
        self
    }

    /// 设置父节点token
    ///
    /// # 参数
    /// - `parent_node_token`: 父节点的唯一标识符，创建根节点时可以不设置
    pub fn parent_node_token(mut self, parent_node_token: impl ToString) -> Self {
        self.request.parent_node_token = Some(parent_node_token.to_string());
        self
    }

    /// 设置节点类型为正常节点（origin）
    pub fn with_origin_node(mut self) -> Self {
        self.request.node_type = Some("origin".to_string());
        self
    }

    /// 设置节点类型为快捷方式（shortcut）
    pub fn with_shortcut_node(mut self) -> Self {
        self.request.node_type = Some("shortcut".to_string());
        self
    }

    /// 自定义节点类型
    ///
    /// # 参数
    /// - `node_type`: 节点类型字符串
    pub fn node_type(mut self, node_type: impl ToString) -> Self {
        self.request.node_type = Some(node_type.to_string());
        self
    }

    /// 设置文档标题
    ///
    /// # 参数
    /// - `title`: 文档标题
    pub fn title(mut self, title: impl ToString) -> Self {
        self.request.title = Some(title.to_string());
        self
    }

    /// 构建请求
    ///
    /// 返回配置好的CreateSpaceNodeRequest实例
    pub fn build(mut self) -> CreateSpaceNodeRequest {
        // 验证必填字段
        if self.request.space_id.is_empty() {
            panic!("space_id 是必填字段");
        }
        if self.request.obj_type.is_empty() {
            panic!("obj_type 是必填字段");
        }

        // 序列化请求体
        self.request.api_request.body = Some(openlark_core::api::RequestData::Json(&self.request))
            .unwrap_or_default();

        self.request
    }
}

/// 创建的节点信息
///
/// 包含新创建节点的详细信息，包括节点标识、类型、标题等。
#[derive(Debug, Deserialize)]
pub struct CreatedNode {
    /// 知识空间ID
    pub space_id: String,

    /// 节点token，节点的唯一标识符
    pub node_token: String,

    /// 文档类型
    pub obj_type: String,

    /// 父节点token
    pub parent_node_token: Option<String>,

    /// 节点类型
    pub node_type: Option<String>,

    /// 原始文档token（对于快捷方式类型的节点）
    pub obj_token: Option<String>,

    /// 文档标题
    pub title: Option<String>,
}

/// 创建知识空间节点响应
///
/// 包含创建操作的结果，主要是新创建节点的信息。
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
///
/// 在指定的知识空间中创建新节点。支持创建多种类型的文档节点，
/// 包括文档、电子表格、思维笔记和多维表格。
///
/// # 参数
/// - `request`: 创建节点的请求
/// - `config`: SDK配置
/// - `option`: 请求选项（可选）
///
/// # 返回值
/// 返回包含新创建节点信息的响应
///
/// # 错误处理
/// - 当space_id无效时返回错误
/// - 当obj_type不支持时返回错误
/// - 当parent_node_token不存在时返回错误
/// - 当权限不足时返回错误
pub async fn create_space_node(
    request: CreateSpaceNodeRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<CreateSpaceNodeResponse>> {
    let mut api_req = request.api_request;

    // 设置HTTP方法和路径
    api_req.set_http_method(Method::POST);
    api_req.set_api_path(EndpointBuilder::replace_param(
        WIKI_V2_SPACE_NODES,
        "space_id",
        &request.space_id,
    ));

    // 设置支持的访问令牌类型
    api_req.set_supported_access_token_types(vec![
        AccessTokenType::Tenant,
        AccessTokenType::User,
    ]);

    // 发送请求
    let api_resp = Transport::request(api_req, config, option).await?;

    Ok(api_resp)
}

// 实现可执行构建器宏，支持与服务集成
impl_executable_builder_owned!(
    CreateSpaceNodeRequestBuilder,
    super::cloud_docs::wiki::v2::space_node::SpaceNodeService,
    CreateSpaceNodeRequest,
    CreateSpaceNodeResponse,
    create,
);

#[cfg(test)]
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

    #[test]
    fn test_create_different_document_types() {
        // 测试创建文档
        let doc_request = CreateSpaceNodeRequest::builder()
            .space_id("spc123")
            .with_doc_type()
            .title("测试文档")
            .build();
        assert_eq!(doc_request.obj_type, "doc");

        // 测试创建电子表格
        let sheet_request = CreateSpaceNodeRequest::builder()
            .space_id("spc123")
            .with_sheet_type()
            .title("测试表格")
            .build();
        assert_eq!(sheet_request.obj_type, "sheet");

        // 测试创建思维笔记
        let mindnote_request = CreateSpaceNodeRequest::builder()
            .space_id("spc123")
            .with_mindnote_type()
            .title("测试思维笔记")
            .build();
        assert_eq!(mindnote_request.obj_type, "mindnote");

        // 测试创建多维表格
        let bitable_request = CreateSpaceNodeRequest::builder()
            .space_id("spc123")
            .with_bitable_type()
            .title("测试多维表格")
            .build();
        assert_eq!(bitable_request.obj_type, "bitable");
    }

    #[test]
    fn test_create_root_node() {
        let request = CreateSpaceNodeRequest::builder()
            .space_id("spcxxxxxx")
            .with_doc_type()
            .title("根节点文档")
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.obj_type, "doc");
        assert_eq!(request.parent_node_token, None); // 根节点没有父节点
        assert_eq!(request.title, Some("根节点文档".to_string()));
    }

    #[test]
    fn test_create_shortcut_node() {
        let request = CreateSpaceNodeRequest::builder()
            .space_id("spcxxxxxx")
            .with_doc_type()
            .parent_node_token("wikcnxxxxxx")
            .with_shortcut_node()
            .title("快捷方式文档")
            .build();

        assert_eq!(request.node_type, Some("shortcut".to_string()));
    }

    #[test]
    #[should_panic(expected = "space_id 是必填字段")]
    fn test_missing_space_id() {
        CreateSpaceNodeRequest::builder()
            .with_doc_type()
            .title("测试文档")
            .build();
    }

    #[test]
    #[should_panic(expected = "obj_type 是必填字段")]
    fn test_missing_obj_type() {
        CreateSpaceNodeRequest::builder()
            .space_id("spc123")
            .title("测试文档")
            .build();
    }

    #[test]
    fn test_custom_obj_type_and_node_type() {
        let request = CreateSpaceNodeRequest::builder()
            .space_id("spc123")
            .obj_type("custom_type")
            .node_type("custom_node_type")
            .build();

        assert_eq!(request.obj_type, "custom_type");
        assert_eq!(request.node_type, Some("custom_node_type".to_string()));
    }
}