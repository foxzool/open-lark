//! 获取告警记录
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/alert/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;

/// 获取告警记录请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ListAlertRequest {
    /// 配置信息
    config: Config,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 获取告警记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListAlertResponse {
    /// 告警列表
    pub alerts: Vec<AlertItem>,
    /// 是否有下一页
    pub has_more: Option<bool>,
}

/// 告警项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlertItem {
    /// 告警 ID
    pub alert_id: String,
    /// 告警级别
    pub level: String,
    /// 告警内容
    pub content: String,
}

impl ApiResponseTrait for ListAlertResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListAlertRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/alert/list
    pub async fn execute(self) -> SDKResult<ListAlertResponse> {
        // 🚀 使用新的枚举+builder系统生成API端点
        let api_endpoint = VcApiV1::AlertList;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListAlertResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 添加查询参数
        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}

/// 获取告警记录请求构建器
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ListAlertRequestBuilder {
    request: ListAlertRequest,
}

impl ListAlertRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListAlertRequest::new(config),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.request = self.request.query_param(key, value);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListAlertRequest {
        self.request
    }
}
