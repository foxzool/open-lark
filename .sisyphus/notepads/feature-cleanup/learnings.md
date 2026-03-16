## 2026-02-27 Task 2
- `openlark-client` 中将 `openlark-auth` 设为 optional 后，需要同步把 `auth` feature 显式映射为 `auth = ["openlark-auth"]`，否则启用 feature 不会拉起依赖。
- 对于可选依赖的运行时注入点（如默认 TokenProvider），应在同一函数内用 `#[cfg(feature = "auth")]` / `#[cfg(not(feature = "auth"))]` 提供双分支，保持 API 签名不变。
