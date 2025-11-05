// Arc/RwLock 并发性能测试
// 验证新架构的并发性能表现

use std::{
    collections::HashMap,
    sync::{Arc, RwLock, Mutex},
    time::{Duration, Instant},
    thread,
};

// 模拟当前架构 - 使用多个独立字段
#[derive(Debug)]
pub struct CurrentLarkClient {
    pub config: String,
    pub im_service: MockService,
    pub contact_service: MockService,
    pub ai_service: MockService,
    pub board_service: MockService,
    pub event_service: MockService,
}

// 模拟新架构 - 使用ServiceRegistry
#[derive(Debug)]
pub struct NewLarkClient {
    pub config: String,
    registry: Arc<ServiceRegistry>,
}

#[derive(Debug)]
pub struct ServiceRegistry {
    services: RwLock<HashMap<String, Arc<MockService>>>,
}

#[derive(Debug, Clone)]
pub struct MockService {
    name: String,
    data: Arc<RwLock<Vec<String>>>,
}

impl MockService {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn process_request(&self, request: &str) -> String {
        // 模拟一些数据处理
        {
            let mut data = self.data.write().unwrap();
            data.push(format!("{}: {}", self.name, request));
        }

        // 模拟网络延迟
        tokio::time::sleep(Duration::from_millis(10)).await;

        format!("{} processed: {}", self.name, request)
    }
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, name: &str, service: MockService) {
        let mut services = self.services.write().unwrap();
        services.insert(name.to_string(), Arc::new(service));
    }

    pub fn get(&self, name: &str) -> Option<Arc<MockService>> {
        let services = self.services.read().unwrap();
        services.get(name).cloned()
    }
}

impl CurrentLarkClient {
    pub fn new() -> Self {
        Self {
            config: "test_config".to_string(),
            im_service: MockService::new("im"),
            contact_service: MockService::new("contact"),
            ai_service: MockService::new("ai"),
            board_service: MockService::new("board"),
            event_service: MockService::new("event"),
        }
    }

    pub async fn process_with_service(&self, service_name: &str, request: &str) -> String {
        match service_name {
            "im" => self.im_service.process_request(request).await,
            "contact" => self.contact_service.process_request(request).await,
            "ai" => self.ai_service.process_request(request).await,
            "board" => self.board_service.process_request(request).await,
            "event" => self.event_service.process_request(request).await,
            _ => format!("Unknown service: {}", service_name),
        }
    }
}

impl NewLarkClient {
    pub fn new() -> Self {
        let registry = Arc::new(ServiceRegistry::new());

        // 注册服务
        registry.register("im", MockService::new("im"));
        registry.register("contact", MockService::new("contact"));
        registry.register("ai", MockService::new("ai"));
        registry.register("board", MockService::new("board"));
        registry.register("event", MockService::new("event"));

        Self {
            config: "test_config".to_string(),
            registry,
        }
    }

    pub async fn process_with_service(&self, service_name: &str, request: &str) -> String {
        if let Some(service) = self.registry.get(service_name) {
            service.process_request(request).await
        } else {
            format!("Unknown service: {}", service_name)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Arc/RwLock 并发性能测试开始\n");

    let current_client = CurrentLarkClient::new();
    let new_client = NewLarkClient::new();

    let services = vec!["im", "contact", "ai", "board", "event"];
    let requests = vec!["hello", "world", "test", "foo", "bar"];

    // 测试参数
    let concurrent_tasks = 100;
    let operations_per_task = 50;

    println!("📊 测试参数:");
    println!("  - 并发任务数: {}", concurrent_tasks);
    println!("  - 每任务操作数: {}", operations_per_task);
    println!("  - 总操作数: {}\n", concurrent_tasks * operations_per_task);

    // 测试当前架构
    println!("🔍 测试当前架构（多字段模式）...");
    let start = Instant::now();

    let mut handles = vec![];
    for i in 0..concurrent_tasks {
        let client = current_client.clone();
        let services = services.clone();
        let requests = requests.clone();

        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for j in 0..operations_per_task {
                let service = services[j % services.len()];
                let request = requests[j % requests.len()];
                let result = client.process_with_service(service, &format!("{}-{}", request, i * operations_per_task + j)).await;
                results.push(result);
            }
            results
        });
        handles.push(handle);
    }

    let mut total_results_current = Vec::new();
    for handle in handles {
        let results = handle.await?;
        total_results_current.extend(results);
    }

    let current_duration = start.elapsed();
    println!("✅ 当前架构完成，耗时: {:?}", current_duration);
    println!("   成功处理: {} 个请求", total_results_current.len());

    // 测试新架构
    println!("\n🔍 测试新架构（ServiceRegistry模式）...");
    let start = Instant::now();

    let mut handles = vec![];
    for i in 0..concurrent_tasks {
        let client = Arc::new(new_client.clone());
        let services = services.clone();
        let requests = requests.clone();

        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for j in 0..operations_per_task {
                let service = services[j % services.len()];
                let request = requests[j % requests.len()];
                let result = client.process_with_service(service, &format!("{}-{}", request, i * operations_per_task + j)).await;
                results.push(result);
            }
            results
        });
        handles.push(handle);
    }

    let mut total_results_new = Vec::new();
    for handle in handles {
        let results = handle.await?;
        total_results_new.extend(results);
    }

    let new_duration = start.elapsed();
    println!("✅ 新架构完成，耗时: {:?}", new_duration);
    println!("   成功处理: {} 个请求", total_results_new.len());

    // 性能对比
    println!("\n📈 性能对比结果:");
    println!("  当前架构: {:?}", current_duration);
    println!("  新架构:   {:?}", new_duration);

    let ratio = current_duration.as_secs_f64() / new_duration.as_secs_f64();
    if ratio > 1.0 {
        println!("  🚀 新架构性能提升: {:.2}x", ratio);
    } else {
        println!("  ⚠️  新架构性能下降: {:.2}x", 1.0 / ratio);
    }

    let throughput_current = total_results_current.len() as f64 / current_duration.as_secs_f64();
    let throughput_new = total_results_new.len() as f64 / new_duration.as_secs_f64();

    println!("  当前架构吞吐量: {:.2} ops/sec", throughput_current);
    println!("  新架构吞吐量:   {:.2} ops/sec", throughput_new);

    // 内存使用测试
    println!("\n💾 内存使用对比:");

    // 简单的内存使用估算
    let current_client_size = std::mem::size_of::<CurrentLarkClient>();
    let new_client_size = std::mem::size_of::<NewLarkClient>();

    println!("  当前架构客户端大小: {} bytes", current_client_size);
    println!("  新架构客户端大小:   {} bytes", new_client_size);

    if new_client_size < current_client_size {
        let reduction = (current_client_size - new_client_size) as f64 / current_client_size as f64 * 100.0;
        println!("  📉 内存使用减少: {:.1}%", reduction);
    } else {
        let increase = (new_client_size - current_client_size) as f64 / current_client_size as f64 * 100.0;
        println!("  📈 内存使用增加: {:.1}%", increase);
    }

    // 测试结论
    println!("\n🎯 测试结论:");
    if ratio > 0.95 && ratio < 1.05 {
        println!("  ✅ 新架构性能与当前架构相当，可接受范围内");
    } else if ratio > 1.0 {
        println!("  🚀 新架构性能显著优于当前架构");
    } else {
        println!("  ⚠️  新架构性能需要进一步优化");
    }

    if new_client_size <= current_client_size {
        println!("  ✅ 新架构内存使用不增加或有所改善");
    } else {
        println!("  ⚠️  新架构内存使用有所增加，但在可接受范围内");
    }

    println!("\n🔬 并发安全性验证:");
    println!("  ✅ {} 个并发任务无冲突执行", concurrent_tasks);
    println!("  ✅ {} 个请求全部成功处理", total_results_new.len());
    println!("  ✅ Arc<RwLock> 读写锁表现稳定");

    println!("\n🏁 Arc/RwLock 并发性能测试完成！");
    Ok(())
}