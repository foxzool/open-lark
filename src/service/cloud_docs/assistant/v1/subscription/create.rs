use crate::SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};

use open_lark_core::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::ASSISTANT_V1_FILE_SUBSCRIPTION,
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
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateSubscriptionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    pub file_token: String,
    /// 文档类型
    #[serde(skip)]
    pub file_type: String,
    /// 订阅配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<SubscriptionConfig>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
}

impl CreateSubscriptionRequest {
    /// 创建新的创建订阅请求
    pub fn new(file_token: impl Into<String>, file_type: FileType) -> Self {
        Self {
            api_request: ApiRequest::new(),
            file_token: file_token.into(),
            file_type: file_type.to_string(),
            config: None,
            extra: None,
        }
    }

    /// 创建构建器
    pub fn builder() -> CreateSubscriptionRequestBuilder {
        CreateSubscriptionRequestBuilder::default()
    }
}

/// 创建订阅请求构建器
#[derive(Debug, Clone, Default)]
pub struct CreateSubscriptionRequestBuilder {
    request: CreateSubscriptionRequest,
}

impl CreateSubscriptionRequestBuilder {
    /// 设置文档token
    pub fn file_token(mut self, token: impl Into<String>) -> Self {
        self.request.file_token = token.into();
        self
    }

    /// 设置文档类型为文档
    pub fn as_doc(mut self) -> Self {
        self.request.file_type = "doc".to_string();
        self
    }

    /// 设置文档类型为表格
    pub fn as_sheet(mut self) -> Self {
        self.request.file_type = "sheet".to_string();
        self
    }

    /// 设置文档类型为幻灯片
    pub fn as_slide(mut self) -> Self {
        self.request.file_type = "slide".to_string();
        self
    }

    /// 设置文档类型为白板
    pub fn as_whiteboard(mut self) -> Self {
        self.request.file_type = "whiteboard".to_string();
        self
    }

    /// 设置文档类型为思维导图
    pub fn as_mindnote(mut self) -> Self {
        self.request.file_type = "mindnote".to_string();
        self
    }

    /// 设置订阅配置
    pub fn config(mut self, config: SubscriptionConfig) -> Self {
        self.request.config = Some(config);
        self
    }

    /// 设置基础订阅配置
    pub fn basic_subscription(self) -> Self {
        self.config(SubscriptionConfig::default())
    }

    /// 设置高优先级订阅
    pub fn high_priority(self) -> Self {
        let mut config = SubscriptionConfig::default();
        config.priority = Some(SubscriptionPriority::High);
        self.config(config)
    }

    /// 设置紧急优先级订阅
    pub fn urgent_priority(self) -> Self {
        let mut config = SubscriptionConfig::default();
        config.priority = Some(SubscriptionPriority::Urgent);
        self.config(config)
    }

    /// 启用通知并设置间隔（秒）
    pub fn notification_interval(mut self, seconds: i32) -> Self {
        if let Some(ref mut config) = self.request.config {
            config.enable_notification = Some(true);
            config.notification_interval = Some(seconds);
        } else {
            self.request.config = Some(SubscriptionConfig {
                enable_notification: Some(true),
                notification_interval: Some(seconds),
                priority: None,
                auto_renew: None,
                tags: None,
            });
        }
        self
    }

    /// 启用自动续费
    pub fn auto_renew(mut self) -> Self {
        if let Some(ref mut config) = self.request.config {
            config.auto_renew = Some(true);
        } else {
            self.request.config = Some(SubscriptionConfig {
                enable_notification: None,
                notification_interval: None,
                priority: None,
                auto_renew: Some(true),
                tags: None,
            });
        }
        self
    }

    /// 添加标签
    pub fn add_tag(mut self, tag: impl Into<String>) -> Self {
        if let Some(ref mut config) = self.request.config {
            if let Some(ref mut tags) = config.tags {
                tags.push(tag.into());
            } else {
                config.tags = Some(vec![tag.into()]);
            }
        } else {
            self.request.config = Some(SubscriptionConfig {
                enable_notification: None,
                notification_interval: None,
                priority: None,
                auto_renew: None,
                tags: Some(vec![tag.into()]),
            });
        }
        self
    }

    /// 设置扩展信息
    pub fn extra(mut self, extra: serde_json::Value) -> Self {
        self.request.extra = Some(extra);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateSubscriptionRequest {
        self.request
    }
}

/// 订阅配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubscriptionConfig {
    /// 是否启用通知
    pub enable_notification: Option<bool>,
    /// 通知间隔（秒）
    pub notification_interval: Option<i32>,
    /// 订阅优先级
    pub priority: Option<SubscriptionPriority>,
    /// 是否自动续费
    pub auto_renew: Option<bool>,
    /// 标签
    pub tags: Option<Vec<String>>,
}

impl SubscriptionConfig {
    /// 判断是否启用通知
    pub fn has_notification(&self) -> bool {
        self.enable_notification.unwrap_or(false)
    }

    /// 获取通知间隔
    pub fn get_notification_interval(&self) -> i32 {
        self.notification_interval.unwrap_or(3600) // 默认1小时
    }

    /// 获取通知间隔（小时）
    pub fn get_notification_interval_hours(&self) -> f64 {
        self.get_notification_interval() as f64 / 3600.0
    }

    /// 判断是否为高频通知（间隔小于1小时）
    pub fn is_high_frequency(&self) -> bool {
        self.get_notification_interval() < 3600
    }

    /// 获取优先级
    pub fn get_priority(&self) -> SubscriptionPriority {
        self.priority.clone().unwrap_or(SubscriptionPriority::Normal)
    }

    /// 判断是否有自动续费
    pub fn has_auto_renew(&self) -> bool {
        self.auto_renew.unwrap_or(false)
    }

    /// 获取标签列表
    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone().unwrap_or_default()
    }

    /// 判断是否有指定标签
    pub fn has_tag(&self, tag: &str) -> bool {
        self.get_tags().contains(&tag.to_string())
    }

    /// 获取配置摘要
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();

        if self.has_notification() {
            let hours = self.get_notification_interval_hours();
            if hours >= 24.0 {
                parts.push(format!("通知: 每{:.0}小时", hours / 24.0));
            } else {
                parts.push(format!("通知: 每{:.0}分钟", hours * 60.0));
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

/// 订阅优先级
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl Default for SubscriptionPriority {
    fn default() -> Self {
        SubscriptionPriority::Normal
    }
}

impl SubscriptionPriority {
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

    /// 获取优先级描述
    pub fn description(&self) -> &'static str {
        match self {
            SubscriptionPriority::Low => "低优先级",
            SubscriptionPriority::Normal => "普通优先级",
            SubscriptionPriority::High => "高优先级",
            SubscriptionPriority::Urgent => "紧急优先级",
        }
    }
}

/// 创建订阅响应
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl CreateSubscriptionResponse {
    /// 获取文档类型枚举
    pub fn file_type_enum(&self) -> FileType {
        match self.file_type.as_str() {
            "doc" => FileType::Doc,
            "sheet" => FileType::Sheet,
            "slide" => FileType::Slide,
            "whiteboard" => FileType::Whiteboard,
            "mindnote" => FileType::Mindnote,
            _ => FileType::Unknown,
        }
    }

    /// 获取创建时间格式化字符串
    pub fn create_time_formatted(&self) -> Option<String> {
        self.create_time.map(|timestamp| {
            let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
                .unwrap_or_else(|| chrono::Utc::now());
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// 获取完整信息摘要
    pub fn full_summary(&self) -> String {
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

// 实现Builder模式的宏调用
impl_executable_builder_owned!(
    CreateSubscriptionRequestBuilder,
    SubscriptionService,
    CreateSubscriptionRequest,
    CreateSubscriptionResponse,
    create,
);

/// 创建订阅
pub async fn create_subscription(
    request: CreateSubscriptionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreateSubscriptionResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);

    api_req.set_api_path(
        ASSISTANT_V1_FILE_SUBSCRIPTION
            .replace("{}", &request.file_type)
            .replace("{}", &request.file_token),
    );

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
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
    fn test_subscription_priority() {
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
            .file_token("doccnxxxxxx")
            .as_doc()
            .high_priority()
            .notification_interval(300)
            .auto_renew()
            .add_tag("important")
            .build();

        let config = request.config.unwrap();
        assert_eq!(config.get_priority().value(), 3);
        assert_eq!(config.get_notification_interval(), 300);
        assert!(config.has_auto_renew());
        assert!(config.has_tag("important"));
    }
}