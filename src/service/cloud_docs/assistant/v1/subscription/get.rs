use SDKResult;use reqwest::Method;
use openlark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
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
    service::cloud_docs::assistant::v1::subscription::SubscriptionService,
};
/// 获取订阅状态请求,
#[derive(Debug, Clone)]
pub struct GetSubscriptionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    file_token: String,
    /// 文档类型,
#[serde(skip)]
    file_type: String}
impl GetSubscriptionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct GetSubscriptionRequestBuilder {
    request: GetSubscriptionRequest}
impl GetSubscriptionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    GetSubscriptionRequestBuilder,
    SubscriptionService,
    GetSubscriptionRequest,
    GetSubscriptionResponse,
    get,
);
/// 文档类型
#[derive(Debug, Clone)]
pub enum FileType {
    /// 多维表格
    Bitable,
    /// 文档
    Doc,
    /// 电子表格
    Sheet,
    /// Wiki
    Wiki}
impl std::fmt::Display for FileType {,
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {,
match self {,
            FileType::Bitable => write!(f, "bitable"),
            FileType::Doc => write!(f, "doc"),
            FileType::Sheet => write!(f, "sheet"),
            FileType::Wiki => write!(f, "wiki")}
impl FileType {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 是否支持助手功能,
    pub fn w+.*{
match self {,
            FileType::Bitable => true,
            FileType::Doc => true,
            FileType::Sheet => true,
            FileType::Wiki => true}
/// 订阅状态,
#[derive(Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
/// 已订阅,
    Subscribed,
    /// 未订阅
    Unsubscribed,
    /// 已暂停
    Paused,
    /// 已取消
    Cancelled,
    /// 未知状态,
#[serde(other)]
    Unknown}
impl SubscriptionStatus {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取状态颜色（用于UI显示）,
    pub fn w+.*{
match self {,
            SubscriptionStatus::Subscribed => "green",
            SubscriptionStatus::Unsubscribed => "gray",
            SubscriptionStatus::Paused => "orange",
            SubscriptionStatus::Cancelled => "red",
            SubscriptionStatus::Unknown => "gray"}
/// 订阅详情,
#[derive(Debug, Clone)]
pub struct SubscriptionDetail {
    /// 订阅状态
    pub status: SubscriptionStatus,
    /// 订阅开始时间（时间戳）
    pub start_time: Option<i64>,
    /// 订阅结束时间（时间戳）
    pub end_time: Option<i64>,
    /// 最后更新时间（时间戳）
    pub last_update_time: Option<i64>,
    /// 订阅者ID
    pub subscriber_id: Option<String>,
    /// 订阅者类型
    pub subscriber_type: Option<String>,
    /// 订阅配置
    pub config: Option<serde_json::Value>,
    /// 扩展信息
    pub extra: Option<serde_json::Value>}
/// 获取订阅状态响应,
#[derive(Debug, Clone)]
pub struct GetSubscriptionResponse {
    /// 订阅详情
    pub subscription: SubscriptionDetail,
    /// 文档token
    pub file_token: String,
    /// 文档类型
    pub file_type: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取订阅状态,
pub async fn get_subscription(
    request: GetSubscriptionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<GetSubscriptionResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.api_path = ASSISTANT_V1_FILE_SUBSCRIPTION,
        .replace("{}", &request.file_type)
        .replace("{}", &request.file_token);

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

impl SubscriptionDetail {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 获取订阅持续时间（天）,
    pub fn w+.*{
self.duration_seconds().map(|s| s as f64 / 86400.0)}
/// 获取订阅时间格式化字符串,
    pub fn w+.*{
self.start_time.map(|timestamp| {,
            let datetime =
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
datetime.format("%Y-%m-%d %H:%M:%S").to_string()}),
/// 获取结束时间格式化字符串,
    pub fn w+.*{
self.end_time.map(|timestamp| {,
            let datetime =
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
datetime.format("%Y-%m-%d %H:%M:%S").to_string()}),
/// 获取最后更新时间格式化字符串,
    pub fn w+.*{
self.last_update_time.map(|timestamp| {,
            let datetime =
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
datetime.format("%Y-%m-%d %H:%M:%S").to_string()}),
/// 获取订阅摘要,
    pub fn w+.*{
let mut parts = Vec::new();
        parts.push(format!("状态: {}", self.status.description()));
if let Some(start) = self.start_time_formatted() {,
            parts.push(format!("开始时间: {start}"));
if let Some(end) = self.end_time_formatted() {,
            parts.push(format!("结束时间: {end}"));
if let Some(days) = self.duration_days() {,
            parts.push(format!("持续时间: {days:.1} 天"));
if let Some(ref subscriber_id) = self.subscriber_id {,
            parts.push(format!("订阅者: {subscriber_id}"));
parts.join(" | "),
    }
impl GetSubscriptionResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_get_subscription_request_builder() {
let request = GetSubscriptionRequest::builder(),
            .file_token()
.as_doc()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
#[test]
    ,
        assert_eq!(FileType::Doc.to_string(), "doc");
        assert_eq!(FileType::Doc.chinese_name(), "文档");
assert!(FileType::Doc.supports_assistant());
    }
#[test]
    fn test_subscription_status_methods() {
assert!(SubscriptionStatus::Subscribed.is_active());
        assert!(!SubscriptionStatus::Unsubscribed.is_active());
assert!(SubscriptionStatus::Unsubscribed.can_activate());
        assert!(!SubscriptionStatus::Subscribed.can_activate());

        assert_eq!(SubscriptionStatus::Subscribed.description(), "已订阅");
        assert_eq!(SubscriptionStatus::Subscribed.color(), "green");
#[test]
    fn test_subscription_detail_methods() {
let detail = SubscriptionDetail {,
            status: SubscriptionStatus::Subscribed,
            start_time: Some(1700000000),
            end_time: Some(1700086400),
            last_update_time: Some(1700043200),
            subscriber_id: Some("user123".to_string()),
            subscriber_type: Some("user".to_string()),
            config: None,
            extra: None};
assert!(detail.is_subscribed());
        assert!(!detail.can_activate());
        assert_eq!(detail.duration_seconds(), Some(86400));
        assert_eq!(detail.duration_days(), Some(1.0));
