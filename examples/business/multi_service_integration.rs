//! 多服务集成示例
//!
//! 展示如何集成多个服务构建复杂的企业应用

#[cfg(all(feature = "client-v2", feature = "client-v2-all"))]
use openlark_client::DefaultLarkClient;
#[cfg(all(feature = "client-v2", feature = "client-v2-all"))]
use openlark_core::{config::Config, constants::AppType};

#[cfg(all(feature = "client-v2", feature = "client-v2-all"))]
struct EnterprisePlatform {
    client: DefaultLarkClient,
}

#[cfg(all(feature = "client-v2", feature = "client-v2-all"))]
impl EnterprisePlatform {
    /// 创建企业平台
    pub fn new(app_id: &str, app_secret: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Config::builder()
            .app_id(app_id)
            .app_secret(app_secret)
            .app_type(AppType::SelfBuild)
            .build();

        let client = DefaultLarkClient::new(config);

        Ok(Self { client })
    }

    /// 显示已加载的服务
    pub fn show_loaded_services(&self) {
        println!("🔌 已加载的服务:");
        let services = self.client.services().list_services();
        for (i, service) in services.iter().enumerate() {
            let status = self.check_service_health(service);
            println!("   {}. {} {}", i + 1, service, status);
        }
    }

    /// 检查服务健康状态
    fn check_service_health(&self, service_name: &str) -> &'static str {
        if self.client.services().has_service(service_name) {
            "✅ 可用"
        } else {
            "❌ 不可用"
        }
    }

    /// 文档协作功能
    pub async fn document_collaboration_demo(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n📄 文档协作功能:");

        if self.client.services().has_service("docs") {
            println!("   ✅ 文档服务 - 支持在线文档编辑和协作");
        }
        if self.client.services().has_service("sheet") {
            println!("   ✅ 表格服务 - 支持电子表格协作");
        }
        if self.client.services().has_service("bitable") {
            println!("   ✅ 多维表格 - 支持数据协作");
        }

        // 演示协作功能
        self.create_collaborative_workspace().await?;
        Ok(())
    }

    /// 创建协作工作区
    async fn create_collaborative_workspace(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("   🚀 创建协作工作区...");

        // 这里会实现实际的协作工作区创建逻辑
        // 包括：
        // 1. 创建文档空间
        // 2. 邀请协作者
        // 3. 设置权限
        // 4. 创建初始文档

        println!("   ✅ 协作工作区创建成功");
        println!("   📋 工作区包含: 项目计划文档、进度跟踪表格、数据分析多维表格");
        Ok(())
    }

    /// 团队通信功能
    pub async fn team_communication_demo(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n💬 团队通信功能:");

        if self.client.services().has_service("contact") {
            println!("   ✅ 通讯录服务 - 团队成员管理");
        }
        if self.client.services().has_service("im") {
            println!("   ✅ 即时消息服务 - 团队沟通");
        }

        self.setup_team_channels().await?;
        Ok(())
    }

    /// 设置团队频道
    async fn setup_team_channels(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("   🏢 设置团队频道...");

        // 这里会实现实际的频道设置逻辑
        // 包括：
        // 1. 创建项目讨论频道
        // 2. 创建通知频道
        // 3. 设置频道成员
        // 4. 配置自动化规则

        println!("   ✅ 团队频道设置完成");
        println!("   📋 创建频道: #项目讨论、#技术分享、#公告通知");
        Ok(())
    }

    /// 智能化功能
    pub async fn ai_integration_demo(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🤖 智能化集成:");

        if self.client.services().has_service("ai") {
            println!("   ✅ AI服务 - 智能助手和分析");

            self.setup_ai_assistant().await?;
        } else {
            println!("   ⚠️ AI服务未启用");
        }

        Ok(())
    }

    /// 设置AI助手
    async fn setup_ai_assistant(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("   🤖 设置AI助手...");

        // 这里会实现实际的AI助手设置
        // 包括：
        // 1. 配置AI模型
        // 2. 设置助手技能
        // 3. 集成到工作流
        // 4. 配置自动化响应

        println!("   ✅ AI助手配置完成");
        println!("   🧠 技能: 文档摘要、数据分析、任务提醒、智能问答");
        Ok(())
    }

    /// 生成平台报告
    pub fn generate_platform_report(&self) -> PlatformReport {
        let services = self.client.services().list_services();

        PlatformReport {
            total_services: services.len(),
            loaded_services: services.clone(),
            features: self.extract_features(&services),
            status: if services.len() > 0 {
                "🟢 活跃".to_string()
            } else {
                "🔴 离线".to_string()
            },
            recommendations: self.generate_recommendations(&services),
        }
    }

    /// 提取平台特性
    fn extract_features(&self, services: &[String]) -> Vec<String> {
        let mut features = Vec::new();

        if services.iter().any(|s| s.contains("doc") || s.contains("sheet") || s.contains("bitable")) {
            features.push("文档协作".to_string());
        }

        if services.iter().any(|s| s.contains("contact") || s.contains("im")) {
            features.push("团队通信".to_string());
        }

        if services.iter().any(|s| s.contains("ai")) {
            features.push("智能化".to_string());
        }

        features
    }

    /// 生成推荐
    fn generate_recommendations(&self, services: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if !services.contains(&"contact".to_string()) {
            recommendations.push("建议启用通讯录服务以管理团队成员".to_string());
        }

        if !services.contains(&"sheet".to_string()) {
            recommendations.push("建议启用表格服务以进行数据分析".to_string());
        }

        if !services.contains(&"ai".to_string()) {
            recommendations.push("建议启用AI服务以提升工作效率".to_string());
        }

        recommendations
    }
}

#[cfg(all(feature = "client-v2", feature = "client-v2-all"))]
#[derive(Debug)]
struct PlatformReport {
    total_services: usize,
    loaded_services: Vec<String>,
    features: Vec<String>,
    status: String,
    recommendations: Vec<String>,
}

#[cfg(all(feature = "client-v2", feature = "client-v2-all"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏢 OpenLark 企业集成平台");
    println!("========================");

    // 从环境变量获取配置
    let app_id = std::env::var("APP_ID").unwrap_or_else(|_| "enterprise_app_id".to_string());
    let app_secret = std::env::var("APP_SECRET").unwrap_or_else(|_| "enterprise_app_secret".to_string());

    // 创建企业平台
    let platform = EnterprisePlatform::new(&app_id, &app_secret)?;

    // 显示加载的服务
    platform.show_loaded_services();

    // 演示各项功能
    println!("\n🚀 功能演示:");

    // 文档协作
    platform.document_collaboration_demo().await?;

    // 团队通信
    platform.team_communication_demo().await?;

    // AI集成
    platform.ai_integration_demo().await?;

    // 生成平台报告
    let report = platform.generate_platform_report();

    println!("\n📊 平台报告:");
    println!("   总服务数: {}", report.total_services);
    println!("   平台状态: {}", report.status);
    println!("   核心功能:");
    for feature in report.features {
        println!("     - {}", feature);
    }

    if !report.recommendations.is_empty() {
        println!("   💡 建议:");
        for rec in report.recommendations {
            println!("     - {}", rec);
        }
    }

    println!("\n🎯 企业集成平台演示完成！");
    println!("\n💡 提示: 设置真实的环境变量来使用实际的飞书 API 功能");

    Ok(())
}

#[cfg(not(all(feature = "client-v2", feature = "client-v2-all"))]
fn main() {
    println!("请启用 client-v2-all 功能来运行此示例:");
    println!("cargo run --example multi_service_integration --features client-v2,client-v2-all");
}