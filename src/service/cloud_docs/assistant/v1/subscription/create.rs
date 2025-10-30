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
use super::{,
    get::{FileType, SubscriptionDetail}
    SubscriptionService,
};
/// 创建订阅请求,
#[derive(Debug, Clone)]
pub struct CreateSubscriptionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    file_token: String,
    /// 文档类型,
#[serde(skip)]
    file_type: String,
    /// 订阅配置,
#[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<SubscriptionConfig>,
    /// 扩展信息,
#[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>}
/// 订阅配置,
#[derive(Debug, Clone)]
pub struct SubscriptionConfig {
    pub enable_notification: Option<bool>,
    pub notification_interval: Option<i32>,
    pub priority: Option<SubscriptionPriority>,
    pub auto_renew: Option<bool>,
    pub tags: Option<Vec<String>>}
/// 订阅优先级,
#[derive(Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPriority {
/// 低优先级,
    Low,
    /// 普通优先级
    Normal,
    /// 高优先级
    High,
    /// 紧急优先级
    Urgent}
impl CreateSubscriptionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreateSubscriptionRequestBuilder {
    request: CreateSubscriptionRequest}
impl CreateSubscriptionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    CreateSubscriptionRequestBuilder,
    SubscriptionService,
    CreateSubscriptionRequest,
    CreateSubscriptionResponse,
    create,
);
/// 创建订阅响应
#[derive(Debug, Clone)]
pub struct CreateSubscriptionResponse {
    /// 订阅详情
    pub subscription: SubscriptionDetail,
    /// 文档token
    pub file_token: String,
    /// 文档类型
    pub file_type: String,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 订阅ID
    pub subscription_id: Option<String>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 创建订阅,
pub async fn create_subscription(
    request: CreateSubscriptionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreateSubscriptionResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = ASSISTANT_V1_FILE_SUBSCRIPTION,
        .replace("{}", &request.file_type)
        .replace("{}", &request.file_token);

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

impl SubscriptionPriority {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取优先级数值（用于排序）,
    pub fn w+.*{
match self {,
            SubscriptionPriority::Low => 1,
            SubscriptionPriority::Normal => 2,
            SubscriptionPriority::High => 3,
            SubscriptionPriority::Urgent => 4}
/// 获取优先级颜色,
    pub fn w+.*{
match self {,
            SubscriptionPriority::Low => "gray",
            SubscriptionPriority::Normal => "blue",
            SubscriptionPriority::High => "orange",
            SubscriptionPriority::Urgent => "red"}
impl SubscriptionConfig {
    pub fn new(config: Config) -> Self {
        Self { config }
}        } else {,
parts.push("通知: 已禁用".to_string());
        }
if self.has_auto_renew() {,
            parts.push("自动续费: 是".to_string());
let tags = self.get_tags();
        if !tags.is_empty() {
            parts.push(format!("标签: {}", tags.join(", ")));
parts.join(" | "),
    }
impl CreateSubscriptionResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取创建时间格式化字符串,
    pub fn w+.*{
self.create_time.map(|timestamp| {,
            let datetime =
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
datetime.format("%Y-%m-%d %H:%M:%S").to_string()}),
/// 获取完整信息摘要,
    pub fn w+.*{
let mut parts = vec![,
            format!(
                "{} ({})",
                self.file_type_enum().chinese_name(),
                self.file_token
            ),
            self.subscription.summary(),
        ];
if let Some(ref subscription_id) = self.subscription_id {,
            parts.push(format!("订阅ID: {subscription_id}"));
if let Some(create_time) = self.create_time_formatted() {,
            parts.push(format!("创建时间: {create_time}"));
parts.join(" | "),
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_create_subscription_request_builder() {
let request = CreateSubscriptionRequest::builder(),
            .file_token()
.as_doc()
            .basic_subscription()
.build();
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
assert!(request.config.is_some());
    }
#[test]
    ,
        assert_eq!(SubscriptionPriority::High.description(), "高优先级");
        assert_eq!(SubscriptionPriority::High.value(), 3);
        assert_eq!(SubscriptionPriority::High.color(), "orange");
#[test]
    fn test_subscription_config_methods() {
let config = SubscriptionConfig {,
            enable_notification: Some(true),
            notification_interval: Some(3600),
            ..Default::default()};
assert!(config.has_notification());
        assert_eq!(config.get_notification_interval(), 3600);
        assert_eq!(config.get_notification_interval_hours(), 1.0);
assert!(!config.is_high_frequency());
        assert!(!config.has_auto_renew());
#[test]
    fn test_subscription_config_builder() {
let request = CreateSubscriptionRequest::builder(),
            .file_token()
.as_doc()
            .high_priority()
.notification_interval()
            .auto_renew()
.add_tag()
            .build();
let config = request.config.unwrap();
        assert_eq!(config.get_priority().value(), 3);
        assert_eq!(config.get_notification_interval(), 300);
assert!(config.has_auto_renew());
        assert!(config.has_tag("important"));
