/// ExecutableBuilder trait - 统一的Builder执行接口
///
/// 这个trait为所有Builder提供了统一的execute方法接口，
/// 消除了手动实现重复execute方法的需要。
///
/// # 类型参数
/// - `TService`: 服务类型
/// - `TRequest`: 请求类型  
/// - `TResponse`: 响应类型
///
/// # 方法
/// - `build()`: 构建请求对象
/// - `execute()`: 执行请求
/// - `execute_with_options()`: 带选项执行请求
use async_trait::async_trait;

#[async_trait]
pub trait ExecutableBuilder<TService, TRequest, TResponse>
where
    TService: Send + Sync,
    TRequest: Send + Sync,
    TResponse: Send + Sync,
{
    /// 构建请求对象
    fn build(self) -> TRequest;

    /// 执行请求并返回响应
    async fn execute(self, service: &TService) -> crate::core::SDKResult<TResponse>
    where
        Self: Sized;

    /// 带选项执行请求
    async fn execute_with_options(
        self,
        service: &TService,
        option: crate::core::req_option::RequestOption,
    ) -> crate::core::SDKResult<TResponse>
    where
        Self: Sized;
}
