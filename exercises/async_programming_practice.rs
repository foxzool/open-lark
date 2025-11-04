// 异步编程深度实践练习
// 基于Open-Lark项目的实际异步API需求

use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
    pin::Pin,
    task::{Context, Poll},
    future::Future,
};

// 模拟Open-Lark的异步基础设施
use async_trait::async_trait;

// ==================== 练习1: 基础async/await ====================
// 目标：理解Rust异步编程基础

#[derive(Debug, Clone)]
struct ApiResponse {
    success: bool,
    data: String,
    latency: Duration,
}

impl ApiResponse {
    fn new(data: String, latency: Duration) -> Self {
        Self {
            success: true,
            data,
            latency,
        }
    }
}

// TODO: 实现基础的异步函数
// 模拟网络API调用
async fn simulate_api_call(service_name: &str, request_data: &str) -> ApiResponse {
    // 模拟网络延迟
    let latency = Duration::from_millis(
        (service_name.len() + request_data.len()) as u64 % 50 + 10
    );

    // 模拟异步等待
    tokio::time::sleep(latency).await;

    ApiResponse::new(
        format!("{}: processed {}", service_name, request_data),
        latency
    )
}

// TODO: 实现串行vs并行的性能对比
async fn serial_requests() -> Duration {
    let start = Instant::now();

    // 串行执行多个API调用
    let _im_response = simulate_api_call("im", "send_message").await;
    let _contact_response = simulate_api_call("contact", "get_user").await;
    let _approval_response = simulate_api_call("approval", "create_approval").await;

    start.elapsed()
}

async fn parallel_requests() -> Duration {
    let start = Instant::now();

    // 并发执行多个API调用
    let (im_response, contact_response, approval_response) = tokio::join!(
        simulate_api_call("im", "send_message"),
        simulate_api_call("contact", "get_user"),
        simulate_api_call("approval", "create_approval")
    );

    println!("并行响应:");
    println!("  IM: {:?}", im_response.data);
    println!("  Contact: {:?}", contact_response.data);
    println!("  Approval: {:?}", approval_response.data);

    start.elapsed()
}

// ==================== 练习2: async-trait高级应用 ====================
// 目标：掌握异步trait的设计和实现

#[async_trait]
trait Service {
    type Request;
    type Response;
    type Error;

    fn name(&self) -> &str;

    // 异步方法需要async_trait宏
    async fn call(&self, request: Self::Request) -> Result<Self::Response, Self::Error>;

    // 提供默认的异步实现
    async fn health_check(&self) -> bool {
        let health_request = self.health_request();
        self.call(health_request).await.is_ok()
    }

    // 子类需要实现的具体健康检查请求
    fn health_request(&self) -> Self::Request;
}

// TODO: 实现IM服务
#[derive(Debug)]
struct ImService {
    endpoint: String,
    timeout: Duration,
}

#[derive(Debug, Clone)]
struct ImRequest {
    message: String,
    user_id: String,
}

#[derive(Debug, Clone)]
struct ImResponse {
    message_id: String,
    status: String,
}

#[derive(Debug)]
struct ImError {
    message: String,
}

#[async_trait]
impl Service for ImService {
    type Request = ImRequest;
    type Response = ImResponse;
    type Error = ImError;

    fn name(&self) -> &str {
        "IM Service"
    }

    async fn call(&self, request: Self::Request) -> Result<Self::Response, Self::Error> {
        // 模拟网络延迟
        tokio::time::sleep(Duration::from_millis(20)).await;

        if request.message.is_empty() {
            return Err(ImError {
                message: "Message cannot be empty".to_string(),
            });
        }

        Ok(ImResponse {
            message_id: format!("msg_{}", uuid::Uuid::new_v4().simple()),
            status: "sent".to_string(),
        })
    }

    fn health_request(&self) -> Self::Request {
        ImRequest {
            message: "health_check".to_string(),
            user_id: "system".to_string(),
        }
    }
}

impl ImService {
    fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            timeout: Duration::from_secs(30),
        }
    }

    // TODO: 实现高级异步方法
    async fn send_batch_messages(&self, messages: Vec<String>) -> Vec<Result<ImResponse, ImError>> {
        let mut handles = vec![];

        for message in messages {
            let service = self.clone();
            let handle = tokio::spawn(async move {
                let request = ImRequest {
                    message,
                    user_id: "batch_user".to_string(),
                };
                service.call(request).await
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => println!("任务执行失败: {:?}", e),
            }
        }

        results
    }
}

// 注意：需要为ImService实现Clone以支持并发
impl Clone for ImService {
    fn clone(&self) -> Self {
        Self {
            endpoint: self.endpoint.clone(),
            timeout: self.timeout,
        }
    }
}

// ==================== 练习3: 自定义Future实现 ====================
// 目标：深入理解Future trait的工作原理

struct DelayedFuture {
    duration: Duration,
    start: Option<Instant>,
}

impl DelayedFuture {
    fn new(duration: Duration) -> Self {
        Self {
            duration,
            start: None,
        }
    }
}

impl Future for DelayedFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.start {
            None => {
                // 第一次调用，记录开始时间
                self.start = Some(Instant::now());
                Poll::Pending
            }
            Some(start) => {
                // 检查是否已经过了指定时间
                if start.elapsed() >= self.duration {
                    Poll::Ready("Delayed operation completed!".to_string())
                } else {
                    Poll::Pending
                }
            }
        }
    }
}

// TODO: 使用自定义Future
async fn use_custom_future() -> String {
    let delayed = DelayedFuture::new(Duration::from_millis(100));
    delayed.await
}

// ==================== 练习4: 异步ServiceRegistry ====================
// 目标：结合前面学习的智能指针和异步编程

use std::sync::RwLock;

#[derive(Debug)]
struct AsyncServiceRegistry {
    services: Arc<RwLock<HashMap<String, Arc<dyn Service<Request = String, Response = String, Error = String> + Send + Sync>>>>,
}

impl AsyncServiceRegistry {
    fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // TODO: 实现异步服务注册
    async fn register<S>(&self, name: String, service: S)
    where
        S: Service<Request = String, Response = String, Error = String> + Send + Sync + 'static,
    {
        let mut services = self.services.write().expect("写锁应该可用");
        services.insert(name, Arc::new(service));

        // 模拟注册延迟
        tokio::time::sleep(Duration::from_millis(1)).await;
    }

    // TODO: 实现异步服务调用
    async fn call_service(&self, name: &str, request: String) -> Result<String, String> {
        let services = self.services.read().expect("读锁应该可用");

        if let Some(service) = services.get(name) {
            service.call(request).await
        } else {
            Err(format!("服务 '{}' 未找到", name))
        }
    }

    // TODO: 实现批量并发调用
    async fn batch_call(&self, requests: Vec<(String, String)>) -> Vec<Result<String, String>> {
        let mut handles = vec![];

        for (service_name, request) in requests {
            let registry = self.clone();
            let handle = tokio::spawn(async move {
                registry.call_service(&service_name, request).await
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            match handle.await {
                Ok(result) => results.push(result),
                Err(e) => results.push(Err(format!("任务失败: {:?}", e))),
            }
        }

        results
    }

    // TODO: 实现健康检查
    async fn health_check_all(&self) -> HashMap<String, bool> {
        let services = self.services.read().expect("读锁应该可用");
        let mut handles = vec![];

        for (name, service) in services.iter() {
            let name = name.clone();
            let service = service.clone();
            let handle = tokio::spawn(async move {
                let health = service.health_check().await;
                (name, health)
            });
            handles.push(handle);
        }

        let mut results = HashMap::new();
        for handle in handles {
            if let Ok((name, health)) = handle.await {
                results.insert(name, health);
            }
        }

        results
    }
}

impl Clone for AsyncServiceRegistry {
    fn clone(&self) -> Self {
        Self {
            services: Arc::clone(&self.services),
        }
    }
}

// 简单的服务实现用于演示
#[derive(Debug)]
struct SimpleService {
    name: String,
    delay: Duration,
}

#[async_trait]
impl Service for SimpleService {
    type Request = String;
    type Response = String;
    type Error = String;

    fn name(&self) -> &str {
        &self.name
    }

    async fn call(&self, request: String) -> Result<String, String> {
        tokio::time::sleep(self.delay).await;
        Ok(format!("{} 处理请求: {}", self.name, request))
    }

    fn health_request(&self) -> String {
        "health".to_string()
    }
}

// ==================== 测试和演示 ====================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 异步编程深度实践练习\n");

    // 练习1: 基础async/await对比
    println!("📚 练习1: 串行 vs 并行执行");

    let serial_time = serial_requests().await;
    let parallel_time = parallel_requests().await;

    println!("  串行执行耗时: {:?}", serial_time);
    println!("  并行执行耗时: {:?}", parallel_time);

    if parallel_time < serial_time {
        let speedup = serial_time.as_secs_f64() / parallel_time.as_secs_f64();
        println!("  🚀 并行执行提升性能: {:.2}x\n", speedup);
    }

    // 练习2: async-trait应用
    println!("📚 练习2: 异步trait应用");
    let im_service = ImService::new("https://api.larksuite.com".to_string());

    let health = im_service.health_check().await;
    println!("  IM服务健康状态: {}", health);

    let batch_results = im_service.send_batch_messages(vec![
        "Hello".to_string(),
        "World".to_string(),
        "Rust".to_string(),
        "".to_string(), // 空消息会失败
    ]).await;

    println!("  批量消息发送结果:");
    for (i, result) in batch_results.iter().enumerate() {
        match result {
            Ok(response) => println!("    消息{}: 成功 - {}", i, response.message_id),
            Err(e) => println!("    消息{}: 失败 - {}", i, e.message),
        }
    }
    println!();

    // 练习3: 自定义Future
    println!("📚 练习3: 自定义Future");
    let custom_result = use_custom_future().await;
    println!("  自定义Future结果: {}\n", custom_result);

    // 练习4: 异步ServiceRegistry
    println!("📚 练习4: 异步ServiceRegistry");
    let registry = AsyncServiceRegistry::new();

    // 注册服务
    registry.register("im".to_string(), SimpleService {
        name: "IM Service".to_string(),
        delay: Duration::from_millis(10),
    }).await;

    registry.register("contact".to_string(), SimpleService {
        name: "Contact Service".to_string(),
        delay: Duration::from_millis(20),
    }).await;

    registry.register("approval".to_string(), SimpleService {
        name: "Approval Service".to_string(),
        delay: Duration::from_millis(15),
    }).await;

    println!("  服务注册完成");

    // 单个调用
    let single_result = registry.call_service("im", "Hello World".to_string()).await;
    println!("  单个调用结果: {:?}", single_result);

    // 批量调用
    let batch_requests = vec![
        ("im".to_string(), "Message 1".to_string()),
        ("contact".to_string(), "Get user".to_string()),
        ("approval".to_string(), "Create approval".to_string()),
        ("unknown".to_string(), "Test".to_string()), // 不存在的服务
    ];

    let batch_results = registry.batch_call(batch_requests).await;
    println!("  批量调用结果:");
    for (i, result) in batch_results.iter().enumerate() {
        match result {
            Ok(response) => println!("    请求{}: 成功 - {}", i, response),
            Err(e) => println!("    请求{}: 失败 - {}", i, e),
        }
    }

    // 健康检查
    let health_results = registry.health_check_all().await;
    println!("  健康检查结果:");
    for (service, healthy) in health_results {
        println!("    {}: {}", service, if healthy { "✅ 健康" } else { "❌ 异常" });
    }

    println!("\n🎉 异步编程深度练习完成！");
    println!("\n💡 关键学习点:");
    println!("  1. async/await 让异步代码看起来像同步代码");
    println!("  2. 并发执行可以显著提升性能");
    println!("  3. async-trait宏支持异步trait方法");
    println!("  4. 自定义Future可以精确控制异步行为");
    println!("  5. Arc<RwLock> + async = 强大的并发模式");

    Ok(())
}

// 注意：这个练习需要以下依赖在Cargo.toml中：
// [dependencies]
// tokio = { version = "1.0", features = ["full"] }
// async-trait = "0.1"
// uuid = { version = "1.0", features = ["v4"] }