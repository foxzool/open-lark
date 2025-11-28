//! HTTP Mock 服务器
//!
//! 提供 WireMock 基础设施，用于模拟飞书开放平台 API 响应

use wiremock::{Mock, MockServer, ResponseTemplate};
use serde_json::json;
use crate::testing::{TestDataFactory, TestConfig};

/// HTTP Mock 服务器管理器
pub struct MockServerManager {
    server: MockServer,
    config: TestConfig,
}

impl MockServerManager {
    /// 创建新的 Mock 服务器
    pub async fn new(config: TestConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let server = MockServer::start().await;
        Ok(Self { server, config })
    }

    /// 获取服务器 URI
    pub fn uri(&self) -> String {
        self.server.uri()
    }

    /// 获取服务器端口
    pub fn port(&self) -> u16 {
        self.server.address().port()
    }

    /// 注册所有基础 API Mock
    pub async fn register_base_mocks(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.register_app_mocks().await?;
        self.register_table_mocks().await?;
        self.register_record_mocks().await?;
        self.register_role_mocks().await?;
        Ok(())
    }

    /// 注册应用相关 API Mock
    async fn register_app_mocks(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 应用基础路径模式
        let app_pattern = r"^/open-apis/bitable/v1/apps/[^/]+$";

        // 创建应用
        Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path_regex(app_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_app()
            ))
            .mount(&self.server)
            .await;

        // 获取应用信息
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path_regex(app_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_app()
            ))
            .mount(&self.server)
            .await;

        // 更新应用信息
        Mock::given(wiremock::matchers::method("PATCH"))
            .and(wiremock::matchers::path_regex(app_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_app()
            ))
            .mount(&self.server)
            .await;

        // 删除应用
        Mock::given(wiremock::matchers::method("DELETE"))
            .and(wiremock::matchers::path_regex(app_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true
            })))
            .mount(&self.server)
            .await;

        Ok(())
    }

    /// 注册数据表相关 API Mock
    async fn register_table_mocks(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 数据表列表路径
        let tables_pattern = r"^/open-apis/bitable/v1/apps/[^/]+/tables$";
        let table_pattern = r"^/open-apis/bitable/v1/apps/[^/]+/tables/[^/]+$";

        // 创建数据表
        Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path_regex(tables_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_table()
            ))
            .mount(&self.server)
            .await;

        // 获取数据表列表
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path_regex(tables_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                json!({
                    "tables": [TestDataFactory::create_test_table()],
                    "page_token": null,
                    "has_more": false
                })
            ))
            .mount(&self.server)
            .await;

        // 获取单个数据表
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path_regex(table_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_table()
            ))
            .mount(&self.server)
            .await;

        // 更新数据表
        Mock::given(wiremock::matchers::method("PUT"))
            .and(wiremock::matchers::path_regex(table_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_table()
            ))
            .mount(&self.server)
            .await;

        // 删除数据表
        Mock::given(wiremock::matchers::method("DELETE"))
            .and(wiremock::matchers::path_regex(table_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true
            })))
            .mount(&self.server)
            .await;

        Ok(())
    }

    /// 注册记录相关 API Mock
    async fn register_record_mocks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let records_pattern = r"^/open-apis/bitable/v1/apps/[^/]+/tables/[^/]+/records$";
        let record_pattern = r"^/open-apis/bitable/v1/apps/[^/]+/tables/[^/]+/records/[^/]+$";

        // 创建记录
        Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path_regex(records_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_record()
            ))
            .mount(&self.server)
            .await;

        // 获取记录列表
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path_regex(records_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_paginated_response(50, 20, None)
            ))
            .mount(&self.server)
            .await;

        // 获取单个记录
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path_regex(record_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_record()
            ))
            .mount(&self.server)
            .await;

        // 更新记录
        Mock::given(wiremock::matchers::method("PUT"))
            .and(wiremock::matchers::path_regex(record_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_record()
            ))
            .mount(&self.server)
            .await;

        // 删除记录
        Mock::given(wiremock::matchers::method("DELETE"))
            .and(wiremock::matchers::path_regex(record_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "success": true
            })))
            .mount(&self.server)
            .await;

        // 搜索记录
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path_regex(&format!("{}/search", records_pattern)))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                json!({
                    "items": TestDataFactory::create_batch_test_records(5),
                    "page_token": null,
                    "has_more": false,
                    "total": 5
                })
            ))
            .mount(&self.server)
            .await;

        Ok(())
    }

    /// 注册角色相关 API Mock
    async fn register_role_mocks(&self) -> Result<(), Box<dyn std::error::Error>> {
        let roles_pattern = r"^/open-apis/base/v2/apps/[^/]+/roles$";
        let role_pattern = r"^/open-apis/base/v2/apps/[^/]+/roles/[^/]+$";

        // 创建角色
        Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path_regex(roles_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_role()
            ))
            .mount(&self.server)
            .await;

        // 获取角色列表
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path_regex(roles_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                json!({
                    "items": [TestDataFactory::create_test_role()],
                    "page_token": null,
                    "has_more": false,
                    "total": 1
                })
            ))
            .mount(&self.server)
            .await;

        // 更新角色
        Mock::given(wiremock::matchers::method("PUT"))
            .and(wiremock::matchers::path_regex(role_pattern))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                TestDataFactory::create_test_role()
            ))
            .mount(&self.server)
            .await;

        Ok(())
    }

    /// 注册错误响应 Mock
    pub async fn register_error_mocks(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 404 错误
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path_regex(r"/not-found"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "error": {
                    "code": 404,
                    "message": "Not Found"
                }
            })))
            .mount(&self.server)
            .await;

        // 401 权限错误
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path_regex(r"/unauthorized"))
            .respond_with(ResponseTemplate::new(401).set_body_json(json!({
                "error": {
                    "code": 401,
                    "message": "Unauthorized"
                }
            })))
            .mount(&self.server)
            .await;

        // 500 服务器错误
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path_regex(r"/server-error"))
            .respond_with(ResponseTemplate::new(500).set_body_json(json!({
                "error": {
                    "code": 500,
                    "message": "Internal Server Error"
                }
            })))
            .mount(&self.server)
            .await;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_server_creation() {
        let config = TestConfig::new();
        let server_manager = MockServerManager::new(config).await.unwrap();

        assert!(!server_manager.uri().is_empty());
        assert!(server_manager.port() > 0);
    }

    #[tokio::test]
    async fn test_register_base_mocks() {
        let config = TestConfig::new();
        let server_manager = MockServerManager::new(config).await.unwrap();

        let result = server_manager.register_base_mocks().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_error_mocks() {
        let config = TestConfig::new();
        let server_manager = MockServerManager::new(config).await.unwrap();

        let result = server_manager.register_error_mocks().await;
        assert!(result.is_ok());
    }
}