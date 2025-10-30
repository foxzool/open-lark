use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
/// 事件订阅服务,
pub struct EventService {
    config: Config}
impl EventService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 订阅云文档事件,
    ///,
/// 该接口用于订阅云文档的相关事件。,
    ///,
/// # API文档,
    ///,
/// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/combined_create,
    pub async fn subscribe_file_events(
        &self,
        request: SubscribeFileEventsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SubscribeFileEventsRespData>> {,
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: DRIVE_V1_FILES_SUBSCRIBE.to_string(),
            supported_access_token_types: vec![AccessTokenType::User, AccessTokenType::Tenant]
            body: serde_json::to_vec(&request)?,
            ..Default::default()};

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 查询云文档事件订阅状态,
    ///,
/// 该接口用于查询云文档的事件订阅状态。,
    ///,
/// # API文档,
    ///,
/// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/role/get,
    pub async fn get_file_subscription(
        &self,
        request: GetFileSubscriptionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetFileSubscriptionRespData>> {,
let mut api_req = ApiRequest {,
            http_method: Method::GET,
            api_path: DRIVE_V1_FILE_SUBSCRIPTIONS
                .replace("{}", &request.file_token)
                .replace()
            ..Default::default()
};
api_req
            .set_supported_access_token_types(vec![AccessTokenType::User, AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 取消云文档事件订阅,
    ///,
/// 该接口用于取消云文档的事件订阅。,
    ///,
/// # API文档,
    ///,
/// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job_requirement/delete,
    pub async fn unsubscribe_file_events(
        &self,
        request: UnsubscribeFileEventsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UnsubscribeFileEventsRespData>> {,
let mut api_req = ApiRequest {,
            http_method: Method::DELETE,
            api_path: DRIVE_V1_FILE_SUBSCRIPTIONS
                .replace("{}", &request.file_token)
                .replace()
            ..Default::default()
};
api_req
            .set_supported_access_token_types(vec![AccessTokenType::User, AccessTokenType::Tenant]);

        let api_resp = Transport::request(api_req, &self.config, option).await?;
Ok(api_resp),
    }
/// 订阅云文档事件请求参数,
#[derive(Debug, Clone)]
pub struct SubscribeFileEventsRequest {
    /// 文件token
    pub file_token: String,
    /// 文件类型
    pub file_type: String,
    /// 事件类型列表
    pub event_types: Vec<String>}
impl SubscribeFileEventsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 订阅云文档事件请求构建器,
#[derive(Default)]
pub struct SubscribeFileEventsRequestBuilder {
    request: SubscribeFileEventsRequest}
impl SubscribeFileEventsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    SubscribeFileEventsRequestBuilder,
    EventService,
    SubscribeFileEventsRequest,
    BaseResponse<SubscribeFileEventsRespData>,
    subscribe_file_events,
);
/// 订阅云文档事件响应数据
#[derive(Debug, Clone)]
pub struct SubscribeFileEventsRespData {
    /// 订阅ID
    pub subscription_id: String,
    /// 订阅类型
    pub subscription_type: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 查询云文档事件订阅状态请求参数,
#[derive(Debug, Clone)]
pub struct GetFileSubscriptionRequest {
    /// 文件token
    pub file_token: String,
    /// 订阅ID
    pub subscription_id: String,
impl GetFileSubscriptionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 查询云文档事件订阅状态请求构建器,
#[derive(Default)]
pub struct GetFileSubscriptionRequestBuilder {
    request: GetFileSubscriptionRequest}
impl GetFileSubscriptionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    GetFileSubscriptionRequestBuilder,
    EventService,
    GetFileSubscriptionRequest,
    BaseResponse<GetFileSubscriptionRespData>,
    get_file_subscription,
);
/// 查询云文档事件订阅状态响应数据
#[derive(Debug, Clone)]
pub struct GetFileSubscriptionRespData {
    /// 订阅信息
    pub subscription: FileSubscription,

#[derive(Debug, Clone)]
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
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 取消云文档事件订阅请求参数,
#[derive(Debug, Clone)]
pub struct UnsubscribeFileEventsRequest {
    /// 文件token
    pub file_token: String,
    /// 订阅ID
    pub subscription_id: String,
impl UnsubscribeFileEventsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 取消云文档事件订阅请求构建器,
#[derive(Default)]
pub struct UnsubscribeFileEventsRequestBuilder {
    request: UnsubscribeFileEventsRequest}
impl UnsubscribeFileEventsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    UnsubscribeFileEventsRequestBuilder,
    EventService,
    UnsubscribeFileEventsRequest,
    BaseResponse<UnsubscribeFileEventsRespData>,
    unsubscribe_file_events,
);
/// 取消云文档事件订阅响应数据
#[derive(Debug, Clone)]
pub struct UnsubscribeFileEventsRespData {
    /// 操作结果
    pub result: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
