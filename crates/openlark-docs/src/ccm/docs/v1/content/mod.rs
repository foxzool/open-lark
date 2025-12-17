/// 文档内容模块
///
/// 提供云文档内容的获取功能，包括文档详细信息、文本内容等。
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_mut)]
#[allow(non_snake_case)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::module_inception)]
use openlark_core::{
    api::Response, config::Config, error::validation_error, req_option::RequestOption, SDKResult,
};

// 重新导出所有模块类型
pub use get::*;

// 子模块
mod get;

/// 文档内容服务
///
/// 提供云文档内容的完整管理功能，包括获取文档详细内容等。
/// 支持多种文档格式的内容访问和处理。
#[derive(Clone)]
pub struct ContentService {
    config: Config,
}

impl ContentService {
    /// 创建新的文档内容服务实例
    ///
    /// # 参数
    /// * `config` - SDK配置信息
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取云文档内容
    ///
    /// 获取指定云文档的详细内容，包括文档结构、文本内容等。
    /// 支持多种文档格式，如文档、表格、思维笔记等。
    ///
    /// # 参数
    /// * `request` - 获取文档内容请求
    /// * `option` - 可选请求配置
    ///
    /// # 返回
    /// 成功返回文档内容信息，失败返回错误信息
    ///
    /// # 示例
    /// ```rust,no_run
    /// use open_lark::service::cloud_docs::docs::v1::content::{ContentService, GetDocsContentRequest};
    ///
    /// let service = ContentService::new(config);
    /// let request = GetDocsContentRequest::new("document_token");
    ///
    /// let response = service.get(request, None).await?;
    /// println!("文档内容获取成功");
    /// ```
    pub async fn get(
        &self,
        request: GetDocsContentRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<serde_json::Value> {
        let response = get_docs_content(request, &self.config, option).await?;
        let resp_data = response
            .data
            .ok_or_else(|| validation_error("response_data", "Response data is missing"))?;
        resp_data
            .data
            .ok_or_else(|| validation_error("data", "Docs content data is missing"))
    }
}

impl openlark_core::trait_system::service::Service for ContentService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "content"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::trait_system::service::Service;

    fn create_test_service() -> ContentService {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        ContentService::new(config)
    }

    #[test]
    fn test_content_service_creation() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let service = ContentService::new(config);

        assert_eq!(service.config().app_id(), "test_app_id");
        assert_eq!(service.config().app_secret(), "test_app_secret");
    }

    #[test]
    fn test_content_service_clone() {
        let service = create_test_service();
        let cloned_service = service.clone();

        assert_eq!(service.config().app_id(), cloned_service.config().app_id());
        assert_eq!(
            service.config().app_secret(),
            cloned_service.config().app_secret()
        );
    }

    #[test]
    fn test_service_trait_implementation() {
        let service = create_test_service();

        // 测试Service trait的实现
        let config_ref = service.config();
        assert_eq!(config_ref.app_id(), "test_app_id");
    }

    #[test]
    fn test_get_docs_content_builder() {
        let request = GetDocsContentRequest::new("document_token");

        assert_eq!(request.document_token, "document_token");
    }

    #[test]
    fn test_module_structure() {
        // 这个测试验证模块结构的完整性
        let service = create_test_service();

        // 验证可以访问所有服务方法
        let _get_request = GetDocsContentRequest::new("document_token");

        // 如果编译通过，说明模块结构正确
        assert!(true);
    }
}
