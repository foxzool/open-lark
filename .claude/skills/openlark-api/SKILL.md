---
name: openlark-api
description: OpenLark 项目 API 接口实现规范（速查）。用于添加/重构飞书开放平台 API：确定落盘路径、实现 Body/Response + Builder(Request)、对齐 endpoints 常量/enum、补齐 mod.rs 导出，并明确“调用服务端 API”的方法签名/RequestOption 传递约定。触发关键词：API 接口、API 文件、飞书 API、添加 API、调用服务端 API
allowed-tools:
  - Bash
  - Read
  - Write
  - Edit
  - Glob
  - LspDocumentSymbols
  - LspDiagnostics
---

# OpenLark API 接口实现规范（速查）

本文件只保留“可执行的最小流程”，标准示例与 docPath 抓取能力见 `references/` 与 `scripts/`。

## 0. 快速工作流（新增一个 API 文件）

1) 在仓库根目录 `./api_list_export.csv` 定位目标 API，拿到：
   - `bizTag`、`meta.Project`、`meta.Version`、`meta.Resource`、`meta.Name`、`url`
   - 若存在 `docPath`（文档链接），先抓取“请求体/响应体”定义（见 §4）
2) 选择 feature crate（见 §1；不确定时按现有目录反查）。
3) 计算落盘路径（核心口径）：

```
crates/{feature-crate}/src/{bizTag}/{meta.Project}/{meta.Version}/{meta.Resource...}/{meta.Name}.rs
```

- `meta.Resource` 若包含 `.`，按目录层级拆开：`app.table.record` → `app/table/record/`
- 若该模块已有“重复层级/旧目录”（如 `im/im/v1`、`contact/contact/v3`、`old/default`），以现有结构为准：新增文件放在同级目录下
4) 在文件内实现：`Body/Response` + Builder（`execute/send`）+ 端点（常量或 enum）
   - **必须支持 per-request 选项（RequestOption）**：用于传入 `user_access_token` / `tenant_key` / 自定义 header（对标官方 SDK 的“调用服务端 API”指南）
5) 在相应 `mod.rs` 里 `pub mod ...` / `pub use ...` 导出
6) 统一调用形态（方案 1）：确保 openlark-client 走 `client.<biz>.service().<project>().<version>()...<api>() -> Builder`
   - 在 `crates/{feature-crate}/src/service.rs` 补齐/新增 service 链路方法（见 §6）
   - 若 `client.<biz>` 的链式入口（通常在 `crates/{feature-crate}/src/common/chain.rs`）未提供 `service()`：补一个 `pub fn service(&self) -> crate::service::{Biz}Service`
7) 运行：`just fmt && just lint && just test`

## 1. Feature Crate ↔ bizTag（单一真相）

仓库以 `tools/api_coverage.toml` 作为 **crate→bizTag** 的唯一来源（避免 tool/skill 各写一份导致漂移）。

常用命令：

```bash
# 查看所有映射（crate -> src + biz_tags）
python3 tools/validate_apis.py --list-crates

# 用映射自动补齐 --src/--filter，直接做覆盖率验证
python3 tools/validate_apis.py --crate openlark-docs
python3 tools/validate_apis.py --crate openlark-meeting
```

反查技巧（落盘路径最终以“目标 crate 现有结构”为准）：
- 在 `crates/<crate>/src/` 下查是否存在 `bizTag` / `meta.Project` 相关目录
- 参考 `references/file-layout.md`

## 2. API 文件模板（以仓库现有风格为准）

本仓库多数 API 模块采用“Builder struct + execute/send”的方式，而不是为每个 API 定义一个 trait。

### 2.1 Request / Response

```rust
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Name}Body {
    // 字段按官方文档定义，并用 serde rename 对齐
    // 可选字段：Option<T> + #[serde(skip_serializing_if = "Option::is_none")]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {Name}Response {
    // 字段按官方文档定义
}
```

### 2.2 Builder + execute/send

```rust
pub struct {Name}Request {
    config: Config,
    // 路径/查询参数（按需）
}

impl {Name}Request {
    pub fn new(config: Config) -> Self { /* ... */ }

    /// 默认调用：不显式传入 RequestOption（租户 token 由 TokenProvider 自动处理）
    pub async fn execute(self, body: {Name}Body) -> SDKResult<{Name}Response> {
        self.execute_with_options(body, RequestOption::default()).await
    }

    /// 带选项调用：用于 user_access_token / tenant_key / 自定义 header / request_id 等
    pub async fn execute_with_options(
        self,
        body: {Name}Body,
        option: RequestOption,
    ) -> SDKResult<{Name}Response> {
        // 端点必须复用 crate 的 endpoints 常量或 enum（禁止手写 "/open-apis/..."）
        // 具体写法以目标 crate 现有风格为准：见 references/standard-example.md
        let req: ApiRequest<{Name}Response> = ApiRequest::post({ENDPOINT_CONST_OR_ENUM});
        // ⚠️ 注意：必须把 option 传给 Transport（否则 token 推断/注入不会生效）
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}
```

## 3. 提交前检查清单

- [ ] 落盘路径正确（crate + 目录层级与同模块现有结构一致）
- [ ] Request/Response 字段对齐官方文档（含 `serde(rename)`）
- [ ] HTTP 方法与 `url` 字段一致；path 常量/enum 端点使用一致
- [ ] 对应 `mod.rs` 已导出（必要时 client 链路可访问）
- [ ] `crates/{feature-crate}/src/service.rs` 已提供链式访问 `{bizTag}().{meta.Project}().{meta.Version}().{meta.Resource...}().{meta.Name}()`（或与现有结构等价）
- [ ] 若该 API 可能需要用户态 token：已提供 `execute_with_options(..., RequestOption)`（或函数签名 `(..., option: Option<RequestOption>)`），并将 option 透传到 `Transport::request`
- [ ] `just fmt && just lint && just test` 通过

## 4. docPath 网页读取（生成字段草稿）

当 CSV 提供 `docPath` 时，优先用脚本抓取文档页的“请求/响应定义”（失败则退回手动复制粘贴）。

```bash
python3 .claude/skills/openlark-api/scripts/fetch_docpath.py "<docPath>" --format md --out /tmp/doc.md
```

然后把 `/tmp/doc.md` 中的“请求体/响应体/示例 JSON/字段表”贴进上下文，用于生成 `Body/Response` 字段。

## 5. References（标准示例 + 规则）

- 目录规范与反查：`references/file-layout.md`
- `api_list_export.csv` 映射与提取规则：`references/csv-mapping.md`
- 标准示例（建议照抄结构）：`references/standard-example.md`

## 6. service.rs 链式调用（给 openlark-client 使用）

目标：让 `openlark-client` 能稳定走 **方案 1**：`client.<biz>.service().<project>().<version>()...<api>() -> Builder`，而不是在 client 内重复实现 API。

推荐实现方式：
- 若 crate 已有 `src/service.rs`：在顶层 service 中新增 `pub fn {bizTag}(&self) -> ...`，逐层返回 `{Project}Service` / `{Version}Service` / `{Resource}Service`
- 若 crate 还没有 `src/service.rs`：创建该文件并在 `lib.rs` 里 `pub mod service;`

最终调用链应能覆盖（等价即可）：
`client.<biz>.service().{meta.Project}().{meta.Version}().{meta.Resource...}().{meta.Name}()`

具体可参考 `references/standard-example.md` 的 service 链示例（以 `openlark-docs` 为样板）。

## 7. 调用服务端 API：方法调用约定（对标官方 SDK 指南）

目标：让调用侧具备与官方 SDK 一致的心智模型——“构建 Client → 构建 Request/Builder → 调用方法 → 可选传入 per-request 选项（token/header/request_id）”。

### 7.1 入口选择（推荐顺序）

1) **openlark-client 单入口**：`Client::from_env()` / `Client::with_config(...)`
2) **统一调用形态（方案 1）**：只走 `client.<biz>.service()`，后续全部通过 service 链定位到 Builder
3) **直接用 feature crate（高级/测试）**：`openlark_{biz}::service::{Biz}Service::new(core_config)`（绕过 openlark-client，但保持相同 service 链）

### 7.2 per-request 选项（RequestOption）何时用

- **用户态 API**：必须传 `user_access_token`
- **商店应用（Marketplace）租户态**：通常需要 `tenant_key` / `app_ticket`（取决于 token 获取/接口要求）
- **自定义 header / request_id**：用于链路追踪与排障

> 实现侧要求：option 只能通过 `Transport::request(..., Some(option))` 生效；不要只调用 `ApiRequest::request_option(...)`（它目前仅合并 header）。

### 7.3 新 API 的“方法签名”硬性约定（新增/重构时统一）

二选一即可，但必须 **保证 option 透传到 Transport**：

- **Builder 型 API**：提供 `execute(...)` + `execute_with_options(..., RequestOption)`（见 §2.2）
- **函数型 API**：使用 `fn xxx(request, config, option: Option<RequestOption>) -> SDKResult<_>`，并在内部 `Transport::request(api_request, config, option)`（禁止丢弃 option）

### 7.4 最小调用示例（以“Builder + option”为准）

```rust
use openlark_client::prelude::*;
use openlark_core::req_option::RequestOption;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::from_env()?;

    // 统一调用形态（方案 1）：client.<biz>.service()...<api>() -> Builder
    // 典型：租户态调用（默认 token 由 TokenProvider 自动处理）
    // let resp = client.docs.service().base().v2().app().role().list().send().await?;

    // 用户态调用：显式传 user_access_token（要求 API 实现 execute_with_options/option 透传）
    let option = RequestOption::builder()
        .user_access_token("user_access_token")
        .request_id("trace-id-123")
        .build();
    // let resp = client
    //     .docs
    //     .service()
    //     .base()
    //     .v2()
    //     .app()
    //     .role()
    //     .create()
    //     .execute_with_options(/* body 或 request */, option)
    //     .await?;

    Ok(())
}
```
