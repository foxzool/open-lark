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
    service::cardkit::v1::models::{CardSettings, UserIdType},
};

use super::CardService;

/// 更新卡片配置请求
#[derive(Default, Clone)]
pub struct UpdateCardSettingsRequest {
    pub api_req: ApiRequest,
    /// 卡片ID
    pub card_id: String,
    /// 是否启用交互
    pub enable_interaction: Option<bool>,
    /// 卡片主题
    pub theme: Option<String>,
    /// 自定义配置
    pub custom_config: Option<serde_json::Value>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
}

impl UpdateCardSettingsRequest {
    /// 创建更新卡片配置请求的构建器
    pub fn builder(card_id: impl ToString) -> UpdateCardSettingsRequestBuilder {
        UpdateCardSettingsRequestBuilder {
            request: UpdateCardSettingsRequest {
                card_id: card_id.to_string(),
                ..Default::default()
            },
        }
    }
}

/// 更新卡片配置请求构建器
pub struct UpdateCardSettingsRequestBuilder {
    request: UpdateCardSettingsRequest,
}

impl UpdateCardSettingsRequestBuilder {
    /// 设置是否启用交互
    pub fn enable_interaction(mut self, enable_interaction: bool) -> Self {
        self.request.enable_interaction = Some(enable_interaction);
        self
    }

    /// 设置卡片主题
    pub fn theme(mut self, theme: impl ToString) -> Self {
        self.request.theme = Some(theme.to_string());
        self
    }

    /// 设置自定义配置
    pub fn custom_config(mut self, custom_config: serde_json::Value) -> Self {
        self.request.custom_config = Some(custom_config);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.request.user_id_type = Some(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(mut self) -> UpdateCardSettingsRequest {
        // 构建查询参数
        if let Some(user_id_type) = &self.request.user_id_type {
            self.request
                .api_req
                .query_params
                .insert("user_id_type", user_id_type.to_string());
        }

        // 构建请求体
        let mut body = json!({});

        if let Some(enable_interaction) = self.request.enable_interaction {
            body["enable_interaction"] = json!(enable_interaction);
        }

        if let Some(ref theme) = self.request.theme {
            body["theme"] = json!(theme);
        }

        if let Some(ref custom_config) = self.request.custom_config {
            body["custom_config"] = custom_config.clone();
        }

        self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request
    }
}

/// 更新卡片配置响应数据
#[derive(Debug, Deserialize)]
pub struct UpdateCardSettingsResponseData {
    /// 卡片配置信息
    pub settings: CardSettings,
}

/// 更新卡片配置响应
#[derive(Debug, Deserialize)]
pub struct UpdateCardSettingsResponse {
    /// 响应数据
    pub data: UpdateCardSettingsResponseData,
}

impl ApiResponseTrait for UpdateCardSettingsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CardService {
    /// 更新卡片配置
    ///
    /// 更新指定卡片的配置信息。身份验证方式为应用身份验证
    ///
    /// # 参数
    /// - `request`: 更新卡片配置请求
    /// - `option`: 请求选项
    ///
    /// # 返回
    /// 更新后的卡片配置信息
    ///
    /// # 示例
    /// ```rust,ignore
    /// let response = client.cardkit.v1.card.settings(
    ///     UpdateCardSettingsRequest::builder("card_id")
    ///         .enable_interaction(true)
    ///         .theme("dark")
    ///         .build(),
    ///     None
    /// ).await?;
    /// ```
    ///
    /// 参考: <https://open.feishu.cn/document/cardkit-v1/card/settings>
    pub async fn settings(
        &self,
        request: UpdateCardSettingsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateCardSettingsResponse>> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::PATCH;
        api_req.api_path =
            EndpointBuilder::replace_param(CARDKIT_V1_CARD_SETTINGS, "card_id", &request.card_id);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

// 应用ExecutableBuilder宏
impl_executable_builder_owned!(
    UpdateCardSettingsRequestBuilder,
    CardService,
    UpdateCardSettingsRequest,
    BaseResponse<UpdateCardSettingsResponse>,
    settings
);
