//! Bitable 删除一个数据表
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 删除数据表请求
pub struct DeleteTableRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
}

/// 删除数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteTableResponse {}

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
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteTableResponse> {
        // 参数验证
        validate_required!(self.app_token.trim(), "app_token");

        validate_required!(self.table_id.trim(), "table_id");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::TableDelete(self.app_token.clone(), self.table_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<DeleteTableResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}




