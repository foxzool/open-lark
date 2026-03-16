## 2026-02-27 Task 1
- `#![allow(unexpected_cfgs)]` 移除后，立即暴露真实 feature 配置漂移（本次是 `cardkit` 缺失于 Cargo features）。
- crate 根导出使用 `pub use traits::*;` 容易与 `registry` 的同名类型形成歧义，建议维持显式导出。
- `traits/mod.rs` 建议只导出 trait 和必要辅助类型，避免导出业务含义重叠的 `ServiceMetadata`/`ServiceStatus`。

## 2026-02-27 Task 2
- 对拥有多个变体且上下文字段布局不一的错误枚举，优先在核心类型实现 `map_context`，可彻底消除下游重复 match。
- `with_operation` 场景除更新 `ErrorContext` 外，还需覆盖 `Timeout.operation` 字段，避免 enum 字段与 ctx 出现语义漂移。
- 把上下文注入能力放在 openlark-core 可直接复用到所有 crate，能显著降低 openlark-client 的维护成本。

## 2026-02-27 Task 3
- `DefaultServiceRegistry` 目前是“元信息注册表”而非 IOC 容器：`register_service` 仅写入 `ServiceEntry.metadata`，`instance` 默认为 `None`，运行时访问主要依赖 `Client` 的 feature-gated 字段。
- 服务可用性由两层共同决定：编译期 `#[cfg(feature = ...)]` 决定 `Client` 是否拥有字段，启动期 `registry::bootstrap::register_compiled_services` 决定 registry 中是否出现对应元信息。
- 错误处理统一落在 `CoreError`：registry 错误通过 `error::registry_error` 映射为内部错误，`Client::with_config` 与 `ClientErrorHandling` 通过 `with_context/with_operation_context` 注入操作上下文。
