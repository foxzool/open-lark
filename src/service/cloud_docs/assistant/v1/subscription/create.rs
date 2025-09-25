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

use super::{
    get::{FileType, SubscriptionDetail},
    SubscriptionService,
};

/// 创建订阅请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct CreateSubscriptionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    file_token: String,
    /// 文档类型
    #[serde(skip)]
    file_type: String,
    /// 订阅配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<SubscriptionConfig>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
}

/// 订阅配置
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SubscriptionConfig {
    pub enable_notification: Option<bool>,
    pub notification_interval: Option<i32>,
    pub priority: Option<SubscriptionPriority>,
    pub auto_renew: Option<bool>,
    pub tags: Option<Vec<String>>,
}

/// 订阅优先级
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPriority {
    /// 低优先级
    Low,
    /// 普通优先级
    Normal,
    /// 高优先级
    High,
    /// 紧急优先级
    Urgent,
}

impl CreateSubscriptionRequest {
    pub fn builder() -> CreateSubscriptionRequestBuilder {
        CreateSubscriptionRequestBuilder::default()
    }

    pub fn new(file_token: impl ToString, file_type: FileType) -> Self {
        Self {
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CreateSubscriptionRequestBuilder {
    request: CreateSubscriptionRequest,
}

impl CreateSubscriptionRequestBuilder {
    /// 文档token
    pub fn file_token(mut self, token: impl ToString) -> Self {
        self.request.file_token = token.to_string();
        self
    }

    /// 文档类型
    pub fn file_type(mut self, file_type: FileType) -> Self {
        self.request.file_type = file_type.to_string();
        self
    }

    /// 设置为多维表格
    pub fn as_bitable(mut self) -> Self {
        self.request.file_type = FileType::Bitable.to_string();
        self
    }

    /// 设置为文档
    pub fn as_doc(mut self) -> Self {
        self.request.file_type = FileType::Doc.to_string();
        self
    }

    /// 设置为表格
    pub fn as_sheet(mut self) -> Self {
        self.request.file_type = FileType::Sheet.to_string();
        self
    }

    /// 设置为Wiki
    pub fn as_wiki(mut self) -> Self {
        self.request.file_type = FileType::Wiki.to_string();
        self
    }

    /// 设置订阅配置
    pub fn config(mut self, config: SubscriptionConfig) -> Self {
        self.request.config = Some(config);
        self
    }

    /// 启用实时通知
    pub fn with_notification(mut self, enable: bool) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        config.enable_notification = Some(enable);
        self.request.config = Some(config);
        self
    }

    /// 设置通知频率（秒）
    pub fn notification_interval(mut self, interval: i32) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        config.notification_interval = Some(interval.max(1));
        self.request.config = Some(config);
        self
    }

    /// 设置订阅优先级
    pub fn priority(mut self, priority: SubscriptionPriority) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        config.priority = Some(priority);
        self.request.config = Some(config);
        self
    }

    /// 设置为高优先级
    pub fn high_priority(self) -> Self {
        self.priority(SubscriptionPriority::High)
    }

    /// 设置为低优先级
    pub fn low_priority(self) -> Self {
        self.priority(SubscriptionPriority::Low)
    }

    /// 启用自动续费
    pub fn auto_renew(mut self, enable: bool) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        config.auto_renew = Some(enable);
        self.request.config = Some(config);
        self
    }

    /// 添加标签
    pub fn add_tag(mut self, tag: impl ToString) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        let mut tags = config.tags.unwrap_or_default();
        tags.push(tag.to_string());
        config.tags = Some(tags);
        self.request.config = Some(config);
        self
    }

    /// 添加多个标签
    pub fn add_tags(mut self, tags: Vec<String>) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        let mut existing_tags = config.tags.unwrap_or_default();
        existing_tags.extend(tags);
        config.tags = Some(existing_tags);
        self.request.config = Some(config);
        self
    }

    /// 设置扩展信息
    pub fn extra(mut self, extra: serde_json::Value) -> Self {
        self.request.extra = Some(extra);
        self
    }

    /// 快速创建基础订阅
    pub fn basic_subscription(mut self) -> Self {
        let config = SubscriptionConfig::default();
        self.request.config = Some(config);
        self
    }

    /// 快速创建高级订阅
    pub fn premium_subscription(mut self) -> Self {
        let config = SubscriptionConfig::default();
        self.request.config = Some(config);
        self
    }

    pub fn build(mut self) -> CreateSubscriptionRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    CreateSubscriptionRequestBuilder,
    SubscriptionService,
    CreateSubscriptionRequest,
    CreateSubscriptionResponse,
    create
);

/// 创建订阅响应
#[derive(Debug, Deserialize)]
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
    pub subscription_id: Option<String>,
}

impl ApiResponseTrait for CreateSubscriptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建订阅
pub async fn create_subscription(
    request: CreateSubscriptionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreateSubscriptionResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;

    api_req.api_path = ASSISTANT_V1_FILE_SUBSCRIPTION
        .replace("{}", &request.file_type)
        .replace("{}", &request.file_token);

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl SubscriptionPriority {
    /// 获取优先级描述
    pub fn description(&self) -> &'static str {
        match self {
            SubscriptionPriority::Low => "低优先级",
            SubscriptionPriority::Normal => "普通优先级",
            SubscriptionPriority::High => "高优先级",
            SubscriptionPriority::Urgent => "紧急优先级",
        }
    }

    /// 获取优先级数值（用于排序）
    pub fn value(&self) -> u8 {
        match self {
            SubscriptionPriority::Low => 1,
            SubscriptionPriority::Normal => 2,
            SubscriptionPriority::High => 3,
            SubscriptionPriority::Urgent => 4,
        }
    }

    /// 获取优先级颜色
    pub fn color(&self) -> &'static str {
        match self {
            SubscriptionPriority::Low => "gray",
            SubscriptionPriority::Normal => "blue",
            SubscriptionPriority::High => "orange",
            SubscriptionPriority::Urgent => "red",
        }
    }
}

impl SubscriptionConfig {
    /// 获取通知频率（秒）
    pub fn get_notification_interval(&self) -> i32 {
        self.notification_interval.unwrap_or(3600)
    }

    /// 获取通知频率（分钟）
    pub fn get_notification_interval_minutes(&self) -> f64 {
        self.get_notification_interval() as f64 / 60.0
    }

    /// 获取通知频率（小时）
    pub fn get_notification_interval_hours(&self) -> f64 {
        self.get_notification_interval() as f64 / 3600.0
    }

    /// 是否为高频通知（小于1小时）
    pub fn is_high_frequency(&self) -> bool {
        self.get_notification_interval() < 3600
    }

    /// 是否启用通知
    pub fn has_notification(&self) -> bool {
        self.enable_notification.unwrap_or(false)
    }

    /// 获取优先级
    pub fn get_priority(&self) -> SubscriptionPriority {
        self.priority
            .clone()
            .unwrap_or(SubscriptionPriority::Normal)
    }

    /// 是否启用自动续费
    pub fn has_auto_renew(&self) -> bool {
        self.auto_renew.unwrap_or(false)
    }

    /// 获取标签列表
    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone().unwrap_or_default()
    }

    /// 是否包含指定标签
    pub fn has_tag(&self, tag: &str) -> bool {
        self.get_tags().contains(&tag.to_string())
    }

    /// 获取配置摘要
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();

        parts.push(format!("优先级: {}", self.get_priority().description()));

        if self.has_notification() {
            let interval = self.get_notification_interval_hours();
            if interval < 1.0 {
                parts.push(format!("通知: 每{:.0}分钟", interval * 60.0));
            } else {
                parts.push(format!("通知: 每{interval:.1}小时"));
            }
        } else {
            parts.push("通知: 已禁用".to_string());
        }

        if self.has_auto_renew() {
            parts.push("自动续费: 是".to_string());
        }

        let tags = self.get_tags();
        if !tags.is_empty() {
            parts.push(format!("标签: {}", tags.join(", ")));
        }

        parts.join(" | ")
    }
}

impl CreateSubscriptionResponse {
    /// 获取文档类型枚举
    pub fn file_type_enum(&self) -> FileType {
        match self.file_type.as_str() {
            "bitable" => FileType::Bitable,
            "doc" => FileType::Doc,
            "sheet" => FileType::Sheet,
            "wiki" => FileType::Wiki,
            _ => FileType::Doc,
        }
    }

    /// 获取创建时间格式化字符串
    pub fn create_time_formatted(&self) -> Option<String> {
        self.create_time.map(|timestamp| {
            let datetime =
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// 获取完整信息摘要
    pub fn info_summary(&self) -> String {
        let mut parts = vec![
            format!(
                "{} ({})",
                self.file_type_enum().chinese_name(),
                self.file_token
            ),
            self.subscription.summary(),
        ];

        if let Some(ref subscription_id) = self.subscription_id {
            parts.push(format!("订阅ID: {subscription_id}"));
        }

        if let Some(create_time) = self.create_time_formatted() {
            parts.push(format!("创建时间: {create_time}"));
        }

        parts.join(" | ")
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_create_subscription_request_builder() {
        let request = CreateSubscriptionRequest::builder()
            .file_token("doccnxxxxxx")
            .as_doc()
            .basic_subscription()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert!(request.config.is_some());
    }

    #[test]
    fn test_subscription_priority_methods() {
        assert_eq!(SubscriptionPriority::High.description(), "高优先级");
        assert_eq!(SubscriptionPriority::High.value(), 3);
        assert_eq!(SubscriptionPriority::High.color(), "orange");
    }

    #[test]
    fn test_subscription_config_methods() {
        let config = SubscriptionConfig {
            enable_notification: Some(true),
            notification_interval: Some(3600),
            ..Default::default()
        };

        assert!(config.has_notification());
        assert_eq!(config.get_notification_interval(), 3600);
        assert_eq!(config.get_notification_interval_hours(), 1.0);
        assert!(!config.is_high_frequency());
        assert!(!config.has_auto_renew());
    }

    #[test]
    fn test_subscription_config_builder() {
        let request = CreateSubscriptionRequest::builder()
            .file_token("test")
            .as_doc()
            .high_priority()
            .notification_interval(300)
            .auto_renew(true)
            .add_tag("important")
            .build();

        let config = request.config.unwrap();
        assert_eq!(config.get_priority().value(), 3);
        assert_eq!(config.get_notification_interval(), 300);
        assert!(config.has_auto_renew());
        assert!(config.has_tag("important"));
    }
}
