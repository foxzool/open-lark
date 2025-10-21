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
    create::{SubscriptionConfig, SubscriptionPriority},
    get::{FileType, SubscriptionDetail, SubscriptionStatus},
    SubscriptionService,
};

/// 更新订阅状态请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct PatchSubscriptionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    file_token: String,
    /// 文档类型
    #[serde(skip)]
    file_type: String,
    /// 订阅状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatus>,
    /// 订阅配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<SubscriptionConfig>,
    /// 扩展信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<serde_json::Value>,
}

impl PatchSubscriptionRequest {
    pub fn builder() -> PatchSubscriptionRequestBuilder {
        PatchSubscriptionRequestBuilder::default()
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
pub struct PatchSubscriptionRequestBuilder {
    request: PatchSubscriptionRequest,
    changes: Vec<String>,
}

impl PatchSubscriptionRequestBuilder {
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

    /// 设置订阅状态
    pub fn status(mut self, status: SubscriptionStatus) -> Self {
        self.changes.push(format!("状态: {}", status.description()));
        self.request.status = Some(status);
        self
    }

    /// 激活订阅
    pub fn activate(self) -> Self {
        self.status(SubscriptionStatus::Subscribed)
    }

    /// 暂停订阅
    pub fn pause(self) -> Self {
        self.status(SubscriptionStatus::Paused)
    }

    /// 取消订阅
    pub fn cancel(self) -> Self {
        self.status(SubscriptionStatus::Cancelled)
    }

    /// 恢复订阅（从暂停状态）
    pub fn resume(self) -> Self {
        self.status(SubscriptionStatus::Subscribed)
    }

    /// 设置订阅配置
    pub fn config(mut self, config: SubscriptionConfig) -> Self {
        self.changes.push("配置已更新".to_string());
        self.request.config = Some(config);
        self
    }

    /// 启用/禁用实时通知
    pub fn notification(mut self, enable: bool) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        config.enable_notification = Some(enable);
        self.changes.push(format!(
            "通知: {}",
            if enable { "已启用" } else { "已禁用" }
        ));
        self.request.config = Some(config);
        self
    }

    /// 设置通知频率（秒）
    pub fn notification_interval(mut self, interval: i32) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        config.notification_interval = Some(interval.max(1));
        let hours = interval as f64 / 3600.0;
        if hours < 1.0 {
            self.changes
                .push(format!("通知频率: 每{:.0}分钟", hours * 60.0));
        } else {
            self.changes.push(format!("通知频率: 每{hours:.1}小时"));
        }
        self.request.config = Some(config);
        self
    }

    /// 设置快速通知（每5分钟）
    pub fn quick_notification(self) -> Self {
        self.notification_interval(300)
    }

    /// 设置标准通知（每小时）
    pub fn standard_notification(self) -> Self {
        self.notification_interval(3600)
    }

    /// 设置慢速通知（每6小时）
    pub fn slow_notification(self) -> Self {
        self.notification_interval(21600)
    }

    /// 设置订阅优先级
    pub fn priority(mut self, priority: SubscriptionPriority) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        config.priority = Some(priority.clone());
        self.changes
            .push(format!("优先级: {}", priority.description()));
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

    /// 设置为紧急优先级
    pub fn urgent_priority(self) -> Self {
        self.priority(SubscriptionPriority::Urgent)
    }

    /// 启用/禁用自动续费
    pub fn auto_renew(mut self, enable: bool) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        config.auto_renew = Some(enable);
        self.changes.push(format!(
            "自动续费: {}",
            if enable { "已启用" } else { "已禁用" }
        ));
        self.request.config = Some(config);
        self
    }

    /// 添加标签
    pub fn add_tag(mut self, tag: impl ToString) -> Self {
        let tag_str = tag.to_string();
        let mut config = self.request.config.unwrap_or_default();
        let mut tags = config.tags.unwrap_or_default();
        if !tags.contains(&tag_str) {
            tags.push(tag_str.clone());
            self.changes.push(format!("添加标签: {tag_str}"));
        }
        config.tags = Some(tags);
        self.request.config = Some(config);
        self
    }

    /// 移除标签
    pub fn remove_tag(mut self, tag: impl ToString) -> Self {
        let tag_str = tag.to_string();
        let mut config = self.request.config.unwrap_or_default();
        let mut tags = config.tags.unwrap_or_default();
        if let Some(pos) = tags.iter().position(|t| t == &tag_str) {
            tags.remove(pos);
            self.changes.push(format!("移除标签: {tag_str}"));
        }
        config.tags = Some(tags);
        self.request.config = Some(config);
        self
    }

    /// 清空所有标签
    pub fn clear_tags(mut self) -> Self {
        let mut config = self.request.config.unwrap_or_default();
        config.tags = Some(vec![]);
        self.changes.push("清空所有标签".to_string());
        self.request.config = Some(config);
        self
    }

    /// 设置扩展信息
    pub fn extra(mut self, extra: serde_json::Value) -> Self {
        self.request.extra = Some(extra);
        self.changes.push("扩展信息已更新".to_string());
        self
    }

    /// 批量暂停订阅（安全模式）
    pub fn safe_pause(self) -> Self {
        self.pause().notification(false).add_tag("paused_by_system")
    }

    /// 批量激活订阅（快速模式）
    pub fn quick_activate(self) -> Self {
        self.activate()
            .notification(true)
            .quick_notification()
            .high_priority()
            .remove_tag("paused_by_system")
    }

    /// 批量激活订阅（节能模式）
    pub fn eco_activate(self) -> Self {
        self.activate()
            .notification(true)
            .slow_notification()
            .low_priority()
            .auto_renew(false)
    }

    /// 获取更改摘要
    pub fn get_changes(&self) -> Vec<String> {
        self.changes.clone()
    }

    /// 获取更改摘要字符串
    pub fn changes_summary(&self) -> String {
        if self.changes.is_empty() {
            "无更改".to_string()
        } else {
            self.changes.join(", ")
        }
    }

    pub fn build(mut self) -> PatchSubscriptionRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    PatchSubscriptionRequestBuilder,
    SubscriptionService,
    PatchSubscriptionRequest,
    PatchSubscriptionResponse,
    patch
);

/// 更新订阅状态响应
#[derive(Debug, Deserialize)]
pub struct PatchSubscriptionResponse {
    /// 订阅详情
    pub subscription: SubscriptionDetail,
    /// 文档token
    pub file_token: String,
    /// 文档类型
    pub file_type: String,
    /// 更新时间
    pub update_time: Option<i64>,
    /// 订阅ID
    pub subscription_id: Option<String>,
    /// 更新的字段
    pub updated_fields: Option<Vec<String>>,
}

impl ApiResponseTrait for PatchSubscriptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新订阅状态
pub async fn patch_subscription(
    request: PatchSubscriptionRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<PatchSubscriptionResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PATCH;

    api_req.api_path = ASSISTANT_V1_FILE_SUBSCRIPTION
        .replace("{}", &request.file_type)
        .replace("{}", &request.file_token);

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl PatchSubscriptionRequest {
    /// 是否有状态更改
    pub fn has_status_change(&self) -> bool {
        self.status.is_some()
    }

    /// 是否有配置更改
    pub fn has_config_change(&self) -> bool {
        self.config.is_some()
    }

    /// 是否有任何更改
    pub fn has_changes(&self) -> bool {
        self.has_status_change() || self.has_config_change() || self.extra.is_some()
    }

    /// 获取请求摘要
    pub fn summary(&self) -> String {
        let mut parts = vec![format!("文档: {} ({})", self.file_token, self.file_type)];

        if let Some(ref status) = self.status {
            parts.push(format!("状态: {}", status.description()));
        }

        if let Some(ref config) = self.config {
            parts.push(format!("配置: {}", config.summary()));
        }

        parts.join(" | ")
    }
}

impl PatchSubscriptionResponse {
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

    /// 获取更新时间格式化字符串
    pub fn update_time_formatted(&self) -> Option<String> {
        self.update_time.map(|timestamp| {
            let datetime =
                chrono::DateTime::from_timestamp(timestamp, 0).unwrap_or_else(chrono::Utc::now);
            datetime.format("%Y-%m-%d %H:%M:%S").to_string()
        })
    }

    /// 获取更新字段列表
    pub fn get_updated_fields(&self) -> Vec<String> {
        self.updated_fields.clone().unwrap_or_default()
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

        if let Some(update_time) = self.update_time_formatted() {
            parts.push(format!("更新时间: {update_time}"));
        }

        let updated_fields = self.get_updated_fields();
        if !updated_fields.is_empty() {
            parts.push(format!("更新字段: {}", updated_fields.join(", ")));
        }

        parts.join(" | ")
    }

    /// 是否更新成功
    pub fn is_updated(&self) -> bool {
        !self.get_updated_fields().is_empty() || self.update_time.is_some()
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_subscription_request_builder() {
        let builder = PatchSubscriptionRequest::builder()
            .file_token("doccnxxxxxx")
            .as_doc()
            .activate()
            .high_priority()
            .quick_notification()
            .auto_renew(true)
            .add_tag("updated");

        let changes = builder.get_changes();
        assert!(!changes.is_empty());
        assert!(changes.iter().any(|c| c.contains("状态")));
        assert!(changes.iter().any(|c| c.contains("优先级")));

        let request = builder.build();
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert!(request.has_status_change());
        assert!(request.has_config_change());
    }

    #[test]
    fn test_patch_subscription_quick_modes() {
        let request = PatchSubscriptionRequest::builder()
            .file_token("test")
            .as_doc()
            .quick_activate()
            .build();

        assert!(request.has_status_change());
        assert!(request.has_config_change());

        let config = request.config.unwrap();
        assert!(config.has_notification());
        assert_eq!(config.get_notification_interval(), 300);
        assert_eq!(config.get_priority().value(), 3);
    }

    #[test]
    fn test_patch_subscription_eco_mode() {
        let request = PatchSubscriptionRequest::builder()
            .file_token("test")
            .as_doc()
            .eco_activate()
            .build();

        let config = request.config.unwrap();
        assert!(config.has_notification());
        assert_eq!(config.get_notification_interval(), 21600);
        assert_eq!(config.get_priority().value(), 1);
        assert!(!config.has_auto_renew());
    }

    #[test]
    fn test_patch_subscription_tag_management() {
        let request = PatchSubscriptionRequest::builder()
            .file_token("test")
            .as_doc()
            .add_tag("important")
            .add_tag("urgent")
            .remove_tag("old")
            .build();

        let config = request.config.unwrap();
        let tags = config.get_tags();
        assert!(tags.contains(&"important".to_string()));
        assert!(tags.contains(&"urgent".to_string()));
    }
}
