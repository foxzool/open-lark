## 2026-02-27 Task 1
- 决策：crate 根仅公开 `LarkClient` 与 `ServiceLifecycle`，不再通过通配符暴露全部 traits。
- 决策：`traits/client.rs` 中 `ClientBuilder` trait 更名为 `ClientBuildable`，避免与 `client::ClientBuilder` 结构体重名冲突。
- 决策：保留 `ServiceTrait` 在 `traits` 模块导出，保证 prelude 现有导入路径不受影响。

## 2026-02-27 Task 2
- 决策：在 `CoreError` 上新增 `map_context` 作为唯一上下文变更入口，不修改任何错误变体定义。
- 决策：通过 `with_context_kv` / `with_operation` 封装高频上下文注入逻辑，保持 client 侧 API 语义不变。
- 决策：`with_operation` 对 `Timeout` 变体同时更新 enum 字段与 context，确保行为与原实现一致。

## 2026-02-27 Task 3
- 决策：后续评审/重构以“registry 仅承载服务元信息（observability/discovery）”为边界，不再引入 ServiceFactory/DependencyResolver 风格的实例生命周期管理层。
- 决策：新增服务时优先遵循既有双点落盘模式：`client.rs` 增加 `#[cfg(feature)]` 字段 + `registry/bootstrap.rs` 增加同名 metadata 注册函数，并在 `Cargo.toml` 维护 feature 依赖闭包。
