use std::collections::HashMap;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    config::Config,
    constants::AccessTokenType,
    request_executor::RequestExecutor,
    req_option::RequestOption,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    SDKResult,
};

/// 这是一个示例，展示如何使用RequestExecutor重构现有的API服务
/// 相比原始的MessageService实现，这个版本消除了重复代码
pub struct ModernMessageService {
    pub config: Config,
}

impl ModernMessageService {
    /// 使用RequestExecutor重构的发送消息方法
    /// 
    /// 对比原方法，这个实现：
    /// 1. 消除了手动构建ApiRequest的重复代码
    /// 2. 统一了HTTP方法、路径、认证类型的设置
    /// 3. 自动处理请求体序列化
    /// 4. 保持了相同的API接口
    pub async fn create_message_modern(
        &self,
        receive_id_type: &str,
        body: CreateMessageRequestBody,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<Message>> {
        // 构建查询参数
        let mut query_params = HashMap::new();
        query_params.insert("receive_id_type", receive_id_type.to_string());

        // 使用RequestExecutor执行请求 - 所有重复代码都被抽象了
        RequestExecutor::execute(
            &self.config,
            Method::POST,
            "/open-apis/im/v1/messages",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            Some(query_params),
            Some(body),
            option,
        )
        .await
    }

    /// 使用简化的JSON请求执行器
    /// 这个版本更加简洁，使用了默认的认证类型设置
    pub async fn create_message_simple(
        &self,
        receive_id_type: &str,
        body: CreateMessageRequestBody,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<Message>> {
        // 使用带查询参数的路径
        let path = format!("/open-apis/im/v1/messages?receive_id_type={}", receive_id_type);
        
        // 最简洁的调用方式
        RequestExecutor::json_request(&self.config, Method::POST, &path, &body, option).await
    }

    /// 获取消息列表的重构版本
    pub async fn list_messages_modern(
        &self,
        container_id: &str,
        container_id_type: &str,
        start_time: Option<&str>,
        end_time: Option<&str>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListMessageRespData>> {
        // 构建查询参数
        let mut query_params = HashMap::new();
        query_params.insert("container_id", container_id.to_string());
        query_params.insert("container_id_type", container_id_type.to_string());
        
        if let Some(start) = start_time {
            query_params.insert("start_time", start.to_string());
        }
        if let Some(end) = end_time {
            query_params.insert("end_time", end.to_string());
        }
        if let Some(token) = page_token {
            query_params.insert("page_token", token.to_string());
        }
        if let Some(size) = page_size {
            query_params.insert("page_size", size.to_string());
        }

        // 使用查询请求执行器
        RequestExecutor::query_request(
            &self.config,
            "/open-apis/im/v1/messages",
            Some(query_params),
            option,
        )
        .await
    }

    /// 使用路径参数的示例 - 获取特定消息
    pub async fn get_message_by_id(
        &self,
        message_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<Message>> {
        let path_params = HashMap::from([("message_id", message_id)]);

        RequestExecutor::execute_with_path_params(
            &self.config,
            Method::GET,
            "/open-apis/im/v1/messages/{message_id}",
            path_params,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            None,
            None::<()>,
            option,
        )
        .await
    }

    /// 删除消息的示例
    pub async fn delete_message(
        &self,
        message_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let path_params = HashMap::from([("message_id", message_id)]);

        RequestExecutor::execute_with_path_params(
            &self.config,
            Method::DELETE,
            "/open-apis/im/v1/messages/{message_id}",
            path_params,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            None,
            None::<()>,
            option,
        )
        .await
    }

    /// 更新消息的示例
    pub async fn update_message(
        &self,
        message_id: &str,
        body: UpdateMessageRequestBody,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<Message>> {
        let path_params = HashMap::from([("message_id", message_id)]);

        RequestExecutor::execute_with_path_params(
            &self.config,
            Method::PUT,
            "/open-apis/im/v1/messages/{message_id}",
            path_params,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            None,
            Some(body),
            option,
        )
        .await
    }
}

// 重用现有的数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMessageRequestBody {
    pub receive_id: String,
    pub msg_type: String,
    pub content: String,
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMessageRequestBody {
    pub content: String,
    pub msg_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: String,
    pub root_id: Option<String>,
    pub parent_id: Option<String>,
    pub thread_id: Option<String>,
    pub msg_type: String,
    pub create_time: String,
    pub update_time: String,
    pub deleted: bool,
    pub updated: bool,
    pub chat_id: String,
    pub sender: MessageSender,
    pub body: MessageBody,
}

#[derive(Debug, Deserialize)]
pub struct MessageSender {
    pub id: String,
    pub id_type: String,
    pub sender_type: String,
    pub tenant_key: String,
}

#[derive(Debug, Deserialize)]
pub struct MessageBody {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ListMessageRespData {
    pub has_more: bool,
    pub page_token: Option<String>,
    pub items: Vec<Message>,
}

#[derive(Debug, Deserialize)]
pub struct EmptyResponse {}

// 实现ApiResponseTrait
impl ApiResponseTrait for Message {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for ListMessageRespData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for EmptyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::core::config::Config;

    #[tokio::test]
    async fn test_modern_message_service_api() {
        // 这个测试展示了新API的使用方式
        let config = Config::default(); // 在实际使用中需要正确的配置
        let service = ModernMessageService { config };

        let body = CreateMessageRequestBody {
            receive_id: "ou_test123".to_string(),
            msg_type: "text".to_string(),
            content: r#"{"text":"Hello, World!"}"#.to_string(),
            uuid: Some("test-uuid".to_string()),
        };

        // 注意：这个测试不会实际运行，因为没有真实的配置
        // 但它展示了API的使用方式
        println!("Modern API example:");
        println!("service.create_message_modern('open_id', body, None).await");
        
        // 对比原始API调用方式：
        // 1. 需要手动构建CreateMessageRequest
        // 2. 需要使用builder模式设置receive_id_type
        // 3. 需要手动设置request_body
        // 4. 调用create方法
        
        // 新API的优势：
        // 1. 直接传递参数，无需复杂的builder
        // 2. 更清晰的函数签名
        // 3. 更少的样板代码
    }
}

/// 使用RequestExecutor进行批量操作的示例
pub struct BulkMessageOperations;

impl BulkMessageOperations {
    /// 批量发送消息
    pub async fn send_bulk_messages(
        config: &Config,
        messages: Vec<(String, CreateMessageRequestBody)>, // (receive_id_type, body)
        option: Option<RequestOption>,
    ) -> Vec<SDKResult<BaseResponse<Message>>> {
        let mut results = Vec::new();
        
        for (receive_id_type, body) in messages {
            let mut query_params = HashMap::new();
            query_params.insert("receive_id_type", receive_id_type);

            let result = RequestExecutor::execute(
                config,
                Method::POST,
                "/open-apis/im/v1/messages",
                vec![AccessTokenType::Tenant, AccessTokenType::User],
                Some(query_params),
                Some(body),
                option.clone(),
            )
            .await;
            
            results.push(result);
        }
        
        results
    }
}

/// 总结：RequestExecutor的优势
/// 
/// 1. **消除重复代码**：
///    - 原始方法：每个API都需要手动设置http_method、api_path、supported_access_token_types
///    - 新方法：这些设置被抽象到RequestExecutor中
/// 
/// 2. **统一错误处理**：
///    - 所有请求都通过相同的错误处理流程
///    - 序列化错误统一处理
/// 
/// 3. **更好的类型安全**：
///    - 泛型确保响应类型正确
///    - 编译时检查确保API契约
/// 
/// 4. **更简洁的API**：
///    - query_request用于GET请求
///    - json_request用于JSON POST/PUT请求
///    - execute_with_path_params处理路径参数
/// 
/// 5. **更好的可测试性**：
///    - 可以轻松模拟RequestExecutor
///    - 更清晰的函数签名便于单元测试
/// 
/// 6. **向后兼容**：
///    - 可以逐步迁移现有API
///    - 保持相同的外部接口