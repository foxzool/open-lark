//! ExecutableBuilder trait
//!
//! 为“Builder → Request → execute”模式提供统一的 trait 约束，
//! 以便在各业务 crate 里通过宏批量实现。

use crate::req_option::RequestOption;
use crate::SDKResult;

/// Builder 可执行抽象
#[async_trait::async_trait]
pub trait ExecutableBuilder<S: Sync, Req, Resp>: Sized {
    /// 构建请求
    fn build(self) -> Req;

    /// 执行请求（默认不带 option）
    async fn execute(self, service: &S) -> SDKResult<Resp>;

    /// 执行请求（可选传入 RequestOption）
    ///
    /// 默认实现退化为 `execute`，以兼容仅实现基础 execute 的场景。
    async fn execute_with_options(self, service: &S, _option: RequestOption) -> SDKResult<Resp> {
        self.execute(service).await
    }
}
