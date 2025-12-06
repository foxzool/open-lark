//! Bitable 删除数据表API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/table/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 删除数据表请求
#[allow(dead_code)]
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
    pub data: bool,
}

impl ApiResponseTrait for DeleteTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::table_delete(&self.app_token, &self.table_id);

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<DeleteTableResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
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
