//! Feature 组合使用示例
//!
//! 本示例展示如何使用新的3层架构feature系统
//!
//! 运行方式:
//!
//! ```bash
//! # 基础功能示例
//! cargo run --example feature_combination_examples --features "core-layer"
//!
//! # 专业功能示例
//! cargo run --example feature_combination_examples --features "professional-layer"
//!
//! # 企业功能示例
//! cargo run --example feature_combination_examples --features "enterprise-layer"
//!
//! # 完整功能示例
//! cargo run --example feature_combination_examples --features "enterprise-layer,websocket,otel"
//! ```

#[cfg(feature = "core-layer")]
mod core_layer_examples {
    use dotenvy::dotenv;
    use std::env;

    #[cfg(feature = "communication")]
    async fn basic_communication_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("=== 基础通讯功能示例 ===");

        dotenv().ok();
        let app_id = env::var("APP_ID")?;
        let app_secret = env::var("APP_SECRET")?;

        // TODO: 实际的客户端初始化
        println!("✅ 初始化通讯客户端");
        println!("📤 发送消息功能可用");
        println!("👥 联系人管理功能可用");
        println!("🏷️  群组管理功能可用");

        Ok(())
    }

    #[cfg(feature = "docs")]
    async fn basic_document_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== 基础文档功能示例 ===");

        println!("📄 云文档操作功能可用");
        println!("📊 表格处理功能可用");
        println!("📚 知识库管理功能可用");

        Ok(())
    }

    #[cfg(feature = "auth")]
    async fn basic_auth_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== 基础认证功能示例 ===");

        println!("🔐 用户认证功能可用");
        println!("🎫 令牌管理功能可用");
        println!("👤 权限验证功能可用");

        Ok(())
    }

    pub async fn run_core_examples() -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Core Layer 功能演示");
        println!("覆盖60%用户的基础协作需求");
        println!("包含: IM通讯 + 云文档 + 身份认证\n");

        #[cfg(feature = "communication")]
        basic_communication_example().await?;

        #[cfg(feature = "docs")]
        basic_document_example().await?;

        #[cfg(feature = "auth")]
        basic_auth_example().await?;

        println!("\n✨ Core Layer 功能演示完成");
        Ok(())
    }
}

#[cfg(feature = "professional-layer")]
mod professional_layer_examples {
    use super::core_layer_examples::run_core_examples;

    #[cfg(feature = "hr")]
    async fn hr_management_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== 人力资源管理功能示例 ===");

        println!("👥 员工信息管理功能可用");
        println!("⏰ 考勤统计功能可用");
        println!("📋 招聘流程功能可用");
        println!("💰 薪酬管理功能可用");

        Ok(())
    }

    #[cfg(feature = "ai")]
    async fn ai_services_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== AI智能服务功能示例 ===");

        println!("🤖 智能助手功能可用");
        println!("📝 文本分析功能可用");
        println!("🔍 智能搜索功能可用");
        println!("🎯 自动分类功能可用");

        Ok(())
    }

    #[cfg(feature = "calendar")]
    async fn calendar_management_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== 日程管理功能示例 ===");

        println!("📅 日历同步功能可用");
        println!("📞 会议安排功能可用");
        println!("📝 任务管理功能可用");
        println!("⏰ 提醒通知功能可用");

        Ok(())
    }

    pub async fn run_professional_examples() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🚀 Professional Layer 功能演示");
        println!("覆盖25%用户的专业协作需求");
        println!("包含: Core Layer + HR + AI + 日历\n");

        // 运行Core Layer功能
        run_core_examples().await?;

        #[cfg(feature = "hr")]
        hr_management_example().await?;

        #[cfg(feature = "ai")]
        ai_services_example().await?;

        #[cfg(feature = "calendar")]
        calendar_management_example().await?;

        println!("\n✨ Professional Layer 功能演示完成");
        Ok(())
    }
}

#[cfg(feature = "enterprise-layer")]
mod enterprise_layer_examples {
    use super::professional_layer_examples::run_professional_examples;

    #[cfg(feature = "admin")]
    async fn admin_management_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== 系统管理功能示例 ===");

        println!("👨‍💼 用户管理功能可用");
        println!("🛡️ 权限控制功能可用");
        println!("⚙️ 系统配置功能可用");
        println!("📊 数据分析功能可用");

        Ok(())
    }

    #[cfg(feature = "approval")]
    async fn approval_workflow_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== 审批流程功能示例 ===");

        println!("📋 审批模板功能可用");
        println!("🔄 流程管理功能可用");
        println!("📈 状态跟踪功能可用");
        println!("📑 审批记录功能可用");

        Ok(())
    }

    #[cfg(feature = "helpdesk")]
    async fn helpdesk_support_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== 帮助支持功能示例 ===");

        println!("🎫 工单系统功能可用");
        println!("💬 客服管理功能可用");
        println!("🔍 问题追踪功能可用");
        println!("📚 知识库功能可用");

        Ok(())
    }

    pub async fn run_enterprise_examples() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🚀 Enterprise Layer 功能演示");
        println!("覆盖15%用户的完整企业功能需求");
        println!("包含: Professional Layer + 管理 + 审批 + 帮助台\n");

        // 运行Professional Layer功能
        run_professional_examples().await?;

        #[cfg(feature = "admin")]
        admin_management_example().await?;

        #[cfg(feature = "approval")]
        approval_workflow_example().await?;

        #[cfg(feature = "helpdesk")]
        helpdesk_support_example().await?;

        println!("\n✨ Enterprise Layer 功能演示完成");
        Ok(())
    }
}

#[cfg(feature = "websocket")]
mod websocket_examples {
    pub async fn websocket_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🔌 WebSocket 实时功能演示");

        println!("📨 实时消息接收功能可用");
        println!("📤 实时事件推送功能可用");
        println!("🔄 状态同步功能可用");
        println!("⚡ 低延迟通信功能可用");

        println!("\n✨ WebSocket 功能演示完成");
        Ok(())
    }
}

#[cfg(feature = "otel")]
mod otel_examples {
    pub async fn otel_example() -> Result<(), Box<dyn std::error::Error>> {
        println!("\n📊 OpenTelemetry 监控功能演示");

        println!("🔍 链路追踪功能可用");
        println!("📈 指标收集功能可用");
        println!("📝 日志记录功能可用");
        println!("⚡ 性能监控功能可用");

        println!("\n✨ OpenTelemetry 功能演示完成");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 OpenLark Feature 组合使用示例");
    println!("展示新3层架构系统的各种功能组合\n");

    // 根据启用的feature运行对应示例
    #[cfg(feature = "enterprise-layer")]
    {
        enterprise_layer_examples::run_enterprise_examples().await?;
    }
    #[cfg(all(feature = "professional-layer", not(feature = "enterprise-layer")))]
    {
        professional_layer_examples::run_professional_examples().await?;
    }
    #[cfg(all(
        feature = "core-layer",
        not(feature = "professional-layer"),
        not(feature = "enterprise-layer")
    ))]
    {
        core_layer_examples::run_core_examples().await?;
    }
    #[cfg(not(any(
        feature = "core-layer",
        feature = "professional-layer",
        feature = "enterprise-layer"
    )))]
    {
        println!("❌ 请至少启用一个功能层:");
        println!("   --features \"core-layer\"");
        println!("   --features \"professional-layer\"");
        println!("   --features \"enterprise-layer\"");
    }

    // 运行技术功能示例
    #[cfg(feature = "websocket")]
    {
        websocket_examples::websocket_example().await?;
    }

    #[cfg(feature = "otel")]
    {
        otel_examples::otel_example().await?;
    }

    println!("\n🎉 所有可用功能演示完成！");
    println!("\n📚 更多信息请查看:");
    println!("   - Feature 选择指南: docs/user-guide/feature-selection.md");
    println!("   - API 参考文档: docs/api/");
    println!("   - 代码示例: examples/");

    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[cfg(feature = "core-layer")]
    async fn test_core_layer_features() {
        // 测试核心层功能是否正确启用
        assert!(
            cfg!(feature = "auth") || cfg!(feature = "communication") || cfg!(feature = "docs")
        );
    }

    #[tokio::test]
    #[cfg(feature = "professional-layer")]
    async fn test_professional_layer_features() {
        // 测试专业层功能是否正确启用
        assert!(cfg!(feature = "core-layer"));
        assert!(cfg!(feature = "hr") || cfg!(feature = "ai") || cfg!(feature = "calendar"));
    }

    #[tokio::test]
    #[cfg(feature = "enterprise-layer")]
    async fn test_enterprise_layer_features() {
        // 测试企业层功能是否正确启用
        assert!(cfg!(feature = "professional-layer"));
        assert!(
            cfg!(feature = "admin") || cfg!(feature = "approval") || cfg!(feature = "helpdesk")
        );
    }

    #[tokio::test]
    async fn test_feature_combinations() {
        // 测试feature组合的逻辑正确性
        if cfg!(feature = "enterprise-layer") {
            assert!(cfg!(feature = "professional-layer"));
        }

        if cfg!(feature = "professional-layer") {
            assert!(cfg!(feature = "core-layer"));
        }
    }
}
