[![crates.io](https://img.shields.io/crates/v/open-lark)](https://crates.io/crates/v/open-lark)](https://crates.io/crates/v/open-lark)]
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)
[![Quality](https://github.com/foxzool/open-lark/actions/workflows/quality.yml/badge.svg)](https://github.com/foxzool/open-lark/actions/workflows/quality.yml/badge.svg)]
[![Documentation](https://docs.rs/open-lark/badge.svg)](https://docs.rs/open-lark/badge.svg)]
![Discord Shield](https://discord.com/api/guilds/1319490473060073532/widget.png?style=shield)

# 飞书开放平台非官方SDK - 企业级高覆盖率Rust实现

> ✅ **openlark-docs 链式调用支持与 API 覆盖率更新**
>
> 🏗️ 22个专业模块，1,134+个API，企业级质量和完整文档支持。
>
> 🎯 **100% API 覆盖** - openlark-docs 实现 254 个 API，零未完成标记

支持自定义机器人、长连接机器人、云文档、飞书卡片、消息、群组、招聘管理等API调用。

## 🚀 快速开始

### 1. 添加依赖

在您的 `Cargo.toml` 中添加：

```toml
[dependencies]
open-lark = "0.15"
```

### 2. 选择功能组合

**默认配置**（推荐新手）：
```toml
open-lark = "0.15"  # 包含 IM 消息、文档协作、认证功能
```

**按需选择**：
```toml
# 纯通讯功能（IM + 联系人 + 群组）
open-lark = { version = "0.15", features = ["communication-core"] }

# 企业协作套件
open-lark = { version = "0.15", features = ["professional-suite"] }

# 人力资源套件（IM + HR + AI）
open-lark = { version = "0.15", features = ["enterprise-suite"] }

# 完整功能
open-lark = { version = "0.15", features = ["full-suite"] }
```

### 3. 基础使用

```rust,no_run
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build()?;

    // 上传文件
    let file_data = std::fs::read("document.pdf")?;
    let result = UploadAllRequest::new(
        client.docs.ccm.drive.v1().config().clone(),
        "document.pdf".to_string(),
        "folder_token".to_string(),
        "explorer".to_string(),
        file_data.len(),
        file_data,
    )
    .execute()
    .await?;

    println!("文件上传成功: {}", result.file_token);

    // 创建多维表格
    let table = client.docs.base.bitable().create_tables(
    tables: vec!["测试表格".to_string()],
        folder_token: "folder_token".to_string(),
    table: false,
    default: true,
    )
    .execute()
    .await?;

    println!("表格创建成功");

    // 创建记录
    let fields = serde_json::json!({
        "姓名": "张三",
        "部门": "技术部",
        "状态": "在职"
    });

    let record = RecordCreateRequest::new(
        client.docs.base.bitable().config().clone(),
        "app_token".to_string(),
        "table_id".to_string(),
        fields,
    )
    .execute()
    .await?;

    println!("记录创建成功: {}", record.data.record_id);

    // 创建知识空间
    let space = client.docs.ccm.wiki.v2().space.create(
        name: "技术文档库".to_string(),
        wiki_space_id: "space_id".to_string(),
        description: Some("存储技术文档".to_string()),
    )
    .execute()
    .await?;

    println!("知识空间创建成功: {}", space.data.wiki_space.space_id);

    Ok(())
}
```

## 🔄 架构重构

### 重构原因

本次重构旨在将项目从早期快速开发模式升级为企业级 SDK 架构：

| 目标 | 说明 |
|------|------|
| **🔗 统一 API 调用模式** | 将散乱的 API 实现统一为 Builder 模式，提供一致的开发体验 |
| **🚪 单入口架构** | 实现 ServiceRegistry 统一服务注册，所有服务通过 `Client` 单入口访问 |
| **📦 模块化设计** | 按业务领域拆分为独立 crates（通讯、文档、HR、会议等），支持按需引入 |
| **🧹 技术债务清理** | 清理过时模块、简化 trait 系统、移除死代码和硬编码 URL |

### 重构进度

| 模块 | 状态 | 说明 |
|------|------|------|
| openlark-core | ✅ 完成 | 清理过时模块，简化 trait 系统 |
| openlark-auth | ✅ 完成 | 重构 Token 管理和 API 结构 |
| openlark-cardkit | ✅ 完成 | 重构为 Builder 模式 |
| openlark-client | ✅ 完成 | 实现单入口架构，统一 ServiceRegistry |
| openlark-docs | ✅ 完成 | 254 个 API 100% 覆盖，链式调用支持 |
| openlark-meeting | ✅ 完成 | 重构为统一模式，添加 feature 条件编译 |
| openlark-hr | ✅ 完成 | 重构为统一模式 |
| openlark-communication | ✅ 完成 | 重构 feature 系统，支持条件编译 |
| openlark-protocol | ✅ 完成 | 简化 WebSocket 分包逻辑 |

### 新架构特点

```rust
// 链式调用体验
client.docs.ccm.drive.v1().file().upload(...).execute().await?;
client.docs.base.bitable().record().create(...).execute().await?;

// 统一的服务访问
client.communication.im.v1().message().send(...).execute().await?;
client.meeting.vc.v1().room().create(...).execute().await?;
```

- **🔗 链式调用** - 流畅的 API 调用路径：`client.docs.ccm.drive.v1()`
- **🛡️ 类型安全** - 编译时验证所有 API 调用参数
- **⚡ 按需编译** - 50+ feature flags 支持按需引入功能
- **🏢 企业级质量** - 零警告构建，严格 clippy 检查

## 📖 文档和资源

- **[招聘系统实现报告](reports/hire_v1_implementation_report.md)** - 详细的技术架构和功能说明
- **[openlark-docs AGENTS.md](crates/openlark-docs/AGENTS.md)** - 文档服务模块知识库
- **[API 覆盖率报告](docs/API_COVERAGE_REPORT.md)** - openlark-docs 实现状态分析
- **[功能选择指南](docs/feature-guide.md)** - 完整的功能选择指南
- **[快速启动示例](examples/quick_start.rs)** - 完整功能演示

### 📊 开发者体验革命性改进

- **🔍 透明的项目状态** - 用户可以清楚了解每个模块的实现状态和可用功能
- **📋 清晰的发展路线图** - 详细的3阶段实施计划和量化成功指标
- **🤝 社区共建邀请** - 开放的贡献指南和优先级指导
- **📚 完善的技术文档** - 从模糊宣传转向精确的技术分析

## 📈 项目成熟度提升

- **从工具到平台** - SDK实现从基础工具向企业级开发平台的战略升级
- **质量文化建设** - 建立透明、诚实的项目状态沟通机制
- **可持续发展** - 清晰的模块化架构支持长期维护和功能扩展
- **企业级可靠性** - 通过架构重构显著提升代码质量和系统稳定性

## 📋 TODO

目前主要功能模块均已完成，后续计划：

- [ ] 更多AI能力集成
- [ ] 更多事件处理器支持
- [ ] 性能优化和缓存策略
- [ ] 更多示例和文档

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT OR Apache-2.0
