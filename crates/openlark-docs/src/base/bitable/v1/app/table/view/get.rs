//! Bitable 获取视图
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

// 从 patch 模块导入 View 类型
use super::patch::View;

/// 获取视图请求
///
/// 用于获取多维表格数据表中的指定视图信息。
///
/// # 字段说明
///
/// - `app_token`: 多维表格的 app_token
/// - `table_id`: 数据表的 table_id
/// - `view_id`: 视图的 view_id
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_docs::base::bitable::v1::app::table::view::get::GetViewRequest;
/// use openlark_core::Config;
///
/// let config = Config::default();
/// let request = GetViewRequest::new(config)
///     .app_token("app_token_xyz".to_string())
///     .table_id("table_id_xyz".to_string())
///     .view_id("view_id_xyz".to_string());
/// ```
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
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetViewResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");

        validate_required!(self.table_id.trim(), "table_id");

        validate_required!(self.view_id.trim(), "view_id");

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
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
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

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_empty_app_token() {
        let config = Config::default();
        let request = GetViewRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .view_id("view_id".to_string());

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("app_token"));
    }

    #[test]
    fn test_empty_table_id() {
        let config = Config::default();
        let request = GetViewRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("".to_string())
            .view_id("view_id".to_string());

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("table_id"));
    }

    #[test]
    fn test_empty_view_id() {
        let config = Config::default();
        let request = GetViewRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .view_id("".to_string());

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("view_id"));
    }

    #[test]
    fn test_get_view_request_builder() {
        let config = Config::default();
        let request = GetViewRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .view_id("view_id".to_string());

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.view_id, "view_id");
    }
}
