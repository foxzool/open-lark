// 统一构建器框架 POC 实现
// 基于Codex评估修正后的版本

use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

/// 模拟的SDK错误类型
#[derive(Debug)]
pub enum SDKError {
    InvalidInput(String),
    NetworkError(String),
    SerializationError(String),
}

impl std::fmt::Display for SDKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SDKError::InvalidInput(msg) => write!(f, "输入无效: {}", msg),
            SDKError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            SDKError::SerializationError(msg) => write!(f, "序列化错误: {}", msg),
        }
    }
}

impl std::error::Error for SDKError {}

/// 模拟的请求和响应类型
#[derive(Debug, Clone)]
pub struct MessageSendRequest {
    pub text: String,
    pub receive_id: String,
    pub msg_type: String,
}

#[derive(Debug, Clone)]
pub struct MessageSendResponse {
    pub message_id: String,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct UserCreateRequest {
    pub name: String,
    pub department_id: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserCreateResponse {
    pub user_id: String,
    pub invite_url: Option<String>,
}

/// API 服务基础特征
#[async_trait::async_trait]
pub trait ApiService: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn send_message(&self, request: MessageSendRequest) -> Result<MessageSendResponse, Self::Error>;
    async fn create_user(&self, request: UserCreateRequest) -> Result<UserCreateResponse, Self::Error>;
}

/// 模拟的消息服务
#[derive(Debug)]
pub struct MockMessageService;

#[async_trait::async_trait]
impl ApiService for MockMessageService {
    type Error = SDKError;

    async fn send_message(&self, request: MessageSendRequest) -> Result<MessageSendResponse, Self::Error> {
        // 模拟异步API调用
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        if request.text.is_empty() {
            return Err(SDKError::InvalidInput("消息内容不能为空".to_string()));
        }

        if request.receive_id.is_empty() {
            return Err(SDKError::InvalidInput("接收者ID不能为空".to_string()));
        }

        Ok(MessageSendResponse {
            message_id: format!("msg_{}", rand::random::<u32>()),
            success: true,
        })
    }

    async fn create_user(&self, request: UserCreateRequest) -> Result<UserCreateResponse, Self::Error> {
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        if request.name.is_empty() {
            return Err(SDKError::InvalidInput("用户名不能为空".to_string()));
        }

        Ok(UserCreateResponse {
            user_id: format!("user_{}", rand::random::<u32>()),
            invite_url: Some(format!("https://example.com/invite/{}", rand::random::<u32>())),
        })
    }
}

/// 统一的端点构建器接口
#[async_trait::async_trait]
pub trait EndpointBuilder: Sized {
    type Service: ApiService;
    type Request: Send + Sync + 'static;
    type Response: Send + Sync + 'static;
    type Error: std::error::Error + Send + Sync + 'static;

    /// 参数验证
    fn validate(&self) -> Result<(), Self::Error>;

    /// 构建请求对象
    fn build(self) -> Self::Request;

    /// 执行 API 调用
    async fn execute(self, service: &Self::Service) -> Result<Self::Response, Self::Error>;
}

/// 消息发送构建器
pub struct MessageSendBuilder {
    text: Option<String>,
    receive_id: Option<String>,
    msg_type: Option<String>,
}

impl MessageSendBuilder {
    pub fn new() -> Self {
        Self {
            text: None,
            receive_id: None,
            msg_type: Some("text".to_string()),
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn receive_id(mut self, receive_id: impl Into<String>) -> Self {
        self.receive_id = Some(receive_id.into());
        self
    }

    pub fn msg_type(mut self, msg_type: impl Into<String>) -> Self {
        self.msg_type = Some(msg_type.into());
        self
    }
}

#[async_trait::async_trait]
impl EndpointBuilder for MessageSendBuilder {
    type Service = MockMessageService;
    type Request = MessageSendRequest;
    type Response = MessageSendResponse;
    type Error = SDKError;

    fn validate(&self) -> Result<(), Self::Error> {
        if self.text.as_ref().map_or(true, |t| t.is_empty()) {
            return Err(SDKError::InvalidInput("消息内容不能为空".to_string()));
        }
        if self.receive_id.is_none() {
            return Err(SDKError::InvalidInput("接收者ID不能为空".to_string()));
        }
        Ok(())
    }

    fn build(self) -> Self::Request {
        MessageSendRequest {
            text: self.text.unwrap_or_default(),
            receive_id: self.receive_id.unwrap_or_default(),
            msg_type: self.msg_type.unwrap_or_else(|| "text".to_string()),
        }
    }

    async fn execute(self, service: &Self::Service) -> Result<Self::Response, Self::Error> {
        self.validate()?;
        let req = self.build();
        service.send_message(req).await
    }
}

/// 用户创建构建器
pub struct UserCreateBuilder {
    name: Option<String>,
    department_id: Option<String>,
    email: Option<String>,
}

impl UserCreateBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            department_id: None,
            email: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = Some(department_id.into());
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
}

#[async_trait::async_trait]
impl EndpointBuilder for UserCreateBuilder {
    type Service = MockMessageService;
    type Request = UserCreateRequest;
    type Response = UserCreateResponse;
    type Error = SDKError;

    fn validate(&self) -> Result<(), Self::Error> {
        if self.name.as_ref().map_or(true, |n| n.is_empty()) {
            return Err(SDKError::InvalidInput("用户名不能为空".to_string()));
        }
        if self.department_id.as_ref().map_or(true, |d| d.is_empty()) {
            return Err(SDKError::InvalidInput("部门ID不能为空".to_string()));
        }
        Ok(())
    }

    fn build(self) -> Self::Request {
        UserCreateRequest {
            name: self.name.unwrap_or_default(),
            department_id: self.department_id.unwrap_or_default(),
            email: self.email,
        }
    }

    async fn execute(self, service: &Self::Service) -> Result<Self::Response, Self::Error> {
        self.validate()?;
        let req = self.build();
        service.create_user(req).await
    }
}

/// 向后兼容的宏辅助实现
macro_rules! impl_executable_builder {
    ($builder_type:ty, $service_type:ty, $request_type:ty, $response_type:ty, $method:ident) => {
        impl $builder_type {
            pub async fn execute(self, service: &$service_type) -> Result<$response_type, SDKError> {
                let request = self.build();
                service.$method(request).await
            }
        }
    };
}

// 使用宏为其他构建器提供兼容性
impl_executable_builder!(
    MessageSendBuilder,
    MockMessageService,
    MessageSendRequest,
    MessageSendResponse,
    send_message
);

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_message_send_builder_success() {
        let service = MockMessageService;

        let result = MessageSendBuilder::new()
            .text("Hello, World!")
            .receive_id("user_123")
            .execute(&service)
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.success);
        assert!(!response.message_id.is_empty());
    }

    #[tokio::test]
    async fn test_message_send_builder_validation_error() {
        let service = MockMessageService;

        // 测试空文本
        let result = MessageSendBuilder::new()
            .text("")
            .receive_id("user_123")
            .execute(&service)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            SDKError::InvalidInput(msg) => assert!(msg.contains("消息内容不能为空")),
            _ => panic!("期望 InvalidInput 错误"),
        }

        // 测试缺少接收者ID
        let result = MessageSendBuilder::new()
            .text("Hello")
            .execute(&service)
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            SDKError::InvalidInput(msg) => assert!(msg.contains("接收者ID不能为空")),
            _ => panic!("期望 InvalidInput 错误"),
        }
    }

    #[tokio::test]
    async fn test_user_create_builder_success() {
        let service = MockMessageService;

        let result = UserCreateBuilder::new()
            .name("张三")
            .department_id("dept_001")
            .email("zhangsan@example.com")
            .execute(&service)
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.user_id.is_empty());
        assert!(response.invite_url.is_some());
    }

    #[tokio::test]
    async fn test_builder_chaining_and_fluent_api() {
        let service = MockMessageService;

        // 测试流畅的链式调用
        let result = MessageSendBuilder::new()
            .text("测试消息")
            .receive_id("test_user")
            .msg_type("interactive")
            .execute(&service)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_performance_baseline() {
        let service = MockMessageService;

        let start = std::time::Instant::now();

        // 执行多个构建器操作
        for i in 0..100 {
            let _result = MessageSendBuilder::new()
                .text(format!("消息 {}", i))
                .receive_id(format!("user_{}", i))
                .execute(&service)
                .await;
        }

        let duration = start.elapsed();
        println!("100次构建器操作耗时: {:?}", duration);

        // 基本性能断言（100次操作应该在合理时间内完成）
        assert!(duration.as_secs() < 10); // 小于10秒
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("统一构建器框架 POC 测试");

    let service = MockMessageService;

    // 测试消息发送构建器
    println!("\n=== 测试消息发送构建器 ===");
    match MessageSendBuilder::new()
        .text("你好，这是一条测试消息！")
        .receive_id("test_user_001")
        .execute(&service)
        .await
    {
        Ok(response) => {
            println!("✅ 消息发送成功: ID={}, Success={}", response.message_id, response.success);
        }
        Err(error) => {
            println!("❌ 消息发送失败: {}", error);
        }
    }

    // 测试用户创建构建器
    println!("\n=== 测试用户创建构建器 ===");
    match UserCreateBuilder::new()
        .name("李四")
        .department_id("dept_002")
        .email("lisi@example.com")
        .execute(&service)
        .await
    {
        Ok(response) => {
            println!("✅ 用户创建成功: ID={}, InviteUrl={:?}", response.user_id, response.invite_url);
        }
        Err(error) => {
            println!("❌ 用户创建失败: {}", error);
        }
    }

    // 测试错误处理
    println!("\n=== 测试错误处理 ===");
    match MessageSendBuilder::new()
        .text("") // 空消息，应该触发验证错误
        .receive_id("test_user")
        .execute(&service)
        .await
    {
        Ok(_) => {
            println!("❌ 期望错误但操作成功了");
        }
        Err(error) => {
            println!("✅ 正确捕获验证错误: {}", error);
        }
    }

    println!("\n构建器框架 POC 测试完成！");
    Ok(())
}