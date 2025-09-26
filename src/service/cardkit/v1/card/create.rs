use reqwest::Method;
use serde::Deserialize;
use serde_json::json;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cardkit::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
    service::cardkit::v1::models::{Card, UserIdType},
};

use super::CardService;

/// 创建卡片实体请求
#[derive(Default, Clone)]
pub struct CreateCardRequest {
    pub api_req: ApiRequest,
    /// 卡片标题
    pub title: Option<String>,
    /// 卡片描述
    pub description: Option<String>,
    /// 卡片JSON内容
    pub card_json: Option<serde_json::Value>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl CreateCardRequest {
    /// 创建创建卡片请求的构建器
    pub fn builder() -> CreateCardRequestBuilder {
        CreateCardRequestBuilder::default()
    }
}

/// 创建卡片实体请求构建器
#[derive(Default)]
pub struct CreateCardRequestBuilder {
    request: CreateCardRequest,
}

impl CreateCardRequestBuilder {
    /// 设置卡片标题
    pub fn title(mut self, title: impl ToString) -> Self {
        self.request.title = Some(title.to_string());
        self
    }

    /// 设置卡片描述
    pub fn description(mut self, description: impl ToString) -> Self {
        self.request.description = Some(description.to_string());
        self
    }

    /// 设置卡片JSON内容
    pub fn card_json(mut self, card_json: serde_json::Value) -> Self {
        self.request.card_json = Some(card_json);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> CreateCardRequest {
        // 构建查询参数
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_req
                .query_params
                .insert("user_id_type", user_id_type.to_string());
        }

        // 构建请求体
        let mut body = json!({});

        if let Some(ref title) = self.request.title {
            body["title"] = json!(title);
        }

        if let Some(ref description) = self.request.description {
            body["description"] = json!(description);
        }

        if let Some(ref card_json) = self.request.card_json {
            body["card_json"] = card_json.clone();
        }

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 创建卡片实体响应数据
#[derive(Debug, Deserialize)]
pub struct CreateCardResponseData {
    /// 创建的卡片信息
    pub card: Card,
}

/// 创建卡片实体响应
#[derive(Debug, Deserialize)]
pub struct CreateCardResponse {
    /// 响应数据
    pub data: CreateCardResponseData,
}

impl ApiResponseTrait for CreateCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CardService {
    /// 创建卡片实体
    ///
    /// 创建一个新的卡片实体。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 创建卡片请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 创建的卡片信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.cardkit.v1.card.create(
    ///     CreateCardRequest::builder()
    ///         .title("示例卡片")
    ///         .description("这是一个示例卡片")
    ///         .card_json(serde_json::json!({"elements": []}))
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/cardkit-v1/card/create>
    pub async fn create(
        &self,
        request: CreateCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateCardResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = CARDKIT_V1_CARDS.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    CreateCardRequestBuilder,
    CardService,
    CreateCardRequest,
    BaseResponse<CreateCardResponse>,
    create
);
