//! Bitable V1 删除数据表API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 删除数据表请求
pub struct DeleteTableRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<DeleteTableResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
}

/// 删除数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteTableResponse {
    /// 操作结果
    pub success: bool,
}

impl DeleteTableRequest {
    /// 创建删除数据表请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::delete(""),
            app_token: String::new(),
            table_id: String::new(),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteTableResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/tables/{}",
            self.config.base_url, self.app_token, self.table_id
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> =
            ApiRequest::delete(api_request.url.clone()).body(RequestData::Empty);

        // 发送请求
        let response: openlark_core::api::Response<()> =
            Transport::request(request_for_transport, &self.config, None).await?;

        Ok(DeleteTableResponse {
            success: response.raw_response.is_success(),
        })
    }
}

/// 删除数据表Builder
pub struct DeleteTableRequestBuilder {
    request: DeleteTableRequest,
}

impl DeleteTableRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteTableRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.request = self.request.table_id(table_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> DeleteTableRequest {
        self.request
    }
}
