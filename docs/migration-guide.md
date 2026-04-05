# OpenLark 0.15 迁移指南

适用范围：从 `0.14.x` 或更早版本迁移到 `0.15.x`

## 一句话结论

`0.15` 的核心变化不是“多了多少 API”，而是将 SDK 的公开入口、feature 表达和文档路径统一到更稳定的模型：

- 普通用户优先依赖根 crate `openlark`
- `openlark-client` 保留，但不再是默认推荐入口
- feature 命名应表达业务能力，而不是内部实现分层

## 迁移优先级

建议按以下顺序迁移：

1. 先切依赖入口
2. 再切 feature 写法
3. 最后清理历史入口和兼容调用

## 1. 依赖入口迁移

### 推荐写法

```toml
[dependencies]
openlark = "0.15"
```

或按需启用业务 feature：

```toml
[dependencies]
openlark = { version = "0.15", default-features = false, features = ["auth", "communication"] }
```

### 何时继续使用 `openlark-client`

只有在以下场景才建议继续直接依赖 `openlark-client`：

- 你明确要复用高级客户端实现层
- 你需要直接操作客户端层能力，而不是以业务 feature 为中心接入
- 你正在维护内部封装，对下游屏蔽根 crate 的组合 feature

普通业务应用与 SDK 使用者，优先使用 `openlark`。

## 2. feature 模型迁移

### 迁移原则

从 `0.15` 开始，feature 应表达“我要什么能力”，而不是“我要哪个内部层”。

优先使用以下三类 feature：

- 业务 feature：`auth`、`communication`、`docs`、`security`、`hr`、`workflow`、`meeting`、`ai`、`cardkit`、`webhook`
- 技术 feature：`websocket`、`otel`
- 组合 feature：`essential`、`enterprise`、`full`

### 推荐组合

```toml
openlark = "0.15"
openlark = { version = "0.15", features = ["essential"] }
openlark = { version = "0.15", features = ["enterprise"] }
openlark = { version = "0.15", features = ["full"] }
```

## 3. 公开入口迁移

### 推荐入口

```rust
use open_lark::prelude::*;

let client = Client::builder()
    .app_id("your_app_id")
    .app_secret("your_app_secret")
    .build()?;
```

### 推荐访问方式

```rust
client.docs.list_folder_children_all("folder_token", None).await?;
client.docs.find_sheet_by_title("spreadsheet_token", "汇总表").await?;
client.communication;
```

## 4. legacy entrypoint 说明

`legacy_client` 不再作为 `0.15` 的公开迁移目标。

如果你的历史代码依赖旧入口，请按下面的方向调整：

- 旧的“先决定依赖 `openlark` 还是 `openlark-client`”心智，迁移为“默认先用 `openlark`”
- 旧的实现层 feature 心智，迁移为业务能力 feature 心智
- 旧的分散示例入口，迁移为根 crate 与根 examples 的统一入口

## 5. 哪些变化可能影响你

以下变化最可能影响升级：

- 公开文档示例从 `openlark-client` 迁移到 `openlark`
- 部分历史入口不再作为默认推荐路径
- feature 组合的建议写法发生变化

## 6. 升级自检

升级到 `0.15` 后，建议至少确认以下事项：

- 依赖入口是否已经统一到 `openlark`
- README 或内部接入文档是否还保留历史依赖示例
- feature 是否表达业务能力而不是内部实现层
- 公开示例是否仍能编译通过

## 7. 常见问题

### `openlark-client` 被移除了吗？

没有。它仍然存在，但定位从“普通用户默认入口”调整为“高级入口/底层实现层”。

### `0.15` 会立即删除所有历史兼容层吗？

不会。`0.15` 的目标是先统一公开入口和迁移路径，再逐步收敛历史兼容层。

### 我应该优先跟随哪个文档？

优先级建议如下：

1. 根 `README.md`
2. 本文档
3. 对应业务 crate 的 README

## 8. 后续约束

从 `0.15` 开始，任何公开入口或公开 feature 的变化，都应同时提供：

- changelog 说明
- release note 说明
- 必要时的迁移文档更新
