// 注意：CSV 的 meta.Name 为 `docs-api/...`，目录名包含 `-`。
// Rust 模块名不能包含 `-`，因此这里通过 `#[path]` 映射到真实目录。
#[path = "docs-api/mod.rs"]
pub mod docs_api;

pub use docs_api::*;
