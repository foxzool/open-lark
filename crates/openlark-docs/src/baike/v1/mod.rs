/// 知识库服务 v1 模块
///
/// 说明：
/// - API 实现文件严格放在 `src/bizTag/project/version/resource/name.rs`（如 `src/baike/baike/v1/...`）。
/// - 这里仅提供版本聚合与服务入口，不属于 API 实现文件。
pub mod baike;

// 重新导出服务入口
pub use baike::BaikeV1Service;
