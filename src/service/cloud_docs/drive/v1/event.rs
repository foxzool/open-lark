use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

/// 事件订阅服务
pub struct EventService {
    config: Config,
}

impl EventService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 订阅云文档事件
    ///
    /// 该接口用于订阅云文档的相关事件。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/event-subscription/create>
    pub async fn subscribe_file_events(
        &self,
        request: SubscribeFileEventsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SubscribeFileEventsRespData>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: DRIVE_V1_FILES_SUBSCRIBE.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 查询云文档事件订阅状态
    ///
    /// 该接口用于查询云文档的事件订阅状态。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/event-subscription/get>
    pub async fn get_file_subscription(
        &self,
        request: GetFileSubscriptionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetFileSubscriptionRespData>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: DRIVE_V1_FILE_SUBSCRIPTIONS
                .replace("{}", &request.file_token)
                .replace("{}", &request.subscription_id),
            ..Default::default()
        };
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 取消云文档事件订阅
    ///
    /// 该接口用于取消云文档的事件订阅。
    ///
    /// <https://open.feishu.cn/document/server-docs/docs/drive-v1/event-subscription/delete>
    pub async fn unsubscribe_file_events(
        &self,
        request: UnsubscribeFileEventsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UnsubscribeFileEventsRespData>> {
        let mut api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: DRIVE_V1_FILE_SUBSCRIPTIONS
                .replace("{}", &request.file_token)
                .replace("{}", &request.subscription_id),
            ..Default::default()
        };
        api_req.supported_access_token_types = vec![AccessTokenType::User, AccessTokenType::Tenant];

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 订阅云文档事件请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscribeFileEventsRequest {
    /// 文件token
    pub file_token: String,
    /// 文件类型
    pub file_type: String,
    /// 事件类型列表
    pub event_types: Vec<String>,
}

impl SubscribeFileEventsRequest {
    pub fn builder() -> SubscribeFileEventsRequestBuilder {
        SubscribeFileEventsRequestBuilder::default()
    }

    pub fn new(
        file_token: impl Into<String>,
        file_type: impl Into<String>,
        event_types: Vec<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            file_type: file_type.into(),
            event_types,
        }
    }
}

/// 订阅云文档事件请求构建器
#[derive(Default)]
pub struct SubscribeFileEventsRequestBuilder {
    request: SubscribeFileEventsRequest,
}

impl SubscribeFileEventsRequestBuilder {
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.request.file_token = file_token.into();
        self
    }

    pub fn file_type(mut self, file_type: impl Into<String>) -> Self {
        self.request.file_type = file_type.into();
        self
    }

    pub fn event_types(mut self, event_types: Vec<String>) -> Self {
        self.request.event_types = event_types;
        self
    }

    pub fn add_event_type(mut self, event_type: impl Into<String>) -> Self {
        self.request.event_types.push(event_type.into());
        self
    }

    pub fn build(self) -> SubscribeFileEventsRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    SubscribeFileEventsRequestBuilder,
    EventService,
    SubscribeFileEventsRequest,
    BaseResponse<SubscribeFileEventsRespData>,
    subscribe_file_events
);

/// 订阅云文档事件响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeFileEventsRespData {
    /// 订阅ID
    pub subscription_id: String,
    /// 订阅类型
    pub subscription_type: String,
}

impl ApiResponseTrait for SubscribeFileEventsRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询云文档事件订阅状态请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetFileSubscriptionRequest {
    /// 文件token
    pub file_token: String,
    /// 订阅ID
    pub subscription_id: String,
}

impl GetFileSubscriptionRequest {
    pub fn builder() -> GetFileSubscriptionRequestBuilder {
        GetFileSubscriptionRequestBuilder::default()
    }

    pub fn new(file_token: impl Into<String>, subscription_id: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            subscription_id: subscription_id.into(),
        }
    }
}

/// 查询云文档事件订阅状态请求构建器
#[derive(Default)]
pub struct GetFileSubscriptionRequestBuilder {
    request: GetFileSubscriptionRequest,
}

impl GetFileSubscriptionRequestBuilder {
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.request.file_token = file_token.into();
        self
    }

    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.request.subscription_id = subscription_id.into();
        self
    }

    pub fn build(self) -> GetFileSubscriptionRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    GetFileSubscriptionRequestBuilder,
    EventService,
    GetFileSubscriptionRequest,
    BaseResponse<GetFileSubscriptionRespData>,
    get_file_subscription
);

/// 查询云文档事件订阅状态响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileSubscriptionRespData {
    /// 订阅信息
    pub subscription: FileSubscription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSubscription {
    /// 订阅ID
    pub subscription_id: String,
    /// 文件token
    pub file_token: String,
    /// 文件类型
    pub file_type: String,
    /// 事件类型列表
    pub event_types: Vec<String>,
    /// 订阅状态
    pub is_active: bool,
}

impl ApiResponseTrait for GetFileSubscriptionRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 取消云文档事件订阅请求参数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnsubscribeFileEventsRequest {
    /// 文件token
    pub file_token: String,
    /// 订阅ID
    pub subscription_id: String,
}

impl UnsubscribeFileEventsRequest {
    pub fn builder() -> UnsubscribeFileEventsRequestBuilder {
        UnsubscribeFileEventsRequestBuilder::default()
    }

    pub fn new(file_token: impl Into<String>, subscription_id: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            subscription_id: subscription_id.into(),
        }
    }
}

/// 取消云文档事件订阅请求构建器
#[derive(Default)]
pub struct UnsubscribeFileEventsRequestBuilder {
    request: UnsubscribeFileEventsRequest,
}

impl UnsubscribeFileEventsRequestBuilder {
    pub fn file_token(mut self, file_token: impl Into<String>) -> Self {
        self.request.file_token = file_token.into();
        self
    }

    pub fn subscription_id(mut self, subscription_id: impl Into<String>) -> Self {
        self.request.subscription_id = subscription_id.into();
        self
    }

    pub fn build(self) -> UnsubscribeFileEventsRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    UnsubscribeFileEventsRequestBuilder,
    EventService,
    UnsubscribeFileEventsRequest,
    BaseResponse<UnsubscribeFileEventsRespData>,
    unsubscribe_file_events
);

/// 取消云文档事件订阅响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeFileEventsRespData {
    /// 操作结果
    pub result: bool,
}

impl ApiResponseTrait for UnsubscribeFileEventsRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
