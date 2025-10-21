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
    service::cloud_docs::assistant::v1::subscription::SubscriptionService,
};

/// 获取订阅状态请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct GetSubscriptionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    file_token: String,
    /// 文档类型
    #[serde(skip)]
    file_type: String,
}

impl GetSubscriptionRequest {
    pub fn builder() -> GetSubscriptionRequestBuilder {
        GetSubscriptionRequestBuilder::default()
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
pub struct GetSubscriptionRequestBuilder {
    request: GetSubscriptionRequest,
}

impl GetSubscriptionRequestBuilder {
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

    pub fn build(mut self) -> GetSubscriptionRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    GetSubscriptionRequestBuilder,
    SubscriptionService,
    GetSubscriptionRequest,
    GetSubscriptionResponse,
    get
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
    Wiki,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Bitable => write!(f, "bitable"),
            FileType::Doc => write!(f, "doc"),
            FileType::Sheet => write!(f, "sheet"),
            FileType::Wiki => write!(f, "wiki"),
        }
    }
}

impl FileType {
    /// 获取文档类型的中文名称
    pub fn chinese_name(&self) -> &'static str {
        match self {
            FileType::Bitable => "多维表格",
            FileType::Doc => "文档",
            FileType::Sheet => "电子表格",
            FileType::Wiki => "Wiki",
        }
    }

    /// 是否支持助手功能
    pub fn supports_assistant(&self) -> bool {
        match self {
            FileType::Bitable => true,
            FileType::Doc => true,
            FileType::Sheet => true,
            FileType::Wiki => true,
        }
    }
}

/// 订阅状态
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    /// 已订阅
    Subscribed,
    /// 未订阅
    Unsubscribed,
    /// 已暂停
    Paused,
    /// 已取消
    Cancelled,
    /// 未知状态
    #[serde(other)]
    Unknown,
}

impl SubscriptionStatus {
    /// 是否为活跃状态
    pub fn is_active(&self) -> bool {
        matches!(self, SubscriptionStatus::Subscribed)
    }

    /// 是否可以激活
    pub fn can_activate(&self) -> bool {
        matches!(
            self,
            SubscriptionStatus::Unsubscribed | SubscriptionStatus::Paused
        )
    }

    /// 获取状态描述
    pub fn description(&self) -> &'static str {
        match self {
            SubscriptionStatus::Subscribed => "已订阅",
            SubscriptionStatus::Unsubscribed => "未订阅",
            SubscriptionStatus::Paused => "已暂停",
            SubscriptionStatus::Cancelled => "已取消",
            SubscriptionStatus::Unknown => "未知状态",
        }
    }

    /// 获取状态颜色（用于UI显示）
    pub fn color(&self) -> &'static str {
        match self {
            SubscriptionStatus::Subscribed => "green",
            SubscriptionStatus::Unsubscribed => "gray",
            SubscriptionStatus::Paused => "orange",
            SubscriptionStatus::Cancelled => "red",
            SubscriptionStatus::Unknown => "gray",
        }
    }
}

/// 订阅详情
#[derive(Debug, Deserialize, Clone)]
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
    pub extra: Option<serde_json::Value>,
}

/// 获取订阅状态响应
#[derive(Debug, Deserialize)]
pub struct GetSubscriptionResponse {
    /// 订阅详情
    pub subscription: SubscriptionDetail,
    /// 文档token
    pub file_token: String,
    /// 文档类型
    pub file_type: String,
}

impl ApiResponseTrait for GetSubscriptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取订阅状态
pub async fn get_subscription(
    request: GetSubscriptionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<GetSubscriptionResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;

    api_req.api_path = ASSISTANT_V1_FILE_SUBSCRIPTION
        .replace("{}", &request.file_type)
        .replace("{}", &request.file_token);

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl SubscriptionDetail {
    /// 是否已订阅
    pub fn is_subscribed(&self) -> bool {
        self.status.is_active()
    }

    /// 是否可以激活订阅
    pub fn can_activate(&self) -> bool {
        self.status.can_activate()
    }

    /// 获取订阅持续时间（秒）
    pub fn duration_seconds(&self) -> Option<i64> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => Some(end - start),
            _ => None,
        }
    }

    /// 获取订阅持续时间（天）
    pub fn duration_days(&self) -> Option<f64> {
        self.duration_seconds().map(|s| s as f64 / 86400.0)
    }

    /// 获取订阅时间格式化字符串
    pub fn start_time_formatted(&self) -> Option<String> {
        self.start_time.map(|timestamp| {
            let datetime =
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// 获取结束时间格式化字符串
    pub fn end_time_formatted(&self) -> Option<String> {
        self.end_time.map(|timestamp| {
            let datetime =
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// 获取最后更新时间格式化字符串
    pub fn last_update_time_formatted(&self) -> Option<String> {
        self.last_update_time.map(|timestamp| {
            let datetime =
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// 获取订阅摘要
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();

        parts.push(format!("状态: {}", self.status.description()));

        if let Some(start) = self.start_time_formatted() {
            parts.push(format!("开始时间: {start}"));
        }

        if let Some(end) = self.end_time_formatted() {
            parts.push(format!("结束时间: {end}"));
        }

        if let Some(days) = self.duration_days() {
            parts.push(format!("持续时间: {days:.1} 天"));
        }

        if let Some(ref subscriber_id) = self.subscriber_id {
            parts.push(format!("订阅者: {subscriber_id}"));
        }

        parts.join(" | ")
    }
}

impl GetSubscriptionResponse {
    /// 获取文档类型枚举
    pub fn file_type_enum(&self) -> FileType {
        match self.file_type.as_str() {
            "bitable" => FileType::Bitable,
            "doc" => FileType::Doc,
            "sheet" => FileType::Sheet,
            "wiki" => FileType::Wiki,
            _ => FileType::Doc, // 默认为文档
        }
    }

    /// 获取完整信息摘要
    pub fn info_summary(&self) -> String {
        format!(
            "{} ({}) - {}",
            self.file_type_enum().chinese_name(),
            self.file_token,
            self.subscription.summary()
        )
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_get_subscription_request_builder() {
        let request = GetSubscriptionRequest::builder()
            .file_token("doccnxxxxxx")
            .as_doc()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
    }

    #[test]
    fn test_file_type_methods() {
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
    }

    #[test]
    fn test_subscription_detail_methods() {
        let detail = SubscriptionDetail {
            status: SubscriptionStatus::Subscribed,
            start_time: Some(1700000000),
            end_time: Some(1700086400),
            last_update_time: Some(1700043200),
            subscriber_id: Some("user123".to_string()),
            subscriber_type: Some("user".to_string()),
            config: None,
            extra: None,
        };

        assert!(detail.is_subscribed());
        assert!(!detail.can_activate());
        assert_eq!(detail.duration_seconds(), Some(86400));
        assert_eq!(detail.duration_days(), Some(1.0));
    }
}
