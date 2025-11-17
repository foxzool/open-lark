//! OpenLark 统一客户端演示
//!
//! 这是一个独立的演示文件，展示统一客户端的设计理念和API接口。

use std::time::Duration;
use std::collections::HashMap;

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
        let mut healthy_services = HashMap::new();

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
            message_id: format!("msg_{}", generate_uuid()),
            send_time: current_time(),
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

    /// 创建电子表格（文档服务）
    pub async fn create_spreadsheet(
        &self,
        title: &str,
        folder_token: Option<&str>,
    ) -> Result<SpreadsheetInfo, Box<dyn std::error::Error>> {
        if !self.is_service_available("docs") {
            return Err("文档服务不可用".into());
        }

        println!("📊 创建电子表格:");
        println!("  标题: {}", title);
        println!("  文件夹令牌: {:?}", folder_token);

        // 模拟API调用
        tokio::time::sleep(Duration::from_millis(200)).await;

        Ok(SpreadsheetInfo {
            spreadsheet_token: format!("sht_{}", generate_uuid()),
            title: title.to_string(),
            url: format!("https://docs.feishu.cn/sheets/{}", generate_uuid()),
            create_time: current_time(),
        })
    }

    /// AI文本生成（AI服务）
    pub async fn generate_text(
        &self,
        prompt: &str,
        model: Option<&str>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> Result<TextGenerationResult, Box<dyn std::error::Error>> {
        if !self.is_service_available("ai") {
            return Err("AI服务不可用".into());
        }

        println!("🤖 AI文本生成:");
        println!("  提示词: {}", prompt);
        println!("  模型: {:?}", model);
        println!("  温度: {:?}", temperature);
        println!("  最大令牌数: {:?}", max_tokens);

        // 模拟API调用
        tokio::time::sleep(Duration::from_millis(300)).await;

        Ok(TextGenerationResult {
            text: "这是AI生成的示例文本，基于您的提示词创作而成。OpenLark统一客户端让AI服务调用变得简单而优雅。".to_string(),
            model: model.unwrap_or("gpt-3.5-turbo").to_string(),
            usage: TokenUsage {
                prompt_tokens: 10,
                completion_tokens: 35,
                total_tokens: 45,
            },
        })
    }

    /// 获取应用访问令牌（认证服务）
    pub async fn get_app_access_token(&self) -> Result<TokenInfo, Box<dyn std::error::Error>> {
        if !self.is_service_available("auth") {
            return Err("认证服务不可用".into());
        }

        println!("🔑 获取应用访问令牌...");

        // 模拟API调用
        tokio::time::sleep(Duration::from_millis(50)).await;

        Ok(TokenInfo {
            access_token: format!("app_access_token_{}", generate_uuid()),
            token_type: "Bearer".to_string(),
            expires_at: current_time() + chrono::Duration::hours(2),
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
    pub services: HashMap<String, bool>,
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

#[derive(Debug)]
pub struct SpreadsheetInfo {
    pub spreadsheet_token: String,
    pub title: String,
    pub url: String,
    pub create_time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct TextGenerationResult {
    pub text: String,
    pub model: String,
    pub usage: TokenUsage,
}

#[derive(Debug)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug)]
pub struct TokenInfo {
    pub access_token: String,
    pub token_type: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

// 辅助函数

fn generate_uuid() -> String {
    // 简单的UUID生成
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut hasher = DefaultHasher::new();
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .hash(&mut hasher);

    format!("{:x}", hasher.finish())
}

fn current_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
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
                "demo_app_id".to_string(),
                "demo_app_secret".to_string(),
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
            "demo_user_001",
            "open_id",
            "Hello from OpenLark 统一客户端! 这是一个全新的API体验。"
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

    // 3. 创建电子表格
    if client.is_service_available("docs") {
        println!("\n📊 测试文档服务...");
        let result = client.create_spreadsheet(
            "OpenLark 统一客户端演示表格",
            None
        ).await?;
        println!("✅ 表格创建成功:");
        println!("  标题: {}", result.title);
        println!("  令牌: {}", result.spreadsheet_token);
        println!("  链接: {}", result.url);
    }

    // 4. AI文本生成
    if client.is_service_available("ai") {
        println!("\n🤖 测试AI服务...");
        match client.generate_text(
            "请写一首关于技术创新的简短诗歌",
            Some("gpt-3.5-turbo"),
            Some(0.8),
            Some(100)
        ).await {
            Ok(result) => {
                println!("✅ AI文本生成成功:");
                println!("  模型: {}", result.model);
                println!("  文本: {}", result.text);
                println!("  令牌使用: {:?}", result.usage);
            }
            Err(e) => {
                println!("❌ AI服务调用失败: {}", e);
            }
        }
    } else {
        println!("\n🤖 AI服务暂时不可用，等待后续版本实现");
    }

    // 5. 获取访问令牌
    if client.is_service_available("auth") {
        println!("\n🔑 测试认证服务...");
        let result = client.get_app_access_token().await?;
        println!("✅ 访问令牌获取成功:");
        println!("  类型: {}", result.token_type);
        println!("  令牌前缀: {}...", &result.access_token[..20.min(result.access_token.len())]);
        println!("  过期时间: {}", result.expires_at);
    }

    println!("\n🎯 统一客户端设计特点:");
    println!("  🔄 统一的API调用模式");
    println!("  🛡️  类型安全的接口设计");
    println!("  🔧 简化的服务管理");
    println!("  📊 完善的错误处理");
    println!("  🚀 高性能异步支持");
    println!("  🏗️  可扩展的架构设计");

    println!("\n🎉 统一客户端演示完成！");
    println!("============================");

    Ok(())
}