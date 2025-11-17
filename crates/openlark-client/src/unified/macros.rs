//! OpenLark 统一客户端宏
//!
//! 提供代码生成和简化的宏定义。

/// 简化异步函数调用的宏
#[macro_export]
macro_rules! async_block {
    ($($tt:tt)*) => {
        {
            let future = async move { $($tt)* };
            future.await
        }
    };
}