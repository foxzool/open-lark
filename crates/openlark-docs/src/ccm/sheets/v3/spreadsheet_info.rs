//! Sheets API 标准模板
//!
//! 使用 rust-api-mapper 工具可识别的标准模板格式
//! 基于 metainfo.rs 的现代实现模式

use serde::{Deserialize, Serialize};

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    trait_system::Service,
    SDKResult,
};
use reqwest::Method;

// ============================================================================
// 1. 数据结构定义
// ============================================================================

/// API请求体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetInfoRequest {
    // 请求字段定义
}

/// API响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetInfoResponse {
    // 响应字段定义
}

/// 响应体包装结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetInfoResponseBody {
    pub data: SpreadsheetInfoResponse,
}

// 实现API响应特征
impl ApiResponseTrait for SpreadsheetInfoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for SpreadsheetInfoResponseBody {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ============================================================================
// 2. 服务结构定义
// ============================================================================

/// 电子表格信息服务
#[derive(Clone, Debug)]
pub struct SpreadsheetInfoService {
    config: Config,
}

impl SpreadsheetInfoService {
    /// 创建服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 主要API方法
    ///
    /// # 参数
    /// - `request`: SpreadsheetInfoRequest
    ///
    /// # 返回
    /// SpreadsheetInfoResponse
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v3::spreadsheet_info::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = openlark_core::config::Config::new("app_id", "app_secret");
    /// let service = SpreadsheetInfoService::new(config);
    ///
    /// let request = SpreadsheetInfoRequest::new("{{example_param}}");
    /// let response = service.get_spreadsheet_info(request).await?;
    ///
    /// println!("操作成功: {:?}", response);
    /// ```
    pub async fn get_spreadsheet_info(
        &self,
        request: SpreadsheetInfoRequest,
    ) -> SDKResult<Response<SpreadsheetInfoResponse>> {
        // 验证请求参数
        request.validate()?;

        // 构建API请求
        let endpoint = format!(
            "{}//open-apis/sheets/v3/spreadsheets/:spreadsheet_token",
            self.config.base_url,
            // 其他参数
        );

        let mut api_req = ApiRequest::with_method(Method::GET);
        api_req.set_api_path(endpoint);
        api_req
            .set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

        // 添加查询参数（如果需要）
        // let query_params = request.build_query_params();
        // if !query_params.is_empty() {
        //     for param in query_params.split('&') {
        //         if let Some((key, value)) = param.split_once('=') {
        //             api_req.query_params.insert(key, value);
        //         }
        //     }
        // }

        // 发送请求
        Transport::<SpreadsheetInfoResponse>::request(api_req, &self.config, None).await
    }

    /// 便捷方法
    pub fn get_spreadsheet_info_builder(
        &self,
        param: impl Into<String>,
    ) -> GetSpreadsheetInfoBuilder {
        GetSpreadsheetInfoBuilder::new(self.clone(), param)
    }
}

// ============================================================================
// 3. Service Trait实现
// ============================================================================

impl Service for SpreadsheetInfoService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "SpreadsheetInfoService"
    }
}

// ============================================================================
// 4. 构建器模式（可选）
// ============================================================================

/// 电子表格信息构建器
pub struct GetSpreadsheetInfoBuilder {
    service: SpreadsheetInfoService,
    // 构建器字段
}

impl GetSpreadsheetInfoBuilder {
    /// 创建新的构建器
    pub fn new(service: SpreadsheetInfoService, param: impl Into<String>) -> Self {
        Self {
            service,
            // 初始化字段
        }
    }

    /// 设置字段（示例）
    pub fn field_name(self, value: impl Into<String>) -> Self {
        // self.field = Some(value.into());
        self
    }

    /// 执行操作
    pub async fn execute(self) -> SDKResult<Response<SpreadsheetInfoResponse>> {
        let request = SpreadsheetInfoRequest::new(/* 参数 */);

        // 设置请求字段
        // if let Some(value) = self.field {
        //     request = request.field_name(value);
        // }

        self.service.get_spreadsheet_info(request).await
    }
}

// ============================================================================
// 5. 请求实现
// ============================================================================

impl SpreadsheetInfoRequest {
    /// 创建新请求
    pub fn new(/* 参数 */) -> Self {
        Self {
            // 初始化字段
        }
    }

    /// 设置字段（示例）
    pub fn field_name(self, value: impl Into<String>) -> Self {
        // self.field = value.into();
        self
    }

    /// 验证请求参数
    pub fn validate(&self) -> SDKResult<()> {
        // 验证逻辑
        Ok(())
    }

    /// 构建查询参数（如果需要）
    pub fn build_query_params(&self) -> String {
        let params: Vec<String> = vec![];

        // 添加查询参数
        // if let Some(value) = &self.field {
        //     params.push(format!("field={}", urlencoding::encode(value)));
        // }

        params.join("&")
    }
}

// ============================================================================
// 6. 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_spreadsheet_info_request_validation() {
        let request = SpreadsheetInfoRequest::new(/* 参数 */);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_spreadsheetinfoservice_creation() {
        let config = openlark_core::config::Config::default();
        let service = SpreadsheetInfoService::new(config);
        assert_eq!(service.service_name(), "SpreadsheetInfoService");
    }

    #[test]
    fn test_builder_pattern() {
        let config = openlark_core::config::Config::default();
        let service = SpreadsheetInfoService::new(config);

        let builder = service.get_spreadsheet_info_builder("test_param");
        // 验证构建器设置
    }
}
