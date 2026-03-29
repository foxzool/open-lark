# Root crate / prelude 公开导出规则

本文件定义 `openlark` 根 crate、`openlark-client` 与各业务 crate 的公开导出边界，用于约束后续新增模块接入时的导出策略。

## 目标

- 根 crate 保持“稳定单入口”，优先服务普通 SDK 使用者。
- `prelude` 只保留高频、稳定、跨业务通用的类型。
- 业务 crate 保留自己的原生入口，避免根 crate 继续吸收实现细节。
- 兼容性优先：已有公开路径尽量不移除，新增规则以“收敛新增暴露面”为主。

## 导出分层

### 1. 根 crate `openlark`

根 crate 负责暴露：

- 统一客户端入口：`Client`、`ClientBuilder`
- 配置与基础结果类型：`Config`、`Result`、`SDKResult`
- 高级常用元入口类型：`AuthClient`、`CommunicationClient`、`DocsClient`、`HrClient`、`MeetingClient`、`CardkitClient`
- 按 feature 暴露的业务命名空间别名：`openlark::docs`、`openlark::communication`、`openlark::hr` 等

根 crate **不应**新增导出：

- registry / feature loader / trait 等客户端内部实现细节
- 某个业务 crate 的中间层资源类型或版本层类型
- 仅服务高级调试/内部拼装的结构

### 2. `openlark-client`

`openlark-client` 是高级入口层，可以导出：

- `Client` / `ClientBuilder` / `Config` / `Error` / `Result`
- `ServiceRegistry`、`FeatureLoader` 等客户端层能力
- 各业务 crate 的“顶层 meta client” 类型（如 `DocsClient`、`HrClient`）

`openlark-client::prelude` 只应放：

- 创建客户端必需的核心类型
- 高频错误类型与最常用扩展
- 顶层 meta client 类型

不应放：

- 某个业务域的深层 resource / request 类型
- 仅在少数高级场景才用到的内部辅助类型

### 3. 业务 crate

业务 crate 保留自己的权威入口与局部 `prelude`。

- `openlark-docs`：`DocsClient` 是唯一推荐入口
- `openlark-auth`：`AuthService` / `AuthenService` / `OAuthService` 是业务入口
- `openlark-hr`：`HrClient` 是 HR 业务入口

业务 crate 的 request / model / resource 类型应继续留在各自 crate 内暴露，不上提到根 crate。

## 当前审计结论

### 保留在根 crate 的类型

- `Client`
- `ClientBuilder`
- `Config`
- `Error`
- `Result`
- `SDKResult`
- `RequestOption`
- `CoreConfig`
- `CoreError` / `ErrorCode` / `ErrorSeverity` / `ErrorTrait` / `ErrorType`
- 顶层 meta client：`AuthClient` / `CommunicationClient` / `DocsClient` / `HrClient` / `MeetingClient` / `CardkitClient`

### 下沉到 `openlark-client` 或业务 crate 的类型

- `ServiceRegistry` / `DefaultServiceRegistry` / `ServiceEntry` / `ServiceMetadata` / `ServiceStatus`
- `FeatureLoader`
- `LarkClient` / `ServiceLifecycle` / `ServiceTrait`
- 各业务 crate 中的 request / response / version / resource 类型

### 保持隐藏或避免新增到根 crate 的类型

- `Client::registry()`、`Client::core_config()` 这类高级辅助能力对应的内部支撑类型
- 各 crate 的内部 `common` / `registry` / `bootstrap` / `traits` 细节
- 为兼容历史而保留的方法式入口背后的实现细节

## `prelude` 准入标准

只有同时满足以下条件的类型才允许进入根 crate `prelude`：

1. 普通用户在“创建客户端并发起调用”时高频使用。
2. 语义稳定，不依赖某个业务子域的内部组织方式。
3. 跨 feature 或跨多个业务域都成立，或者属于顶层 meta 入口。
4. 放入 `prelude` 后不会制造命名歧义或重复语义入口。

以下类型默认**不进入**根 crate `prelude`：

- request / response / model
- 版本层 `*V1Service` / `*V2Service`
- resource 层类型
- registry / traits / bootstrap / internal helper

## 新模块接入根 crate 的导出准入标准

新增模块接入 `openlark` 时，按以下顺序判断：

1. **是否已有业务 crate 的权威入口？**
   - 有：根 crate 最多只补顶层 meta client 或命名空间别名。
2. **是否属于普通用户高频入口？**
   - 是：可考虑加入根 crate 顶层 re-export。
3. **是否应该进入 `prelude`？**
   - 仅当它是顶层入口或通用基础类型时才允许。
4. **是否会形成重复路径？**
   - 若 `openlark::X` 与 `openlark::module::X` 语义完全重复，优先保留 canonical 路径并将另一条视为兼容别名，不再继续扩张。

## 本次 issue #42 的落地约束

- 保留现有稳定入口，避免破坏性删除。
- 将 docs/hr 顶层 meta 入口视为允许保留在根 crate 和 `openlark-client` 的高频类型。
- 通过契约测试确保 `client.docs.ccm` 与 `client.hr.attendance` 这类顶层入口可用。
- 后续若新增公开导出，必须先对照本文件判断其归属层级。
