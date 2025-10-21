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
    service::cardkit::v1::models::{Card, UserIdType},
};

use super::CardService;

/// 全量更新卡片实体请求
#[derive(Default, Clone)]
pub struct UpdateCardRequest {
    pub api_req: ApiRequest,
    /// 卡片ID
    pub card_id: String,
    /// 卡片标题
    pub title: Option<String>,
    /// 卡片描述
    pub description: Option<String>,
    /// 卡片JSON内容
    pub card_json: Option<serde_json::Value>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl UpdateCardRequest {
    /// 创建全量更新卡片请求的构建器
    pub fn builder(card_id: impl ToString) -> UpdateCardRequestBuilder {
        UpdateCardRequestBuilder {
            request: UpdateCardRequest {
                card_id: card_id.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 全量更新卡片实体请求构建器
pub struct UpdateCardRequestBuilder {
    request: UpdateCardRequest,
}

impl UpdateCardRequestBuilder {
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
    pub fn build(mut self) -> UpdateCardRequest {
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

/// 全量更新卡片实体响应数据
#[derive(Debug, Deserialize)]
pub struct UpdateCardResponseData {
    /// 更新后的卡片信息
    pub card: Card,
}

/// 全量更新卡片实体响应
#[derive(Debug, Deserialize)]
pub struct UpdateCardResponse {
    /// 响应数据
    pub data: UpdateCardResponseData,
}

impl ApiResponseTrait for UpdateCardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CardService {
    /// 全量更新卡片实体
    ///
    /// 全量更新指定卡片的信息。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 全量更新卡片请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 更新后的卡片信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.cardkit.v1.card.update(
    ///     UpdateCardRequest::builder("card_id")
    ///         .title("更新后的标题")
    ///         .description("更新后的描述")
    ///         .card_json(serde_json::json!({"elements": []}))
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/cardkit-v1/card/update>
    pub async fn update(
        &self,
        request: UpdateCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateCardResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::PUT;
        api_req.api_path =
            EndpointBuilder::replace_param(CARDKIT_V1_CARD_UPDATE, "card_id", &request.card_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    UpdateCardRequestBuilder,
    CardService,
    UpdateCardRequest,
    BaseResponse<UpdateCardResponse>,
    update
);
