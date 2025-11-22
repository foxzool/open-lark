//! Feature 组合使用示例
//!
//! 本示例展示如何使用新的3层架构feature系统

#[cfg(feature = "core-layer")]
fn demo_core_layer() {
    println!("🚀 Core Layer 功能演示");
    println!("覆盖60%用户的基础协作需求");

    #[cfg(feature = "communication")]
    println!("  ✅ IM通讯功能可用");

    #[cfg(feature = "docs")]
    println!("  ✅ 云文档功能可用");

    #[cfg(feature = "auth")]
    println!("  ✅ 身份认证功能可用");
}

#[cfg(feature = "professional-layer")]
fn demo_professional_layer() {
    println!("\n🚀 Professional Layer 功能演示");
    println!("覆盖25%用户的专业协作需求");

    #[cfg(feature = "hr")]
    println!("  ✅ 人力资源功能可用");

    #[cfg(feature = "ai")]
    println!("  ✅ AI智能服务可用");

    #[cfg(feature = "calendar")]
    println!("  ✅ 日程管理功能可用");
}

#[cfg(feature = "enterprise-layer")]
fn demo_enterprise_layer() {
    println!("\n🚀 Enterprise Layer 功能演示");
    println!("覆盖15%用户的完整企业功能需求");

    #[cfg(feature = "admin")]
    println!("  ✅ 系统管理功能可用");

    #[cfg(feature = "approval")]
    println!("  ✅ 审批流程功能可用");

    #[cfg(feature = "helpdesk")]
    println!("  ✅ 帮助支持功能可用");
}

#[cfg(feature = "websocket")]
fn demo_websocket() {
    println!("\n🔌 WebSocket 实时功能可用");
}

#[cfg(feature = "otel")]
fn demo_otel() {
    println!("\n📊 OpenTelemetry 监控功能可用");
}

fn main() {
    println!("🎯 OpenLark Feature 组合使用示例");
    println!("展示新3层架构系统的各种功能组合\n");

    // 根据启用的feature运行对应示例
    #[cfg(feature = "enterprise-layer")]
    {
        demo_enterprise_layer();
        demo_professional_layer(); // 包含了下层功能
        demo_core_layer();
    }
    #[cfg(all(feature = "professional-layer", not(feature = "enterprise-layer")))]
    {
        demo_professional_layer();
        demo_core_layer();
    }
    #[cfg(all(
        feature = "core-layer",
        not(feature = "professional-layer"),
        not(feature = "enterprise-layer")
    ))]
    {
        demo_core_layer();
    }
    #[cfg(not(any(
        feature = "core-layer",
        feature = "professional-layer",
        feature = "enterprise-layer"
    )))]
    {
        println!("❌ 请至少启用一个功能层");
        println!("   --features \"core-layer\"");
        println!("   --features \"professional-layer\"");
        println!("   --features \"enterprise-layer\"");
    }

    // 技术功能演示
    #[cfg(feature = "websocket")]
    demo_websocket();

    #[cfg(feature = "otel")]
    demo_otel();

    println!("\n🎉 功能演示完成！");
}
