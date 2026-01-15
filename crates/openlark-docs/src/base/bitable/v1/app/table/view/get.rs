//! Bitable 获取视图
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

// 从 patch 模块导入 View 类型
use super::patch::View;

/// 获取视图请求
#[derive(Debug, Clone)]
pub struct GetViewRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String,
}

impl GetViewRequest {
    /// 创建获取视图请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
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

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.view_id = view_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetViewResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.table_id.trim().is_empty() {
            return Err(validation_error("table_id", "数据表ID不能为空"));
        }

        if self.view_id.trim().is_empty() {
            return Err(validation_error("view_id", "视图ID不能为空"));
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::ViewGet(
            self.app_token.clone(),
            self.table_id.clone(),
            self.view_id.clone(),
        );

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<GetViewResponse> = ApiRequest::get(&api_endpoint.to_url());

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| validation_error("response", "响应数据为空"))
    }
}

/// 获取视图Builder
pub struct GetViewRequestBuilder {
    request: GetViewRequest,
}

impl GetViewRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: GetViewRequest::new(config),
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

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.request = self.request.view_id(view_id);
        self
    }

    /// 构建请求
    pub fn build(self) -> GetViewRequest {
        self.request
    }
}

/// 获取视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetViewResponse {
    /// 视图信息
    pub view: View,
}

impl ApiResponseTrait for GetViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
