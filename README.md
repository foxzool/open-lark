[![crates.io](https://img.shields.io/crates/v/open-lark)](https://crates.io/crates/open-lark)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/foxzool/open-lark#license)
[![Quality](https://github.com/foxzool/open-lark/actions/workflows/quality.yml/badge.svg)](https://github.com/foxzool/open-lark/actions/workflows/quality.yml)
[![Documentation](https://docs.rs/open-lark/badge.svg)](https://docs.rs/open-lark)
![Discord Shield](https://discord.com/api/guilds/1319490473060073532/widget.png?style=shield)

# 飞书开放平台非官方SDK - 企业级高覆盖率Rust实现

> 🏗️ 18 个业务模块，1,560+ 个 API，企业级质量保证。
>
> 🎯 测试覆盖率 ~47%，零警告构建，全模块 Builder 模式统一。

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
open-lark = { version = "0.15", features = ["communication"] }

# 企业协作套件（IM + 文档 + 认证 + 工作流）
open-lark = { version = "0.15", features = ["core-services"] }

# 人力资源套件
open-lark = { version = "0.15", features = ["core-services", "hr"] }

# 按需组合更多模块
open-lark = { version = "0.15", features = ["core-services", "hr", "meeting", "ai"] }
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

    // 创建数据表
    use openlark_docs::base::bitable::v1::app::table::{CreateTableRequest, TableData};
    let table_request = CreateTableRequest::new(
        client.docs.base.bitable().config().clone(),
    )
    .app_token("app_token".to_string())
    .table(TableData::new("测试表格"));
    let table = table_request.execute().await?;

    println!("表格创建成功: {}", table.table_id);

    // 创建记录
    use openlark_docs::base::bitable::v1::app::table::record::CreateRecordRequest;
    let fields = serde_json::json!({
        "姓名": "张三",
        "部门": "技术部",
        "状态": "在职"
    });

    let record_request = CreateRecordRequest::new(
        client.docs.base.bitable().config().clone(),
    )
    .app_token("app_token".to_string())
    .table_id("table_id".to_string())
    .fields(fields);
    let record = record_request.execute().await?;

    println!("记录创建成功: {}", record.data.record_id);

    // 创建知识空间
    use openlark_docs::ccm::wiki::v2::space::CreateWikiSpaceRequest;
    let space_request = CreateWikiSpaceRequest::new(
        client.docs.ccm.wiki.v2().config().clone(),
    )
    .name("技术文档库".to_string())
    .description("存储技术文档".to_string());
    let space = space_request.execute().await?;

    println!("知识空间创建成功: {}", space.space.as_ref().unwrap().space_id);

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

| 模块 | 状态 | API 数量 | 说明 |
|------|------|---------|------|
| openlark-core | ✅ 完成 | - | 核心基础设施，HTTP 客户端，错误处理 |
| openlark-client | ✅ 完成 | - | 单入口架构，ServiceRegistry |
| openlark-protocol | ✅ 完成 | - | WebSocket 协议 |
| openlark-auth | ✅ 完成 | 15 | Token 管理，认证服务 |
| openlark-hr | ✅ 完成 | 562 | 招聘、CoreHR、考勤、薪酬等 |
| openlark-docs | ✅ 完成 | 158 | 云文档、多维表格、知识库、会议纪要 |
| openlark-communication | ✅ 完成 | 175 | IM 消息、联系人、群组 |
| openlark-workflow | ✅ 完成 | 117 | 任务、审批、看板 |
| openlark-meeting | ✅ 完成 | 117 | 视频会议、日历 |
| openlark-platform | ✅ 完成 | 102 | 平台服务、Transport API |
| openlark-application | ✅ 完成 | 83 | 应用管理 |
| openlark-mail | ✅ 完成 | 68 | 邮件服务 |
| openlark-helpdesk | ✅ 完成 | 56 | 帮助台 |
| openlark-security | ✅ 完成 | 38 | 安全服务 |
| openlark-ai | ✅ 完成 | 29 | AI 智能助手 |
| openlark-analytics | ✅ 完成 | 20 | 数据分析 |
| openlark-cardkit | ✅ 完成 | 10 | 卡片组件 |
| openlark-user | ✅ 完成 | 9 | 用户设置 |

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

- **[openlark-docs AGENTS.md](crates/openlark-docs/AGENTS.md)** - 文档服务模块知识库
- **[快速启动示例](examples/)** - 完整功能演示示例集

## ​​📊 项目状态

- 核心模块稳定 - openlark-core、openlark-client、openlark-auth 生产就绪
- 功能模块持续完善 - 各业务模块 API 持续补充中
- 测试覆盖率 ~47% - 核心模块完整测试覆盖

**已完成**：
- ✅ 核心基础设施（core、client、protocol、auth）
- ✅ 18 个业务模块全部实现，共 1,560+ 个 API
- ✅ openlark-hr 562 个 API（招聘、CoreHR、考勤、薪酬）
- ✅ openlark-communication 175 个 API
- ✅ openlark-docs 158 个 API（云文档、多维表格、知识库）
- ✅ openlark-workflow 117 个 API（任务、审批、看板）
- ✅ openlark-meeting 117 个 API（视频会议、日历）
- ✅ 链式调用架构 + Builder 模式统一
- ✅ Feature flags 按需编译
- ✅ 测试覆盖率 ~47%

**进行中**：
- 🔄 部分 API 结构体字段文档补充
- 🔄 更多示例和文档完善

**计划中**：
- [ ] 更多事件处理器支持
- [ ] WebSocket 功能完善

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT OR Apache-2.0
