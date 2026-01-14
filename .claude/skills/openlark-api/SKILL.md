---
name: openlark-api
description: OpenLark 项目 API 接口实现规范（200 行速查）。用于添加/重构飞书开放平台 API：确定落盘路径、实现 Body/Response + Builder(Request)、对齐 endpoints 常量/enum、补齐 mod.rs 导出。触发关键词：API 接口、API 文件、飞书 API、添加 API
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

1) 在 `api_list_export.csv` 定位目标 API，拿到：
   - `bizTag`、`meta.Project`、`meta.Version`、`meta.Resource`、`meta.Name`、`url`
   - 若存在 `docPath`（文档链接），先抓取“请求体/响应体”定义（见 §4）
2) 选择 feature crate（见下方映射表；不确定时按现有目录反查）。
3) 计算落盘路径（核心口径）：

```
crates/{feature-crate}/src/{bizTag}/{meta.Project}/{meta.Version}/{meta.Resource...}/{meta.Name}.rs
```

- `meta.Resource` 若包含 `.`，按目录层级拆开：`app.table.record` → `app/table/record/`
- 若该模块已有“重复层级/旧目录”（如 `im/im/v1`、`contact/contact/v3`、`old/default`），以现有结构为准：新增文件放在同级目录下
4) 在文件内实现：`Body/Response` + Builder（`execute/send`）+ 端点（常量或 enum）
5) 在相应 `mod.rs` 里 `pub mod ...` / `pub use ...` 导出（必要时同步 client 入口链路）
6) 运行：`just fmt && just lint && just test`

## 1. Feature Crate 映射（常用）

| CSV bizTag | Feature Crate | 说明 |
|-----------|--------------|------|
| `communication` | `openlark-communication` | 通讯协作模块 |
| `docs` | `openlark-docs` | 文档协作模块 |
| `hr` | `openlark-hr` | 人力管理模块 |
| `auth` | `openlark-auth` | 认证模块 |
| `meeting_room`, `vc`, `calendar` | `openlark-meeting` | 会议与日程模块 |
| `mail` | `openlark-mail` | 邮件服务模块 |
| `cardkit` | `openlark-cardkit` | 卡片工具模块 |
| `ai` | `openlark-ai` | AI 服务模块 |
| `helpdesk` | `openlark-helpdesk` | 帮助台模块 |
| `application` | `openlark-application` | 应用管理模块 |
| `security_and_compliance` | `openlark-security` | 安全合规模块 |

反查技巧（优先模仿现有模块结构）：
- 在 `crates/<crate>/src/` 下查是否存在 `bizTag` / `meta.Project` 相关目录
- 参考 `references/file-layout.md`

## 2. API 文件模板（以仓库现有风格为准）

本仓库多数 API 模块采用“Builder struct + execute/send”的方式，而不是为每个 API 定义一个 trait。

### 2.1 Request / Response

```rust
use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};
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
    pub async fn execute(self, body: {Name}Body) -> SDKResult<{Name}Response> {
        let req: ApiRequest<{Name}Response> = ApiRequest::post("/open-apis/...").json_body(&body);
        let resp = Transport::request(req, &self.config, None).await?;
        resp.data.ok_or_else(|| openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据"))
    }
}
```

## 3. 提交前检查清单

- [ ] 落盘路径正确（crate + 目录层级与同模块现有结构一致）
- [ ] Request/Response 字段对齐官方文档（含 `serde(rename)`）
- [ ] HTTP 方法与 `url` 字段一致；path 常量/enum 端点使用一致
- [ ] 对应 `mod.rs` 已导出（必要时 client 链路可访问）
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
