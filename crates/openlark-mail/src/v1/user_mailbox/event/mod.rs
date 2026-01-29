//! 用户邮箱事件模块

use crate::common::{api_endpoints::MailApiV1, api_utils::*};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::Deserialize;
use std::sync::Arc;

// ===== 数据模型 =====

/// 订阅用户邮箱事件响应
#[derive(Debug, Clone, Deserialize)]
pub struct SubscribeUserMailboxEventResponse {
    /// 是否订阅成功
    pub success: bool,
}

/// 取消订阅用户邮箱事件响应
#[derive(Debug, Clone, Deserialize)]
pub struct UnsubscribeUserMailboxEventResponse {
    /// 是否取消订阅成功
    pub success: bool,
}

/// 获取用户邮箱事件订阅响应
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxEventSubscriptionResponse {
    /// 是否已订阅
    pub is_subscribed: bool,
    /// 订阅时间
    #[serde(default)]
    pub subscribe_time: Option<String>,
}

/// Event：用户邮箱事件资源
#[derive(Clone)]
pub struct Event {
    config: Arc<Config>,
    mailbox_id: String,
}

impl Event {
    pub fn new(config: Arc<Config>, mailbox_id: String) -> Self {
        Self { config, mailbox_id }
    }

    /// 订阅事件
    pub fn subscribe(&self) -> SubscribeUserMailboxEventRequest {
        SubscribeUserMailboxEventRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 取消订阅事件
    pub fn unsubscribe(&self) -> UnsubscribeUserMailboxEventRequest {
        UnsubscribeUserMailboxEventRequest::new(self.config.clone(), self.mailbox_id.clone())
    }

    /// 获取订阅状态
    pub fn subscription(&self) -> UserMailboxEventSubscriptionRequest {
        UserMailboxEventSubscriptionRequest::new(self.config.clone(), self.mailbox_id.clone())
    }
}

/// 订阅用户邮箱事件请求
#[derive(Debug, Clone)]
pub struct SubscribeUserMailboxEventRequest {
    config: Arc<Config>,
    mailbox_id: String,
}

impl SubscribeUserMailboxEventRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<SubscribeUserMailboxEventResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SubscribeUserMailboxEventResponse> {
        let api_endpoint = MailApiV1::UserMailboxEventSubscribe(self.mailbox_id.clone());
        let request = ApiRequest::<SubscribeUserMailboxEventResponse>::post(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "订阅用户邮箱事件")
    }
}

impl ApiResponseTrait for SubscribeUserMailboxEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 取消订阅用户邮箱事件请求
#[derive(Debug, Clone)]
pub struct UnsubscribeUserMailboxEventRequest {
    config: Arc<Config>,
    mailbox_id: String,
}

impl UnsubscribeUserMailboxEventRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UnsubscribeUserMailboxEventResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UnsubscribeUserMailboxEventResponse> {
        let api_endpoint = MailApiV1::UserMailboxEventUnsubscribe(self.mailbox_id.clone());
        let request =
            ApiRequest::<UnsubscribeUserMailboxEventResponse>::post(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "取消订阅用户邮箱事件")
    }
}

impl ApiResponseTrait for UnsubscribeUserMailboxEventResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取用户邮箱事件订阅请求
#[derive(Debug, Clone)]
pub struct UserMailboxEventSubscriptionRequest {
    config: Arc<Config>,
    mailbox_id: String,
}

impl UserMailboxEventSubscriptionRequest {
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<UserMailboxEventSubscriptionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UserMailboxEventSubscriptionResponse> {
        let api_endpoint = MailApiV1::UserMailboxEventSubscription(self.mailbox_id.clone());
        let request =
            ApiRequest::<UserMailboxEventSubscriptionResponse>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取用户邮箱事件订阅状态")
    }
}

impl ApiResponseTrait for UserMailboxEventSubscriptionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscribe_user_mailbox_event_request() {
        let config = Arc::new(
            openlark_core::config::Config::builder()
                .app_id("test")
                .app_secret("test")
                .build(),
        );

        let request = SubscribeUserMailboxEventRequest::new(config, "mailbox_123".to_string());
        assert_eq!(request.mailbox_id, "mailbox_123");
    }
}
