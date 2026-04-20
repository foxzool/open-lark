//! 统一的 HTTP Mock 服务器封装

use serde_json::Value;
#[cfg(test)]
use wiremock::matchers::{body_json, method, path as path_matcher};
#[cfg(test)]
use wiremock::{Mock, MockServer, ResponseTemplate};

/// 统一的 HTTP Mock 服务器封装，提供简洁的 API 来 Mock HTTP 请求。
#[cfg(test)]
pub struct TestServer {
    inner: MockServer,
}

#[cfg(test)]
impl TestServer {
    /// 启动一个新的 mock HTTP 服务器。
    pub async fn new() -> Self {
        Self {
            inner: MockServer::start().await,
        }
    }

    /// 返回 mock 服务器的基础地址。
    pub fn uri(&self) -> String {
        self.inner.uri()
    }

    /// Mock 一个成功的 POST 请求响应。
    pub async fn mock_success(&self, route: &str, body: Value) {
        Mock::given(method("POST"))
            .and(path_matcher(route))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&self.inner)
            .await;
    }

    /// Mock 一个带自定义状态码的 POST 错误响应。
    pub async fn mock_error(&self, route: &str, code: u16, error: Value) {
        Mock::given(method("POST"))
            .and(path_matcher(route))
            .respond_with(ResponseTemplate::new(code).set_body_json(error))
            .mount(&self.inner)
            .await;
    }

    /// Mock 一个会延迟返回的 POST 请求。
    pub async fn mock_timeout(&self, route: &str, delay: std::time::Duration) {
        Mock::given(method("POST"))
            .and(path_matcher(route))
            .respond_with(ResponseTemplate::new(200).set_delay(delay))
            .mount(&self.inner)
            .await;
    }

    /// Mock 一个校验请求体后返回成功结果的 POST 请求。
    pub async fn mock_with_verification(&self, route: &str, expected_body: Value, response: Value) {
        Mock::given(method("POST"))
            .and(path_matcher(route))
            .and(body_json(&expected_body))
            .respond_with(ResponseTemplate::new(200).set_body_json(response))
            .mount(&self.inner)
            .await;
    }

    /// Mock 一个成功的 GET 请求响应。
    pub async fn mock_get(&self, route: &str, body: Value) {
        Mock::given(method("GET"))
            .and(path_matcher(route))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&self.inner)
            .await;
    }

    /// Mock 一个成功的 PUT 请求响应。
    pub async fn mock_put(&self, route: &str, body: Value) {
        Mock::given(method("PUT"))
            .and(path_matcher(route))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&self.inner)
            .await;
    }

    /// Mock 一个成功的 DELETE 请求响应。
    pub async fn mock_delete(&self, route: &str, body: Value) {
        Mock::given(method("DELETE"))
            .and(path_matcher(route))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&self.inner)
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_mock_server_basic() {
        let server = TestServer::new().await;
        assert!(!server.uri().is_empty());
    }

    #[tokio::test]
    async fn test_mock_success() {
        let server = TestServer::new().await;
        server
            .mock_success("/api/v1/test", json!({"code": 0}))
            .await;
    }

    #[tokio::test]
    async fn test_mock_error() {
        let server = TestServer::new().await;
        server
            .mock_error("/api/v1/error", 400, json!({"code": 99991663}))
            .await;
    }

    #[tokio::test]
    async fn test_mock_timeout() {
        let server = TestServer::new().await;
        server
            .mock_timeout("/api/v1/slow", std::time::Duration::from_millis(100))
            .await;
    }
}
