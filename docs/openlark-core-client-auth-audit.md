# openlark-core / openlark-client / openlark-auth 架构审计（最佳实践版）

> 范围：`crates/openlark-core/`、`crates/openlark-client/`、`crates/openlark-auth/`  
> 原则：**不考虑兼容性**，以最佳实践为导向，允许进行破坏性重构。  
> 目标：给出清晰的“分层边界/职责划分”、可执行的“迁移路线图”、以及按收益排序的“修复清单”。

---

## 0. 一页结论（Executive Summary）

### 当前状态（高层）

- **`openlark-core`**：基础设施能力强（HTTP/错误/重试/可观测性/请求与响应模型），但存在明显“**新旧并存 + 边界膨胀**”问题：既像 infra，也承载了 token/auth 业务逻辑，导致与 `openlark-auth` 的职责重叠。
- **`openlark-auth`**：目录结构清晰、API 组织一致（project/version/resource + endpoint enum），可作为业务 crate 的标杆；但 Cargo features（cache/encryption）与实际实现存在“**承诺不落地**”风险，且存在禁用测试残留。
- **`openlark-client`**：意图做统一入口与服务运行时，但仍处于 WIP：存在“**simulate/mock API**”、大量 lint 放宽、以及至少一处会导致真实 auth 不可用的配置传递缺陷；同时存在两套服务注册/运行时体系并行，复杂度偏高。

### 目标状态（最佳实践）

以“清晰分层 + 单一职责 + 可裁剪 + 可验证”为标准：

1. `openlark-core` 只做 **infra**：HTTP 传输/请求构建/响应解析/错误体系/重试/观测，**不**包含业务服务与 auth/token 业务。
2. `openlark-auth` 只做 **认证业务**：token 获取/刷新/缓存策略（若需要）与 OAuth/OIDC，且所有网络请求通过 `openlark-core` 的 Transport。
3. `openlark-client` 只做 **统一门面**：负责从 `openlark-client::Config` 构建 `openlark-core::Config`，按 feature 暴露业务服务的“薄包装”，并提供一致的错误语义；不维护复杂的“动态注册/运行时”体系（除非有强需求）。

---

## 1. 现状结构审计（What We Have）

### 1.1 openlark-core（infra + 混入业务）

入口：`crates/openlark-core/src/lib.rs`  
主要链路：

- 配置：`crates/openlark-core/src/config.rs`
- HTTP 传输：`crates/openlark-core/src/http.rs`
- 请求翻译：`crates/openlark-core/src/req_translator.rs`
- 响应解析：`crates/openlark-core/src/improved_response_handler.rs`
- 错误体系：`crates/openlark-core/src/error/*`
- 重试：`crates/openlark-core/src/retry_middleware.rs`
- Token 管理：`crates/openlark-core/src/token_manager.rs`
- auth 子模块：`crates/openlark-core/src/auth/*`

审计发现：

- `openlark-core` 同时暴露“New modular structure”与大量 legacy 模块，且存在 token/auth 实现，容易导致边界漂移与重复实现。
- `prelude` 同时 re-export `anyhow::Result` 与 SDKResult/错误类型别名，增加调用方混用风险。

### 1.2 openlark-auth（认证业务，结构较统一）

入口：`crates/openlark-auth/src/lib.rs`

- `services/`：面向用户的服务入口（AuthService/AuthenService/OAuthService）
- `api/`：按版本与资源拆分的 API builder
- `common/api_endpoints.rs`：端点枚举（to_url）
- `utils/`：错误处理与小工具（如 error_handler/url_builder/token_validation）

审计发现：

- 结构可复用，端点 enum 集中，API builder 风格统一。
- `Cargo.toml` 声明的 `cache/encryption` features 与可选依赖目前在代码中没有形成清晰闭环（存在“feature 开了也没效果”的维护风险）。
- tests 中存在 `disabled_test()` 占位文件，建议清理。

### 1.3 openlark-client（统一门面 + 运行时体系 WIP）

入口：`crates/openlark-client/src/lib.rs`

现状特点：

- 大量 `#![allow(...)]` 以保证 lint 通过，代表整体仍是 WIP。
- `services/docs.rs` 已采用“薄包装复用 openlark-docs”的正确方向。
- `services/communication.rs` 仍采用 `simulate_*` 模拟调用（不可作为真实 SDK 行为）。
- `services/auth.rs` 存在关键缺陷：构造函数忽略传入的 `openlark-client::Config`，导致内部使用空的 `openlark-core::Config::default()`（真实调用必失败）。
- 同时存在两套服务体系：`registry/*` 与 `services/runtime/*`，且 loader 只注册了 auth（其余 TODO），复杂度与收益不匹配。

---

## 2. 目标分层边界（What We Want）

### 2.1 分层原则

- **单一职责**：一个 crate 只负责一类问题。
- **依赖方向单向**：业务 crate 依赖 core；client 依赖 core + 业务 crate；core 不依赖任何业务 crate。
- **避免重复实现**：端点、token、错误映射、重试策略应尽量集中在 core 或对应业务 crate。
- **不把“示例/占位”当产品 API**：任何模拟 API 都应移到 `examples/` 或 `tests/`。

### 2.2 目标职责划分

#### openlark-core（Infra）

必须包含：

- `Config`：只包含 HTTP、base_url、请求超时、全局 headers、以及与 token 注入相关的抽象接口（见后）。
- `Transport`：唯一的网络发送入口（tracing/重试/metrics/统一错误映射）。
- `ApiRequest/ApiResponse`：统一请求与响应模型（包括 ResponseFormat/RawResponse）。
- `Error`：统一错误类型（错误码、上下文、恢复建议、重试策略）。
- `Retry`：与 ErrorType/HTTP status/feishu code 联动的重试中间件。

必须移除/下沉：

- token/auth 的业务实现（token_manager、auth/*、app_ticket_manager 等）应迁移到 `openlark-auth`。

#### openlark-auth（认证业务）

必须包含：

- token 获取与刷新：app/tenant/user token、oauth/oidc。
- token 缓存策略（若需要）：“缓存”应是 auth crate 的实现细节，而不是 core 的一部分。
- 错误映射：尽量复用 core 的 ErrorCode/RecoveryStrategy，不要自己再造一套。

不应包含：

- 自行实现 HTTP/重试；全部通过 core 的 Transport。

#### openlark-client（统一门面）

必须包含：

- `openlark-client::Config` → `openlark-core::Config` 的**唯一**桥接处（统一校验、统一默认值）。
- 面向用户的统一入口 `Client`：按 feature 返回各业务 crate 的 service（薄包装或直接 re-export）。
- 一致的可观测性开关（tracing/log），以及“错误展示/建议”的上层 API（可选）。

必须移除/禁止：

- `simulate_*` 伪实现。
- 复杂的动态服务发现/注册系统（除非产品明确需要）；优先使用编译期 feature 与静态组合。

---

## 3. 推荐的配置与 token 注入模型（最佳实践）

### 3.1 核心：Transport 不拥有 token 获取逻辑

最佳实践是让 `Transport` 只负责“把 token 放到请求里”，而 token 的获取与刷新由 `openlark-auth` 完成。

建议在 `openlark-core` 引入一个抽象（示意）：

- `trait TokenProvider { async fn get(&self, scope: TokenScope) -> SDKResult<String>; }`
- `Config` 持有 `Arc<dyn TokenProvider>`（或 `Option<...>` + 按需启用）

然后：

- `openlark-auth` 提供 `AuthTokenProvider` 实现（内部可缓存/刷新）
- 业务 crate（docs/communication/…）只声明自己需要的 token 类型（Tenant/App/User）
- `Transport` 在发送前通过 provider 获取/注入

> 这能彻底消除 “core + auth 都实现 token_manager”的重复问题。

### 3.2 统一错误映射

- core 负责把 Feishu 通用 code / HTTP status / 网络错误 统一映射到 `CoreError`（或一个统一 Error）。
- auth crate 不要维护独立映射表；只在必要时补充“认证语义”的上下文键（如 `feishu_code`、`endpoint`、`request_id`）。

---

## 4. 迁移路线图（Roadmap，不考虑兼容性）

> 这里按“最大收益/最小风险”排序，允许破坏性变更。

### Phase 0（立即修复，阻断性问题）

1. **openlark-client 的 AuthService 配置传递缺陷修复**
   - 现状：`crates/openlark-client/src/services/auth.rs` 构造函数忽略传入 Config，内部用 `openlark_core::config::Config::default()`。
   - 目标：像 `DocsService` 一样，从 `openlark-client::Config` 构建 `openlark-core::Config`，并保证 app_id/app_secret/base_url 生效。

2. **移除/隔离 simulate API**
   - `crates/openlark-client/src/services/communication.rs` 的 `simulate_*` 必须移到 `examples/` 或 `tests/`。
   - 客户端对外 API 不允许“假成功/假响应”。

3. **删掉 `openlark-client` 全局 lint 免检策略**
   - 把 `#![allow(...)]` 缩小到最小范围：仅对确实 WIP 的模块局部 `#[allow]`，而不是整个 crate。

### Phase 1（分层重构，收敛职责）

1. **从 openlark-core 移出 token/auth 业务**
   - 移除：`token_manager.rs`、`auth/*`、`app_ticket_manager.rs` 等与 token 获取相关实现（迁移到 openlark-auth）。
   - core 保留：Transport + Error + Request/Response + Retry + Observability。

2. **openlark-auth 完整接管 token 生命周期**
   - 在 openlark-auth 内实现统一的 token provider（含缓存/刷新/预热策略）。
   - 对外提供可插拔的 TokenProvider，并由 openlark-client 负责装配进 core Config。

3. **统一端点定义风格**
   - 业务 crate（auth/docs/communication 等）统一采用 `common/api_endpoints.rs` 的 enum + `to_url()`。
   - 不再在 client 侧维护端点 HashMap。

### Phase 2（简化 openlark-client：只做门面）

1. **删除 registry/runtime 双体系，保留一种**
   - 最佳实践建议：直接删除动态 registry/runtime（除非要做插件化/热插拔），保留“编译期 feature + 静态暴露服务”。

2. **统一 client 层服务策略：薄包装/直接 re-export**
   - docs 已经是正确示范（`crates/openlark-client/src/services/docs.rs`）。
   - auth/communication/hr/… 统一采用相同策略：复用业务 crate，不在 client 里重写 API。

3. **补齐 client tests 目录与最小集成测试**
   - 至少覆盖：`Client::builder()`、Config from_env、服务构造、不会产生假调用。

### Phase 3（质量门禁）

1. 对 `openlark-core` 启用更严格 lint（逐步 `deny(missing_docs)` 或在 public API 上补文档）。
2. `openlark-auth` 清理 disabled tests，补齐“builder 构建不发网”的测试矩阵。
3. 统一 docstring 风格与示例（确保示例 `no_run` 且不依赖网络/凭证）。

---

## 5. 优先级修复清单（按收益排序）

### P0（阻断使用/明显错误）

- 修复 `openlark-client` auth 配置桥接（见上 Phase 0）。
- 移除所有对外的 `simulate_*`，避免用户误用。
- 清理 `openlark-auth` 的 `disabled_test()` 文件（要么删除，要么实现真实测试）。

### P1（结构性风险/重复实现）

- `openlark-core` 移除 token/auth 业务逻辑，防止 core 无限膨胀。
- 建立 `TokenProvider` 抽象并在 `openlark-auth` 实现，core Transport 只注入 token。
- 统一端点定义：enum+to_url；杜绝 client 侧 hardcode endpoint map。

### P2（可维护性与一致性）

- `openlark-client` 删除 registry/runtime 双体系之一（建议删动态体系）。
- `openlark-client` 恢复 lint 门禁：减少全局 allow，尽量做到局部 allow。
- `openlark-core` 的 `prelude` 规范化：不要同时导出 `anyhow::Result` 与 SDKResult（建议只导出 SDKResult）。

### P3（工程化与长期质量）

- 三个 crate 统一测试策略：
  - 单元测试：不发网，验证 builder/request 构建与序列化。
  - 集成测试：可选（需要凭证时用 `.env-example`/feature gate 控制）。
- 统一文档：按 crate 输出“架构/边界/使用示例/feature 说明/错误处理/重试策略”。

---

## 6. 质量指标与验收标准（建议）

> 不考虑兼容性时，建议以“可解释、可验证、可裁剪”为硬指标。

1. **边界清晰**
   - core 代码中不出现 token 获取/刷新 API（只允许 token provider 抽象）。
   - client 不出现业务 API 的复制实现（薄包装或直接调用业务 crate）。

2. **无假实现**
   - `openlark-client` 对外接口不允许返回模拟值。

3. **可测试**
   - 每个 crate 至少有一组“不发网”测试覆盖公共入口与关键 builder。
   - 不允许 `disabled_test()` 这类占位残留。

4. **lint 可控**
   - WIP 的 allow 必须局部化；禁止在 crate 根部长期全局放宽。

---

## 7. 附：建议的目录形态（示意）

> 仅示意（不考虑兼容性），强调边界与依赖方向。

### openlark-core

- `config/`：Config + TokenProvider trait
- `http/`：Transport + middlewares（retry/otel/trace）
- `api/`：ApiRequest/ApiResponse/ResponseFormat/RawResponse
- `error/`：ErrorCode/CoreError/Context/Recovery

### openlark-auth

- `services/`：AuthService/AuthenService/OAuthService（面向用户/门面）
- `api/`：按 project/version/resource 组织 builder
- `token/`：TokenProvider 实现（cache/refresh/预热策略）
- `common/`：api_endpoints enum

### openlark-client

- `config/`：from_env + validate + bridge_to_core()
- `client.rs`：Client（静态服务访问）
- `services/`：薄包装（docs/auth/communication/...），不写端点、不写协议细节

