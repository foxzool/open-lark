//! OpenLark 统一客户端演示
//!
//! 展示统一客户端接口的使用方法和核心功能。

use std::time::Duration;

// 模拟统一客户端结构和功能
pub struct UnifiedClient {
    app_id: String,
    app_secret: String,
    base_url: String,
}

impl UnifiedClient {
    /// 创建新的统一客户端
    pub fn new(app_id: String, app_secret: String) -> Self {
        Self {
            app_id,
            app_secret,
            base_url: "https://open.feishu.cn".to_string(),
        }
    }

    /// 从环境变量创建客户端
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let app_id = std::env::var("APP_ID")
            .map_err(|_| "未设置APP_ID环境变量")?;
        let app_secret = std::env::var("APP_SECRET")
            .map_err(|_| "未设置APP_SECRET环境变量")?;

        Ok(Self::new(app_id, app_secret))
    }

    /// 获取客户端信息
    pub fn client_info(&self) -> ClientInfo {
        ClientInfo {
            app_id: self.app_id.clone(),
            base_url: self.base_url.clone(),
            version: "1.0.0".to_string(),
        }
    }

    /// 列出可用服务
    pub fn available_services(&self) -> Vec<&'static str> {
        vec![
            "communication", // 通讯服务
            "hr",           // 人力资源服务
            "docs",         // 文档服务
            "ai",           // AI服务
            "auth",         // 认证服务
        ]
    }

    /// 检查服务是否可用
    pub fn is_service_available(&self, service_name: &str) -> bool {
        self.available_services().contains(&service_name)
    }

    /// 执行健康检查
    pub async fn health_check(&self) -> Result<HealthStatus, Box<dyn std::error::Error>> {
        let services = self.available_services();
        let mut healthy_services = std::collections::HashMap::new();

        for service in services {
            // 模拟健康检查
            let is_healthy = match service {
                "communication" => true,
                "hr" => true,
                "docs" => true,
                "ai" => false, // AI服务暂时不可用
                "auth" => true,
                _ => false,
            };
            healthy_services.insert(service.to_string(), is_healthy);
        }

        Ok(HealthStatus {
            services: healthy_services,
            overall_healthy: true,
        })
    }

    // 统一API接口演示

    /// 发送文本消息（通讯服务）
    pub async fn send_text_message(
        &self,
        receive_id: &str,
        receive_id_type: &str,
        content: &str,
    ) -> Result<MessageSendResult, Box<dyn std::error::Error>> {
        if !self.is_service_available("communication") {
            return Err("通讯服务不可用".into());
        }

        println!("📱 发送文本消息:");
        println!("  接收者ID: {}", receive_id);
        println!("  接收者类型: {}", receive_id_type);
        println!("  内容: {}", content);

        // 模拟API调用
        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(MessageSendResult {
            message_id: format!("msg_{}", uuid::Uuid::new_v4()),
            send_time: chrono::Utc::now(),
            status: "success".to_string(),
        })
    }

    /// 获取员工列表（HR服务）
    pub async fn list_employees(
        &self,
        user_id_type: Option<&str>,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<EmployeeListResult, Box<dyn std::error::Error>> {
        if !self.is_service_available("hr") {
            return Err("HR服务不可用".into());
        }

        println!("👥 获取员工列表:");
        println!("  用户ID类型: {:?}", user_id_type);
        println!("  页面大小: {:?}", page_size);
        println!("  页面令牌: {:?}", page_token);

        // 模拟API调用
        tokio::time::sleep(Duration::from_millis(150)).await;

        Ok(EmployeeListResult {
            employees: vec![
                Employee {
                    user_id: "user_001".to_string(),
                    name: "张三".to_string(),
                    email: "zhangsan@example.com".to_string(),
                    department: "技术部".to_string(),
                },
                Employee {
                    user_id: "user_002".to_string(),
                    name: "李四".to_string(),
                    email: "lisi@example.com".to_string(),
                    department: "产品部".to_string(),
                },
            ],
            has_more: false,
            page_token: page_token.map(|s| s.to_string()),
        })
    }
}

// 支持数据结构

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub app_id: String,
    pub base_url: String,
    pub version: String,
}

#[derive(Debug)]
pub struct HealthStatus {
    pub services: std::collections::HashMap<String, bool>,
    pub overall_healthy: bool,
}

#[derive(Debug)]
pub struct MessageSendResult {
    pub message_id: String,
    pub send_time: chrono::DateTime<chrono::Utc>,
    pub status: String,
}

#[derive(Debug)]
pub struct Employee {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub department: String,
}

#[derive(Debug)]
pub struct EmployeeListResult {
    pub employees: Vec<Employee>,
    pub has_more: bool,
    pub page_token: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 OpenLark 统一客户端演示");
    println!("============================");

    // 创建客户端
    let client = match UnifiedClient::from_env() {
        Ok(client) => {
            println!("✅ 从环境变量创建客户端成功");
            client
        }
        Err(_) => {
            println!("⚠️  环境变量未设置，使用测试配置");
            UnifiedClient::new(
                "test_app_id".to_string(),
                "test_app_secret".to_string(),
            )
        }
    };

    // 显示客户端信息
    let info = client.client_info();
    println!("\n📋 客户端信息:");
    println!("  应用ID: {}", info.app_id);
    println!("  基础URL: {}", info.base_url);
    println!("  版本: {}", info.version);

    // 列出可用服务
    let services = client.available_services();
    println!("\n🔧 可用服务: {:?}", services);

    // 健康检查
    println!("\n🏥 执行健康检查...");
    let health = client.health_check().await?;
    for (service, healthy) in health.services {
        let status = if healthy { "✅ 健康" } else { "❌ 异常" };
        println!("  {}: {}", service, status);
    }

    // 演示各种API调用

    // 1. 发送文本消息
    if client.is_service_available("communication") {
        println!("\n💬 测试通讯服务...");
        let result = client.send_text_message(
            "test_user_001",
            "open_id",
            "Hello from OpenLark 统一客户端!"
        ).await?;
        println!("✅ 消息发送成功: {} ({})", result.message_id, result.status);
    }

    // 2. 获取员工列表
    if client.is_service_available("hr") {
        println!("\n👥 测试HR服务...");
        let result = client.list_employees(Some("open_id"), Some(10), None).await?;
        println!("✅ 员工列表获取成功: {} 个员工", result.employees.len());
        for (i, employee) in result.employees.iter().enumerate() {
            println!("  {}. {} - {} ({})",
                i + 1,
                employee.name,
                employee.department,
                employee.email
            );
        }
    }

    println!("\n🎉 统一客户端演示完成！");
    println!("============================");

    Ok(())
}