//! 新建记录
//!
//! 文档: https://open.feishu.cn/document/apaas-v1/application-object-record/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 新建记录 Builder
#[derive(Debug, Clone)]
pub struct RecordCreateBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 记录数据
    data: serde_json::Value,
}

impl RecordCreateBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        object_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            object_api_name: object_api_name.into(),
            data: serde_json::json!({}),
        }
    }

    /// 设置记录数据
    pub fn data(mut self, data: impl Into<serde_json::Value>) -> Self {
        self.data = data.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordCreateResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records",
            self.namespace, self.object_api_name
        );

        let request = RecordCreateRequest {
            data: self.data,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, None::<&()>).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<RecordCreateResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records",
            self.namespace, self.object_api_name
        );

        let request = RecordCreateRequest {
            data: self.data,
        };

        let transport = Transport::new(self.config);
        transport.post(url, request, Some(option)).await
    }
}

/// 新建记录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordCreateRequest {
    /// 记录数据
    #[serde(rename = "data")]
    data: serde_json::Value,
}

/// 新建记录响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordCreateResponse {
    /// 记录 ID
    #[serde(rename = "id")]
    id: String,
    /// 创建时间
    #[serde(rename = "created_time")]
    created_time: i64,
}

impl ApiResponseTrait for RecordCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
