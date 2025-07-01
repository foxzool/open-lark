use std::collections::HashMap;

use reqwest::Method;
use serde::Serialize;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 通用请求执行器，统一处理API调用逻辑
/// 消除重复的请求-响应处理代码，提供统一的API调用入口
pub struct RequestExecutor;

impl RequestExecutor {
    /// 执行GET请求
    pub async fn get<T: ApiResponseTrait>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        query_params: Option<HashMap<String, String>>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
        Self::execute(
            config,
            Method::GET,
            path,
            supported_tokens,
            query_params,
            None::<()>,
            option,
        )
        .await
    }

    /// 执行POST请求
    pub async fn post<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
        Self::execute(
            config,
            Method::POST,
            path,
            supported_tokens,
            None,
            body,
            option,
        )
        .await
    }

    /// 执行PUT请求
    pub async fn put<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
        Self::execute(
            config,
            Method::PUT,
            path,
            supported_tokens,
            None,
            body,
            option,
        )
        .await
    }

    /// 执行DELETE请求
    pub async fn delete<T: ApiResponseTrait>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
        Self::execute(
            config,
            Method::DELETE,
            path,
            supported_tokens,
            None,
            None::<()>,
            option,
        )
        .await
    }

    /// 执行PATCH请求
    pub async fn patch<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
        Self::execute(
            config,
            Method::PATCH,
            path,
            supported_tokens,
            None,
            body,
            option,
        )
        .await
    }

    /// 通用请求执行器核心方法
    ///
    /// # 参数
    /// - `config`: 应用配置，包含认证信息
    /// - `method`: HTTP方法
    /// - `path`: API路径
    /// - `supported_tokens`: 支持的访问令牌类型
    /// - `query_params`: 查询参数（可选）
    /// - `body`: 请求体（可选）
    /// - `option`: 请求选项（可选）
    ///
    /// # 返回值
    /// 返回标准的`BaseResponse<T>`格式响应
    ///
    /// # 示例
    /// ```rust,ignore
    /// // GET请求
    /// let response: BaseResponse<MessageList> = RequestExecutor::execute(
    ///     &config,
    ///     Method::GET,
    ///     "/open-apis/im/v1/messages",
    ///     vec![AccessTokenType::Tenant, AccessTokenType::User],
    ///     Some(query_params),
    ///     None::<()>,
    ///     None,
    /// ).await?;
    ///
    /// // POST请求
    /// let response: BaseResponse<Message> = RequestExecutor::execute(
    ///     &config,
    ///     Method::POST,
    ///     "/open-apis/im/v1/messages",
    ///     vec![AccessTokenType::Tenant, AccessTokenType::User],
    ///     None,
    ///     Some(create_request),
    ///     None,
    /// ).await?;
    /// ```
    pub async fn execute<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        method: Method,
        path: &str,
        supported_tokens: Vec<AccessTokenType>,
        query_params: Option<HashMap<String, String>>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
        let mut api_req = ApiRequest {
            http_method: method,
            api_path: path.to_string(),
            supported_access_token_types: supported_tokens,
            ..Default::default()
        };

        // 设置查询参数
        if let Some(params) = query_params {
            api_req.query_params = params;
        }

        // 序列化请求体
        if let Some(body_data) = body {
            api_req.body = serde_json::to_vec(&body_data)
                .map_err(|e| crate::core::error::LarkAPIError::DeserializeError(e.to_string()))?;
        }

        // 执行请求
        Transport::request(api_req, config, option).await
    }

    /// 带路径参数的请求执行器
    /// 支持在路径中动态替换参数，如 `/open-apis/im/v1/messages/{message_id}`
    ///
    /// # 参数
    /// - `path_template`: 包含占位符的路径模板，如 "/open-apis/im/v1/messages/{message_id}"
    /// - `path_params`: 路径参数映射，如 HashMap::from([("message_id", "om_xxx")])
    /// - 其他参数同 `execute` 方法
    ///
    /// # 示例
    /// ```rust,ignore
    /// let path_params = HashMap::from([("message_id", "om_xxx")]);
    /// let response = RequestExecutor::execute_with_path_params(
    ///     &config,
    ///     Method::GET,
    ///     "/open-apis/im/v1/messages/{message_id}",
    ///     path_params,
    ///     vec![AccessTokenType::Tenant],
    ///     None,
    ///     None::<()>,
    ///     None,
    /// ).await?;
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub async fn execute_with_path_params<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        method: Method,
        path_template: &str,
        path_params: HashMap<&str, &str>,
        supported_tokens: Vec<AccessTokenType>,
        query_params: Option<HashMap<String, String>>,
        body: Option<B>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
        // 替换路径参数
        let mut path = path_template.to_string();
        for (key, value) in path_params {
            path = path.replace(&format!("{{{key}}}"), value);
        }

        Self::execute(
            config,
            method,
            &path,
            supported_tokens,
            query_params,
            body,
            option,
        )
        .await
    }

    /// 简化的JSON请求执行器
    /// 自动序列化JSON请求体并设置标准的租户/用户令牌支持
    ///
    /// # 示例
    /// ```rust,ignore
    /// // 创建消息
    /// let response = RequestExecutor::json_request::<CreateMessageResponse, _>(
    ///     &config,
    ///     Method::POST,
    ///     "/open-apis/im/v1/messages",
    ///     &create_message_request,
    ///     None,
    /// ).await?;
    /// ```
    pub async fn json_request<T: ApiResponseTrait, B: Serialize>(
        config: &Config,
        method: Method,
        path: &str,
        body: &B,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
        Self::execute(
            config,
            method,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User], // 默认支持租户和用户令牌
            None,
            Some(body),
            option,
        )
        .await
    }

    /// 简化的查询请求执行器
    /// 用于GET请求，自动设置标准的租户/用户令牌支持
    ///
    /// # 示例
    /// ```rust,ignore
    /// // 获取消息列表
    /// let mut query_params = HashMap::new();
    /// query_params.insert("container_id".to_string(), chat_id);
    /// let response = RequestExecutor::query_request::<MessageListResponse>(
    ///     &config,
    ///     "/open-apis/im/v1/messages",
    ///     Some(query_params),
    ///     None,
    /// ).await?;
    /// ```
    pub async fn query_request<T: ApiResponseTrait>(
        config: &Config,
        path: &str,
        query_params: Option<HashMap<String, String>>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<T>> {
        Self::get(
            config,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User], // 默认支持租户和用户令牌
            query_params,
            option,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::api_resp::ResponseFormat;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    struct TestRequest {
        message: String,
    }

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct TestResponse {
        id: String,
        status: String,
    }

    impl ApiResponseTrait for TestResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    #[tokio::test]
    async fn test_request_executor_path_params() {
        let path_template = "/open-apis/im/v1/messages/{message_id}/replies/{reply_id}";
        let path_params = HashMap::from([("message_id", "om_123"), ("reply_id", "reply_456")]);

        let expected_path = "/open-apis/im/v1/messages/om_123/replies/reply_456";

        // 测试路径参数替换
        let mut path = path_template.to_string();
        for (key, value) in path_params {
            path = path.replace(&format!("{{{key}}}"), value);
        }

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_request_body_serialization() {
        let request = TestRequest {
            message: "Hello, World!".to_string(),
        };

        let serialized = serde_json::to_vec(&request).unwrap();
        let expected = br#"{"message":"Hello, World!"}"#;

        assert_eq!(serialized, expected);
    }
}
