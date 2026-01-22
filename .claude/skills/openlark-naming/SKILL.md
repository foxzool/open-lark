---
name: openlark-naming
description: OpenLark Rust SDK 命名与对外 API 表达规范（Client/Service/Resource/Request/Builder）。用于新增/重构公开类型、设计 meta 调用链、调整模块导出与 prelude、或排查类似 openlark-docs 中 DocsService 重名/语义混乱与调用风格不一致的问题。触发关键词：命名规范、Client vs Service、Resource、重命名、meta 调用链、公开 API
allowed-tools: Bash, Read, Grep, Glob, Edit
---

# OpenLark 命名规范（Client / Service / Resource / Request / Builder）

## 🧭 技能路由指南

**本技能适用场景：**
- 你在设计/调整对外公开类型名（`pub struct` / `pub type` / re-export / prelude）
- 你在设计 `client.xxx.v1.yyy.zzz` 这类 **meta 调用链**
- 你发现 `*Service` 同名、语义混乱、调用方式不一致，想系统性收敛

**其他技能：**
- 设计审查（更广）→ `Skill(openlark-design-review)`
- 新增/重构单个 API（落盘/端点/Builder 模板）→ `Skill(openlark-api)`

---

## 0) 快速决策：先选类型职责，再命名（必须）

- **顶层入口/门面（面向用户）**：持有 `Config`（或 `Arc<Config>`），组织调用链与透传配置 → `*Client`
- **业务能力集合（可执行）**：对外暴露一组 API，承接/实现通用 trait（如 `Service`、`ExecutableBuilder`）→ `*Service`
- **资源节点/命名空间（只组织层级）**：处在 meta 调用链的中间层，主要做字段分组与 config 透传 → `*Resource`
- **版本层对象**：必须把版本写进类型名 → `*V1Service` / `*V2Service`（或 `*V1Client` 视职责而定）
- **单 endpoint 请求类型**：`*Request` 或 `*RequestBuilder`（同一 crate/模块树二选一并保持一致）

> 约束：**不要用 `*Service` 去命名“仅做层级组织/透传 config 的节点”。**

## 1) `*Client` 命名规则

- 语义：**入口 / 门面 / 组合根**；类型名要让读者知道“从这里开始调用”。
- 典型结构：
  - 持有 `Arc<Config>`
  - 暴露 `pub xxx: XxxResource` / `pub v1: XxxV1Client` 之类字段链
  - 很少直接实现业务方法（除非规模很小且能保持一致）
- 建议放置：`common/chain.rs`（避免被 API 实现校验脚本当成 endpoint 实现文件）

## 2) `*Service` 命名规则

- 语义：**能力载体**；需要能回答“这个类型里提供了一组可执行 API/操作”。
- 若引入通用 trait（例如 `openlark_core::trait_system::Service` / `ExecutableBuilder`），优先在 `*Service` 层承接，保证可观测性（service_name/version）一致。

## 3) 版本层命名（强制：避免同名灾难）

- 版本对象必须显式：`DriveV1Service`、`DocsV2Service`
- 禁止：外层 `DocsService`，内层 `v1::DocsService` 也叫 `DocsService`
  - 会导致 `use` 歧义、re-export 冲突、文档示例难以写清

## 4) `*Resource` 命名规则（meta 调用链中间层）

- 语义：**资源节点/命名空间**，主要职责是组织层级与透传 config
- 参考（正例）：`openlark-cardkit` 的 `CardResource` / `CardElementResource`
- 反例：把所有中间层都叫 `*Service`，最终变成“同名泛滥 + 读者不知道哪里能 execute”

## 5) `*Request` vs `*RequestBuilder`（风格统一，禁止混用）

在同一个 crate（至少同一业务域目录树）里二选一：

### A. Builder 风格（推荐：可统一 execute_with_options）
- `XxxRequestBuilder`：负责参数收集与构建
- `execute(&XxxService)` / `execute_with_options(&XxxService, ...)`：统一执行入口

### B. Self-contained Request 风格（可行但要全局一致）
- `XxxRequest::new(config)`：请求对象持有 `Config`
- `.execute()` / `.execute_with_options(option)`：无需传 service

> 禁止：同一层级里一部分 API 需要 `execute(&service)`，另一部分是 `.new(config).execute()`，会显著增加使用心智与封装成本。

## 6) 名字必须与路径/模块语义一致（避免“路径-名字”错配）

- 模块叫 `doc`，类型不要叫 `DocsService`
- 模块叫 `permission`，类型不要叫 `DriveService`
- 类型名应能让读者大致推断它在哪个 bizTag/project/version/resource 下（至少不会“指向错误模块”）

## 7) openlark-docs 的典型反例（用于触发重命名）

### 7.1 多处 `DocsService` 同名但语义不同

- `crates/openlark-docs/src/ccm/docs/mod.rs:8`：`ccm::docs` 的服务入口也叫 `DocsService`
- `crates/openlark-docs/src/ccm/docs/v1/mod.rs:25`：`ccm::docs::v1` 的版本服务也叫 `DocsService`
- `crates/openlark-docs/src/ccm/doc/mod.rs:65`：模块叫 `doc`，但类型依然叫 `DocsService`（语义错配）

### 7.2 推荐的“立即可落地”改名模板（不考虑兼容）

- 版本层显式化：
  - `ccm::docs::v1::DocsService` → `DocsV1Service`
- 模块名与类型名对齐（尤其是 `doc`/`docs` 这种高风险词）：
  - `ccm::doc::DocsService` → `DocService`（如是旧版 API，可用 `LegacyDocService`）

## 8) 改名 review 清单（提交前逐条过一遍）

- 目录/模块路径是否能从类型名推断（至少到 bizTag/project/version/resource）
- `prelude`/re-export 是否引入同名冲突（尤其是 `DocsService` 这种泛名）
- `*Client` / `*Service` / `*Resource` 的职责是否清晰，调用方式是否一致
- 版本层是否统一采用 `*V{N}Service`（或 `*V{N}Client`），避免重复 `*Service`
