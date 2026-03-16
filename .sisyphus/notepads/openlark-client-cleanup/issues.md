## 2026-02-27 Task 1
- 已知：当前环境未配置 `.toml` 的 LSP，无法对 `Cargo.toml` 做 lsp_diagnostics 验证。
- 已知：`traits/service.rs` 内部 `ServiceStatus` 及其方法存在 `dead_code` 警告，不影响本任务目标，但可在后续 trait 清理任务统一处理。

## 2026-02-27 Task 2
- `cargo test -p openlark-client --all-features` 仍会输出既有 `dead_code` 警告（`traits/service.rs`），本任务未触及该区域。
- 本次新增 CoreError 公共方法后，需关注下游是否会继续保留旧的手写 match 逻辑（建议后续统一替换）。

## 2026-02-27 Task 3
- 本次为只读架构调研，无代码改动风险；仅注意到 `RegistryError` 中 `CircularDependency`/`MissingDependencies`/`InvalidFeatureFlag` 目前在注册路径未被触发，属于“预留能力未落地”状态。
