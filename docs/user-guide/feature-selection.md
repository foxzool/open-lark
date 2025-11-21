# Feature 选择指南

> **文档版本**: 1.0
> **创建时间**: 2025-11-20
> **更新时间**: 2025-11-20

## 🎯 快速开始

### 选择您的使用场景

| 用户类型 | 推荐配置 | Cargo.toml 配置 | 功能覆盖 |
|----------|----------|----------------|----------|
| **个人协作** | 基础功能 | `default = ["core-layer"]` | IM、文档、认证 |
| **团队协作** | 专业功能 | `features = ["professional-layer"]` | 基础功能 + HR + AI + 日历 |
| **企业应用** | 完整功能 | `features = ["enterprise-layer"]` | 所有业务功能 |
| **完整系统** | 企业 + 技术 | `features = ["enterprise-layer", "websocket", "otel"]` | 全功能 + 监控 |

## 🏗️ 3层架构详解

### Level 1: Core Layer (核心基础功能)

**适合用户**: 个人用户、小型团队、基础协作需求
**功能覆盖**: 60% 用户日常使用场景

```toml
[dependencies.open-lark]
version = "0.15"
features = ["core-layer"]
# 或者使用默认配置（已设置为 core-layer）
# version = "0.15"
```

**包含功能**:
- ✅ **即时通讯**: 消息收发、群组聊天、联系人管理
- ✅ **云文档**: 文档编辑、表格处理、知识库管理
- ✅ **身份认证**: 用户登录、权限验证、令牌管理

**使用示例**:
```rust
use open_lark::prelude::*;

// 基础通讯功能
let messages = client.communication().im.v1.message
    .create_message_builder()
    .receive_id("user_open_id")
    .content(r#"{"text":"Hello World"}"#)
    .msg_type("text")
    .execute()?;

// 文档操作
let docs = client.docs().sheets.v2.range
    .read_range()
    .spreadsheet_token("spreadsheet_token")
    .range("Sheet1!A1:C10")
    .execute()?;

// 用户认证
let auth_info = client.auth().user.info.get_self_info().await?;
```

### Level 2: Professional Layer (专业协作功能)

**适合用户**: 中型企业、专业团队、高级协作需求
**功能覆盖**: 85% 企业协作场景

```toml
[dependencies.open-lark]
version = "0.15"
features = ["professional-layer"]
```

**包含功能** (Core Layer +):
- ✅ **人力资源**: 考勤管理、招聘流程、员工信息
- ✅ **AI 智能**: 智能助手、文本分析、自动化处理
- ✅ **日程管理**: 日历同步、会议安排、任务管理

**升级示例**:
```rust
// 从 Core Layer 升级
// Cargo.toml: features = ["professional-layer"]

// HR 功能
let attendance = client.hr().attendance.v1.user_list
    .get_user_attendance_records()
    .user_ids(&["user_1", "user_2"])
    .execute()?;

// AI 功能
let ai_response = client.ai().lingo.v1.classification
    .text_classification()
    .text("这是一段需要分类的文本")
    .execute()?;

// 日历功能
let events = client.calendar().v4.event
    .list_events()
    .calendar_id("calendar_id")
    .time_min("2024-01-01T00:00:00Z")
    .time_max("2024-01-31T23:59:59Z")
    .execute()?;
```

### Level 3: Enterprise Layer (企业级功能)

**适合用户**: 大型企业、完整业务流程、高级管理需求
**功能覆盖**: 100% 功能集合

```toml
[dependencies.open-lark]
version = "0.15"
features = ["enterprise-layer"]
```

**包含功能** (Professional Layer +):
- ✅ **系统管理**: 用户管理、权限控制、系统配置
- ✅ **审批流程**: 审批模板、流程管理、状态跟踪
- ✅ **帮助支持**: 工单系统、客服管理、问题追踪

**完整功能示例**:
```rust
// 管理功能
let admin_users = client.admin().v1.user
    .get_user_list()
    .page_size(50)
    .execute()?;

// 审批流程
let approval_instance = client.approval().v4.instance
    .create_instance()
    .approval_code("approval_code")
    .user_id("user_open_id")
    .execute()?;

// 帮助台
let helpdesk_ticket = client.helpdesk().v1.ticket
    .create_ticket()
    .title("系统问题报告")
    .content("详细问题描述...")
    .execute()?;
```

## 🔧 技术支持功能

### WebSocket 实时功能

**用途**: 实时消息推送、事件监听、状态同步

```toml
[dependencies.open-lark]
version = "0.15"
features = ["core-layer", "websocket"]
```

```rust
use open_lark::websocket::{WebSocketClient, EventHandler};

// 创建 WebSocket 连接
let ws_client = WebSocketClient::builder()
    .app_id("app_id")
    .app_secret("app_secret")
    .event_handler(MyEventHandler::new())
    .build()?;

// 连接并监听事件
ws_client.connect().await?;

// 自定义事件处理器
struct MyEventHandler;

impl EventHandler for MyEventHandler {
    async fn handle_message(&self, event: MessageEvent) -> Result<()> {
        println!("收到消息: {:?}", event);
        Ok(())
    }
}
```

### OpenTelemetry 可观测性

**用途**: 性能监控、链路追踪、指标收集

```toml
[dependencies.open-lark]
version = "0.15"
features = ["core-layer", "otel"]
```

```rust
use opentelemetry::global;
use open_lark::otel::init_tracer;

// 初始化链路追踪
let tracer = init_tracer("my-lark-app")?;

// 在 API 调用中使用追踪
let _span = tracer.start("api_call");
let result = client.communication().im.v1.message.send(message).await?;
global::shutdown_tracer_provider();
```

## 📊 使用场景推荐

### 场景 1: 个人聊天机器人

**需求**: 接收消息、AI 回复、文档处理

**推荐配置**:
```toml
[dependencies.open-lark]
version = "0.15"
features = ["core-layer", "ai", "websocket"]
```

**功能特点**:
- 🔄 实时消息接收 (`websocket`)
- 💬 智能回复生成 (`ai`)
- 📄 文档内容处理 (`docs`)

### 场景 2: 企业办公自动化

**需求**: 考勤管理、审批流程、数据统计

**推荐配置**:
```toml
[dependencies.open-lark]
version = "0.15"
features = ["enterprise-layer"]
```

**功能特点**:
- ⏰ 自动考勤统计 (`hr`)
- 📋 审批流程自动化 (`approval`)
- 📊 数据报表生成 (`admin`)

### 场景 3: 协作平台集成

**需求**: 多系统集成、实时同步、监控告警

**推荐配置**:
```toml
[dependencies.open-lark]
version = "0.15"
features = ["enterprise-layer", "websocket", "otel"]
```

**功能特点**:
- 🔌 完整业务功能 (`enterprise-layer`)
- 🔄 实时数据同步 (`websocket`)
- 📈 性能监控 (`otel`)

## 🔄 迁移路径

### 渐进式升级

```bash
# 步骤 1: 使用核心功能
cargo add open-lark --features "core-layer"

# 步骤 2: 升级到专业功能
cargo add open-lark --features "professional-layer"

# 步骤 3: 升级到企业功能
cargo add open-lark --features "enterprise-layer"

# 步骤 4: 添加技术支持
cargo add open-lark --features "enterprise-layer,websocket,otel"
```

## ⚡ 性能优化建议

### 最小化二进制大小

```toml
# 仅包含需要的功能
[dependencies.open-lark]
version = "0.15"
features = ["communication"]  # 只需要 IM 功能
default-features = false
```

### 编译时优化

```toml
# 针对特定环境优化
[profile.release]
lto = true          # 链接时优化
codegen-units = 1   # 减少代码生成单元
panic = "abort"      # 减少二进制大小
```

### 运行时优化

```rust
use open_lark::client::ClientBuilder;

// 连接池配置
let client = ClientBuilder::new()
    .app_id("app_id")
    .app_secret("app_secret")
    .connection_pool_size(10)
    .request_timeout(Duration::from_secs(30))
    .build()?;
```

## 🚨 常见问题

### Q: 如何选择合适的 feature？

**A**: 根据您的具体需求：
- 个人/小团队 → `core-layer`
- 企业协作 → `professional-layer`
- 完整业务 → `enterprise-layer`

### Q: 可以动态切换 feature 吗？

**A**: Feature 在编译时确定，不能动态切换。如果需要动态功能，请使用 `enterprise-layer` 并在运行时选择具体功能。

### Q: 如何添加技术支持功能？

**A**: 技术支持功能独立于业务层次：
```toml
features = ["core-layer", "websocket"]  # 基础功能 + 实时消息
features = ["enterprise-layer", "otel"] # 企业功能 + 监控
```


## 📚 相关资源

- [API 参考文档](../api/)
- [代码示例](../../examples/)
- [最佳实践指南](../best-practices/)
- [故障排除指南](../troubleshooting/)

---

**需要帮助？**
- 📧 技术支持: [GitHub Issues](https://github.com/foxzool/open-lark/issues)
- 💬 社区讨论: [Discord 频道](https://discord.gg/openlark)
- 📖 详细文档: [在线文档](https://docs.open-lark.dev)

---

**文档更新**: 请关注版本更新，及时了解最新的功能特性和最佳实践。