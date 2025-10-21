use reqwest::Method;
use serde::Deserialize;
use serde_json::json;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::{cardkit::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::cardkit::v1::models::{CardElement, UserIdType},
};

use super::CardElementService;

/// 新增组件请求
#[derive(Default, Clone)]
pub struct CreateElementRequest {
    pub api_req: ApiRequest,
    /// 卡片ID
    pub card_id: String,
    /// 组件类型
    pub element_type: Option<String>,
    /// 组件内容
    pub content: Option<serde_json::Value>,
    /// 组件属性
    pub properties: Option<serde_json::Value>,
    /// 父组件ID
    pub parent_id: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl CreateElementRequest {
    /// 创建新增组件请求的构建器
    pub fn builder(card_id: impl ToString) -> CreateElementRequestBuilder {
        CreateElementRequestBuilder {
            request: CreateElementRequest {
                card_id: card_id.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 新增组件请求构建器
pub struct CreateElementRequestBuilder {
    request: CreateElementRequest,
}

impl CreateElementRequestBuilder {
    /// 设置组件类型
    pub fn element_type(mut self, element_type: impl ToString) -> Self {
        self.request.element_type = Some(element_type.to_string());
        self
    }

    /// 设置组件内容
    pub fn content(mut self, content: serde_json::Value) -> Self {
        self.request.content = Some(content);
        self
    }

    /// 设置组件属性
    pub fn properties(mut self, properties: serde_json::Value) -> Self {
        self.request.properties = Some(properties);
        self
    }

    /// 设置父组件ID
    pub fn parent_id(mut self, parent_id: impl ToString) -> Self {
        self.request.parent_id = Some(parent_id.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> CreateElementRequest {
        // 构建查询参数
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_req
                .query_params
                .insert("user_id_type", user_id_type.to_string());
        }

        // 构建请求体
        let mut body = json!({});

        if let Some(ref element_type) = self.request.element_type {
            body["element_type"] = json!(element_type);
        }

        if let Some(ref content) = self.request.content {
            body["content"] = content.clone();
        }

        if let Some(ref properties) = self.request.properties {
            body["properties"] = properties.clone();
        }

        if let Some(ref parent_id) = self.request.parent_id {
            body["parent_id"] = json!(parent_id);
        }

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 新增组件响应数据
#[derive(Debug, Deserialize)]
pub struct CreateElementResponseData {
    /// 创建的组件信息
    pub element: CardElement,
}

/// 新增组件响应
#[derive(Debug, Deserialize)]
pub struct CreateElementResponse {
    /// 响应数据
    pub data: CreateElementResponseData,
}

impl ApiResponseTrait for CreateElementResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CardElementService {
    /// 新增组件
    ///
    /// 在指定卡片中新增一个组件。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 新增组件请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 创建的组件信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.cardkit.v1.card_element.create(
    ///     CreateElementRequest::builder("card_id")
    ///         .element_type("text")
    ///         .content(serde_json::json!({"text": "Hello World"}))
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/cardkit-v1/card-element/create>
    pub async fn create(
        &self,
        request: CreateElementRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateElementResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path =
            EndpointBuilder::replace_param(CARDKIT_V1_CARD_ELEMENTS, "card_id", &request.card_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    CreateElementRequestBuilder,
    CardElementService,
    CreateElementRequest,
    BaseResponse<CreateElementResponse>,
    create
);
