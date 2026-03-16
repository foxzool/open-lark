
- Task 6: ErrorContext 移除 backtrace 捕获后，需要同步删除 tests/unit/error/error_context_tests.rs 与 crates/openlark-core/src/error/mod.rs 中对 backtrace() 的断言，避免集成测试编译失败。
- ErrorRecord 不再保留 backtrace 字段后，From<&CoreError> 仅映射 message/context/request_id/operation/component。
- Task 7: ErrorContext 时间戳可直接替换为 std::time::SystemTime（字段/构造器/clone_with/getter/debug_format/traits 同步），并可移除 openlark-core 的 chrono 依赖；若仍有历史测试调用 backtrace()，保留兼容空实现可避免测试目标编译失败。

## Task 16: 替换 openlark-client 内 async-trait 为原生 async trait

- 静态分发 trait 直接移除 `#[async_trait]` 即可，Rust 1.75+ 原生支持 async fn in trait
- dyn dispatch（如 `Arc<dyn ServiceFactory>`）必须用 boxed future：`fn method(&self) -> Pin<Box<dyn Future<Output = T> + Send + '_>>`
- `async_trait` 宏自动处理 `Arc::new(x)` 到 `Arc<dyn Any>` 的类型强转，移除后需显式 `as Arc<dyn Any + Send + Sync>`
- 上游 trait 改签名后，下游 impl 必须同步更新（openlark-auth token_provider.rs 残留问题）
- boxed future impl 中引用 `&self` 字段时需先 clone 到局部变量再 move 进 async block
