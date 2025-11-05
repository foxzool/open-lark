#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Event API v1版本
//!
//! 实现事件管理的核心功能：
//! - 获取事件出口IP地址
//! - 事件订阅管理
//! - 回调配置管理

use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use open_lark_core::core::api_req::ApiRequest;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// Event服务 v1版本
#[derive(Debug, Clone)]
pub struct EventServiceV1 {
    pub config: Config,
}

impl EventServiceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // ==================== 事件出口IP管理 ====================

    /// 获取事件出口IP
    ///
    /// 飞书开放平台向应用配置的回调地址推送事件时，是通过特定的 IP 发送出去的，
    /// 应用可以通过本接口获取所有相关的 IP 地址。
    ///
    /// # 参数
    ///
    /// * `request` - 获取事件出口IP请求
    ///
    /// # 返回值
    ///
    /// 返回包含IP地址列表的响应
    pub async fn get_outbound_ip(
        &self,
        request: &GetOutboundIpRequest,
    ) -> SDKResult<GetOutboundIpResponse> {
        let endpoint = "/open-apis/event/v1/outbound_ip";

        let mut api_req = ApiRequest::with_method_and_path(Method::GET, endpoint.to_string());

        // 设置访问令牌类型 - 使用应用访问令牌
        api_req.set_supported_access_token_types(vec![AccessTokenType::App]);

        let option = RequestOption::default();
        let api_resp = Transport::request(api_req, &self.config, Some(option)).await?;

        Ok(GetOutboundIpResponse {
            code: api_resp.code(),
            msg: api_resp.msg().to_string(),
            data: api_resp.data,
        })
    }

    /// 构建器模式的获取事件出口IP方法
    pub fn get_outbound_ip_builder(&self) -> GetOutboundIpRequestBuilder {
        GetOutboundIpRequestBuilder::new(self)
    }
}

// ==================== 请求响应模型 ====================

/// 获取事件出口IP请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetOutboundIpRequest {
    // 当前此接口不需要任何请求参数
}

/// 获取事件出口IP响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOutboundIpResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<OutboundIpData>,
}

/// 事件出口IP数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundIpData {
    /// IP地址列表
    pub ip_list: Vec<String>,
}

impl ApiResponseTrait for OutboundIpData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式实现 ====================

/// 获取事件出口IP请求构建器
#[derive(Debug, Clone)]
pub struct GetOutboundIpRequestBuilder<'a> {
    service: &'a EventServiceV1,
    request: GetOutboundIpRequest,
}

impl<'a> GetOutboundIpRequestBuilder<'a> {
    pub fn new(service: &'a EventServiceV1) -> Self {
        Self {
            service,
            request: GetOutboundIpRequest::default(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetOutboundIpResponse> {
        self.service.get_outbound_ip(&self.request).await
    }
}

// ==================== 特征实现 ====================

use crate::core::trait_system::Service;

impl Service for EventServiceV1 {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "event"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_service_v1_creation() {
        let config = Config::default();
        let service = EventServiceV1::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_get_outbound_ip_request_creation() {
        let request = GetOutboundIpRequest::default();
        assert!(!format!("{:?}", request).is_empty());
    }

    #[test]
    fn test_outbound_ip_data_creation() {
        let data = OutboundIpData {
            ip_list: vec!["1.2.3.4".to_string(), "5.6.7.8".to_string()],
        };
        assert_eq!(data.ip_list.len(), 2);
        assert_eq!(data.ip_list[0], "1.2.3.4");
    }

    #[test]
    fn test_get_outbound_ip_request_builder() {
        let config = Config::default();
        let service = EventServiceV1::new(config);
        let builder = service.get_outbound_ip_builder();
        assert!(!format!("{:?}", builder).is_empty());
    }

    #[test]
    fn test_service_config_access() {
        let config = Config::builder().app_id("test_event_service").build();
        let service = EventServiceV1::new(config);
        assert_eq!(service.config.app_id, "test_event_service");
    }

    #[test]
    fn test_service_name_and_version() {
        assert_eq!(EventServiceV1::service_name(), "event");
        assert_eq!(EventServiceV1::service_version(), "v1");
    }
}
