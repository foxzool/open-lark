//! 企业文档管理系统
//!
//! 展示如何使用新的 openlark-client 构建一个实际的业务应用

#[cfg(all(feature = "client-v2", feature = "client-v2-docs"))]
use openlark_client::DefaultLarkClient;
#[cfg(all(feature = "client-v2", feature = "client-v2-docs"))]
use openlark_core::{config::Config, constants::AppType};

#[cfg(all(feature = "client-v2", feature = "client-v2-docs"))]
struct DocumentManager {
    client: DefaultLarkClient,
}

#[cfg(all(feature = "client-v2", feature = "client-v2-docs"))]
impl DocumentManager {
    /// 创建新的文档管理器
    pub fn new(app_id: &str, app_secret: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::builder()
            .app_id(app_id)
            .app_secret(app_secret)
            .app_type(AppType::SelfBuild)
            .build();

        let client = DefaultLarkClient::new(config);

        // 验证必需的服务可用
        if client.services().has_service("docs") {
            println!("✅ 文档服务已加载");
        } else {
            println!("❌ 文档服务未加载，请检查功能标志");
            return Err("文档服务未可用".into());
        }

        Ok(Self { client })
    }

    /// 创建新文档
    pub async fn create_document(&self, title: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("📝 创建文档: {}", title);

        // 这里演示服务访问方式
        if let Some(_docs_service) = self.client.service_accessors().docs_ext() {
            // 实际的文档创建逻辑会在这里实现
            // let request = CreateDocumentRequest::builder()
            //     .title(title)
            //     .content(content)
            //     .build();
            // let response = docs_service.v1.create(request).await?;

            println!("✅ 文档创建成功");
            Ok(())
        } else {
            Err("文档服务不可用".into())
        }
    }

    /// 列出所有文档
    pub async fn list_documents(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        println!("📋 获取文档列表");

        if let Some(_docs_service) = self.client.service_accessors().docs_ext() {
            // 模拟文档列表
            let documents = vec![
                "项目规划.docx".to_string(),
                "会议纪要.docx".to_string(),
                "技术文档.docx".to_string(),
            ];

            println!("✅ 找到 {} 个文档", documents.len());
            Ok(documents)
        } else {
            Err("文档服务不可用".into())
        }
    }

    /// 获取系统状态
    pub fn get_system_status(&self) -> SystemStatus {
        let services = self.client.services().list_services();

        SystemStatus {
            client_name: "OpenLark Document Management System".to_string(),
            loaded_services: services,
            is_healthy: !services.is_empty(),
        }
    }
}

#[cfg(all(feature = "client-v2", feature = "client-v2-docs"))]
#[derive(Debug)]
struct SystemStatus {
    client_name: String,
    loaded_services: Vec<String>,
    is_healthy: bool,
}

#[cfg(all(feature = "client-v2", feature = "client-v2-docs"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏢 企业文档管理系统");
    println!("========================");

    // 从环境变量获取配置
    let app_id = std::env::var("APP_ID").unwrap_or_else(|_| "demo_app_id".to_string());
    let app_secret = std::env::var("APP_SECRET").unwrap_or_else(|_| "demo_app_secret".to_string());

    // 创建文档管理器
    let manager = DocumentManager::new(&app_id, &app_secret)?;

    // 显示系统状态
    let status = manager.get_system_status();
    println!("📊 系统状态:");
    println!("   客户端: {}", status.client_name);
    println!("   健康状态: {}", if status.is_healthy { "✅ 正常" } else { "❌ 异常" });
    println!("   已加载服务: {:?}", status.loaded_services);

    // 演示文档管理功能
    println!("\n🔧 功能演示:");

    // 创建文档
    manager.create_document("2024年度计划", "这是一个年度计划的文档...").await?;

    // 列出文档
    let documents = manager.list_documents().await?;
    println!("📄 现有文档:");
    for doc in documents {
        println!("   - {}", doc);
    }

    println!("\n🎯 文档管理系统演示完成！");
    println!("\n💡 提示: 设置真实的环境变量 APP_ID 和 APP_SECRET 来使用实际功能");

    Ok(())
}

#[cfg(not(all(feature = "client-v2", feature = "client-v2-docs")))]
fn main() {
    println!("请启用 client-v2 和 client-v2-docs 功能来运行此示例:");
    println!("cargo run --example document_management_system --features client-v2,client-v2-docs");
}