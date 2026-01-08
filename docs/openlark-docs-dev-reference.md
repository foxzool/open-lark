# openlark-docs 设计与实现规范（供其他模块开发参考）

本文档基于 `ARCHITECTURE.md` 的架构目标、以及 `API_VALIDATION_REPORT.md` 的代码现状审计，总结 `crates/openlark-docs/` 在“模块拆分、命名约定、端点管理、请求/响应组织、验收方法”等方面的实践规范。可作为新增业务模块（例如 openlark-xxx）时的落地参考。

---

## 1. 范围与目标

`openlark-docs` 负责飞书开放平台“云文档相关”能力，当前覆盖 254 个 API（ccm/base/baike/minutes）。它的设计目标是：

- **按业务域组织**：以 `bizTag` 为一级入口（`ccm/base/baike/minutes`），再按 `meta.Project/meta.Version/meta.Resource/meta.Name` 细分。
- **类型安全优先**：请求/响应结构显式建模，路径参数也尽量显式化（通过端点枚举生成 URL）。
- **可验证**：通过 `tools/validate_apis.py` 与 CSV 清单进行对比，保证覆盖率和命名一致性。
- **可演进**：通过 Cargo features 做“模块/版本”粒度的裁剪与组合。

---

## 2. 术语与数据源（与 CSV 对齐）

SDK 的“规范路径”由 CSV 字段驱动（见 `api_list_export.csv`）：

- `bizTag`：业务域（如 `ccm/base/baike/minutes`）
- `meta.Project`：子项目（如 `drive/wiki/docx/sheets/bitable/...`）
- `meta.Version`：版本（如 `v1/v2/v3/old`）
- `meta.Resource`：资源路径（`.` 分隔）
- `meta.Name`：API 名称（可能包含 `/` 子路径、以及 `:` 路径参数）
- `docPath`：文档链接/路径（建议写入每个实现文件头部注释）

验证报告 `API_VALIDATION_REPORT.md` 是“CSV 预期路径”与“代码扫描结果”的差异输出，适合作为覆盖率验收入口。

---

## 3. crate 结构与分层

以 `crates/openlark-docs/src/` 为核心：

### 3.1 顶层入口（lib.rs）

`src/lib.rs` 负责：

- 按 feature 开关声明模块：`ccm/base/baike/minutes`
- 提供兼容性重导出：例如 `pub use base::bitable;`
- 统一 re-export：如 `pub use service::DocsService`

建议：**lib.rs 只做“模块开关 + 重导出 + 文档”**，不要堆积具体 API 逻辑。

### 3.2 服务聚合入口（service.rs）

`src/service.rs` 定义 `DocsService`（聚合入口），按 feature 暴露子服务：

- `docs.ccm()` → `CcmService`
- `docs.bitable()` → `BitableService`
- `docs.base()` → `BaseService`
- `docs.baike()` / `docs.minutes()` → 对应服务

这层的角色是“面向用户的入口”，不要求包含每个 API 的细粒度方法；更推荐让它返回子服务/访问器。

### 3.3 通用能力（common/）

`src/common/` 是复用核心：

- `api_endpoints.rs`：**类型安全端点枚举**（`to_url()` 生成 URL）
- `api_utils.rs`：`serialize_params`、`extract_response_data` 等通用工具
- `builders.rs`：构建器辅助（如存在）

经验：**优先使用端点枚举而不是散落的字符串拼接**，减少路径错误。

### 3.4 端点常量（endpoints/）

`src/endpoints/` 提供大量 endpoint 常量（多为历史/迁移遗留）。在新模块中可选择：

- 仅保留 enum 端点（推荐）
- 或同时保留 const 端点用于兼容与文档说明

关键原则：**API 实现层不要手写 URL 拼接**，要么用 enum，要么用 const + format。

---

## 4. Cargo Features：按业务域/子项目/版本裁剪

`crates/openlark-docs/Cargo.toml` 的特性拆分值得复用：

- 业务域聚合 feature：例如 `ccm = ["ccm-core", ...]`
- 子项目 feature：例如 `ccm-sheets-v2` / `ccm-sheets-v3` / `ccm-sheets`
- 组合 feature：例如 `full`

建议（给新模块）：

1. 先定义 `xxx-core` 作为“最小公共能力”
2. 再按 `project/version` 拆子 feature（便于编译裁剪与增量发布）
3. 最后提供 `xxx` 聚合 feature，和 `full` 组合 feature

---

## 5. 文件路径命名规范（强约束）

### 5.1 规范路径（从 CSV 推导）

原则：实现文件路径满足：

```
src/<bizTag>/<meta.Project>/<meta.Version>/<meta.Resource>/<meta.Name>.rs
```

其中：

- `meta.Resource`：`.` → `/`
- `meta.Name`：`/` 保留为子目录；`:` → `_`（路径参数）

### 5.2 openlark-docs 的“命名风格归一化”

实际代码中（尤其 CCM old/default 路线）常见约定：

- **所有 path segment 使用 snake_case**
  - `dataValidation` → `data_validation`
- **路径参数统一 snake_case**
  - `_docToken` → `_doc_token`
  - `_spreadsheetToken` → `_spreadsheet_token`
- **HTTP 方法分流目录（历史/兼容）**
  - `post_spreadsheets/...`
  - `delete_spreadsheets/...`
  - `put_spreadsheets/...`
  - `get_spreadsheets/...`
- `#`（若出现在 meta.Name）在实现中常规转为 `_`
  - `delete#spreadsheets` → `delete_spreadsheets`

提示：`tools/validate_apis.py` 已在 `openlark-docs` 场景下加入了上述归一化，否则会出现“文件存在但被判定缺失”的误报。

---

## 6. API 实现的标准结构（建议模板）

一个标准 API 文件通常包含：

1. 文件头部 `//!` 文档：包含中文说明 + `docPath`
2. `Request/Response` 类型定义（serde）
3. `impl ApiResponseTrait for Response`：通常 `ResponseFormat::Data`
4. 参数校验：`validate_required!` + 自定义校验（枚举、范围、长度、跨字段一致性）
5. 端点生成：`let api_endpoint = XxxApi::Variant(...)`
6. 请求构造：`ApiRequest::{get,post,put,delete}(...).query(...).body(...)`
7. 发送：`Transport::request(api_request, config, None).await?`
8. 提取数据：`extract_response_data(response, "中文操作名")`

在实现风格上，`openlark-docs` 目前并存两类：

- **函数式**：`pub async fn xxx(config: &Config, ...) -> SDKResult<Response>`（适合简单 API）
- **Builder/Request 对象式**：`struct XxxRequest { config, ... } + execute()/send()`（适合参数多、需要链式构造）

对新模块的建议：

- 复杂 API 优先 Builder（可读性与可扩展性更好）
- 简单 API 可用函数式，避免过度样板
- 如已采用 Builder，尽量统一到 `ExecutableBuilder`（配合 `impl_executable_builder*` 宏）以减少重复 `execute` 实现

---

## 7. 端点管理规范（enum 优先）

### 7.1 为什么用 enum

`common/api_endpoints.rs` 的 enum + `to_url()` 能把“路径参数数量/顺序”变成编译期约束：

- 变体签名就是路径参数契约（少传/多传会编译失败）
- `to_url()` 是唯一拼 URL 的地方，便于审计与统一修复

### 7.2 新增 API 时的端点改动点

新增一个 API，通常需要：

1. 在对应的端点 enum 添加一个 variant（携带路径参数）
2. 在 `impl XxxApi { fn to_url(...) }` 的 match 分支补齐 URL 模板
3. 在实现文件中只引用 `XxxApi::Variant(...)`，不再手写 URL

---

## 8. 参数验证规范（强一致性）

统一使用：

- `validate_required!`：校验必填字符串/容器
- `openlark_core::error::validation_error(...)`：生成可读错误

并补充：

- **枚举/范围校验**：例如 `page_size` 上限、`order_by` 取值、`user_id_type` 取值
- **跨字段一致性**：例如 `ranges` 内 sheetId 与路径 `sheet_id` 必须一致
- **条件校验**：例如某字段为 true 时必须提供另一个字段

---

## 9. 请求构造与响应提取

统一链路建议：

- 请求：
  - `ApiRequest::get/post/put/delete(&api_endpoint.to_url())`
  - query 参数用 `.query(...)` / `.query_opt(...)`
  - body 用 `serialize_params(&params, "中文操作名")?`
- 响应：
  - 统一用 `extract_response_data(response, "中文操作名")`
  - 避免每个文件手写 `response.data.ok_or_else(...)` 的重复逻辑

---

## 10. Service/访问器组织（面向用户的层次）

推荐层次：

1. 顶层 `DocsService`：只负责按业务域返回子服务（`ccm/base/...`）
2. 子服务（如 `CcmService`）：按子项目返回更细粒度服务（`drive/docx/wiki/sheets/...`）
3. 版本访问器（如 `SheetsService::v3()`）：按版本切换
4. API Builder/函数：最终落到某个具体 API 文件

如需要更一致的体验，可让 service 实现 `openlark_core::trait_system::Service`（提供 `config()` 与 `service_name()`），但不强制；保持最小化即可。

---

## 11. 测试与验收（开发闭环）

`openlark-docs` 的验收闭环可复用为新模块标准流程：

1. **覆盖率验证**：运行 `tools/validate_apis.py`（按 bizTag filter）
2. **编译检查**：`cargo test -p <crate> --no-run`
3. **轻量单测**：至少覆盖 service 构造、请求构建器可创建（不依赖真实凭证）

---

## 12. 常见坑（从审计得出的经验）

1. **文件名风格不一致导致验证误报**
   - camelCase vs snake_case、`_docToken` vs `_doc_token`、`#` 的处理等
2. **端点 enum 变体签名与 URL 模板不一致**
   - 典型表现：路径参数多了/少了，但编译可能不报（如果仍是字符串拼接）
3. **“额外实现文件”不一定是问题**
   - 例如 `prelude.rs/versions.rs/error.rs` 属于 glue/helper，不在 CSV 里是正常的
4. **同一路径不同语义的封装**
   - 例如 `sheets_batch_update` 与 `update_sheet_properties` 可能指向同一 URL，但为语义/可用性做了单独封装；需要在文档/验证层明确其定位

---

## 13. 新模块落地 Checklist（建议直接照抄）

1. 定义 Cargo features：`xxx-core` + 子项目/版本 feature + 聚合 feature + `full`
2. `lib.rs`：只做模块声明 + feature gate + 重导出
3. `common/`：先建 `api_endpoints.rs`（enum+to_url）与 `api_utils.rs`
4. 按 CSV 映射创建目录与实现文件，保持 snake_case 与路径参数命名一致
5. 每个 API 文件：
   - 写 `//!` + `docPath`
   - 定义 Request/Response
   - 参数校验
   - enum 端点 + ApiRequest + Transport + extract_response_data
6. 在对应 `mod.rs` 补齐 `pub mod` 与 `pub use`
7. 写最小单测（service 构造 + builder 可创建）
8. 跑验证脚本 + 编译检查

