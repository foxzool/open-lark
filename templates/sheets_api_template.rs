//! Sheets API 标准模板
//!
//! 使用 rust-api-mapper 工具可识别的标准模板格式
//! 基于 metainfo.rs 的现代实现模式

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat, BaseResponse},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
    standard_response::StandardResponse,
    error::LarkAPIError,
    trait_system::Service,
};

// ============================================================================
// 1. 数据结构定义
// ============================================================================

/// API请求体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{RequestStruct}} {
    // 请求字段定义
}

/// API响应体结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ResponseStruct}} {
    // 响应字段定义
}

/// 响应体包装结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{ResponseBodyStruct}} {
    pub data: {{ResponseStruct}},
}

// 实现API响应特征
impl ApiResponseTrait for {{ResponseStruct}} {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for {{ResponseBodyStruct}} {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ============================================================================
// 2. 服务结构定义
// ============================================================================

/// {{ServiceName}}服务
#[derive(Debug, Clone)]
pub struct {{ServiceStruct}} {
    config: Config,
}

impl {{ServiceStruct}} {
    /// 创建服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 主要API方法
    ///
    /// # 参数
    /// - `request`: {{RequestStruct}}
    ///
    /// # 返回
    /// {{ResponseStruct}}
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::service::sheets::v{{version}}::{{filename}}::*;
    /// use open_lark::core::config::Config;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = {{ServiceStruct}}::new(config);
    ///
    /// let request = {{RequestStruct}}::new("{{example_param}}");
    /// let response = service.{{method_name}}(request).await?;
    ///
    /// println!("操作成功: {:?}", response);
    /// ```
    pub async fn {{method_name}}(&self, request: {{RequestStruct}}) -> SDKResult<BaseResponse<{{ResponseStruct}}>> {
        // 验证请求参数
        request.validate()?;

        // 构建API请求
        let endpoint = format!(
            "{}/{{api_path}}",
            self.config.base_url,
            // 其他参数
        );

        let mut api_req = ApiRequest::with_method(reqwest::Method::{{HttpMethod}});
        api_req.set_api_path(endpoint);
        api_req.set_supported_access_token_types(vec![
            AccessTokenType::Tenant,
            AccessTokenType::User
        ]);

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
        Transport::<{{ResponseStruct}}>::request(api_req, &self.config, None).await
    }

    /// 便捷方法
    pub fn {{method_name}}_builder(&self, {{builder_param}}: impl Into<String>) -> {{BuilderStruct}} {
        {{BuilderStruct}}::new(self.clone(), {{builder_param}})
    }
}

// ============================================================================
// 3. Service Trait实现
// ============================================================================

impl Service for {{ServiceStruct}} {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "{{ServiceStruct}}"
    }
}

// ============================================================================
// 4. 构建器模式（可选）
// ============================================================================

/// {{ServiceName}}构建器
pub struct {{BuilderStruct}} {
    service: {{ServiceStruct}},
    // 构建器字段
}

impl {{BuilderStruct}} {
    /// 创建新的构建器
    pub fn new(service: {{ServiceStruct}}, {{builder_param}}: impl Into<String>) -> Self {
        Self {
            service,
            // 初始化字段
        }
    }

    /// 设置字段（示例）
    pub fn field_name(mut self, value: impl Into<String>) -> Self {
        // self.field = Some(value.into());
        self
    }

    /// 执行操作
    pub async fn execute(self) -> SDKResult<BaseResponse<{{ResponseStruct}}>> {
        let mut request = {{RequestStruct}}::new(/* 参数 */);

        // 设置请求字段
        // if let Some(value) = self.field {
        //     request = request.field_name(value);
        // }

        self.service.{{method_name}}(request).await
    }
}

// ============================================================================
// 5. 请求实现
// ============================================================================

impl {{RequestStruct}} {
    /// 创建新请求
    pub fn new(/* 参数 */) -> Self {
        Self {
            // 初始化字段
        }
    }

    /// 设置字段（示例）
    pub fn field_name(mut self, value: impl Into<String>) -> Self {
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
        let mut params = Vec::new();

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
    fn test_{{method_name}}_request_validation() {
        let request = {{RequestStruct}}::new(/* 参数 */);
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_{{service_struct}}_creation() {
        let config = Config::default();
        let service = {{ServiceStruct}}::new(config);
        assert_eq!(service.service_name(), "{{ServiceStruct}}");
    }

    #[test]
    fn test_builder_pattern() {
        let config = Config::default();
        let service = {{ServiceStruct}}::new(config);

        let builder = service.{{method_name}}_builder("test_param");
        // 验证构建器设置
    }
}