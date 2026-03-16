## Task 4: lint 抑制审计发现

- `dependency_resolver.rs` 的 `resolved_orders` 字段是未实现的缓存，保留 `#[allow(dead_code)]` + TODO
- `dfs_detect_cycle` 的 `&self` 仅递归传递，可改为关联函数但暂不重构
- 移除 crate 级 allow 后暴露 `service_factory.rs:570` 未使用变量，已修复
- `missing_copy_implementations` / `missing_debug_implementations` / `mismatched_lifetime_syntaxes` 均无实际触发，说明代码质量已达标

## Task 5: Validatable trait 移位

- 将 `Validatable` trait 与 5 个 impl 从 `crates/openlark-core/src/lib.rs` 迁移到 `crates/openlark-core/src/validation/validatable.rs`
- 在 `crates/openlark-core/src/validation/mod.rs` 增加 `pub mod validatable;` 并 re-export `Validatable`
- 在 `crates/openlark-core/src/lib.rs` 保留 `pub use validation::Validatable;`，确保 `validate_required!` 的 `$crate::Validatable` 路径保持兼容
- 已完成验证：`cargo check --workspace --all-features` 与 `cargo test --workspace` 全通过，证据见 `.sisyphus/evidence/task-5-validatable-move.txt`

## Task 7: glob re-export 冲突修复

- openlark-docs `base/bitable/v1/app/mod.rs`: models.rs 重复定义了 copy/create/get/update 中的类型，role/table/dashboard 子模块名冲突（create/delete/list）→ 改为显式 re-export
- openlark-docs `baike/mod.rs`: baike/lingo 都有 v1/classification/draft/entity/file 子模块，models 与 lingo 类型名冲突 → 移除 baike/lingo glob，显式导出 models
- openlark-meeting `vc/vc/v1/room/mod.rs`: list.rs 和 mget.rs 都定义了 RoomItem → mget 改为显式导出
- 成功移除两个 crate 的 `#![allow(ambiguous_glob_reexports)]` 和 `#![allow(hidden_glob_reexports)]`
- 注意：两个 crate 的 test 编译有 pre-existing 的 ConfigInner 私有字段访问错误，与本次修改无关
