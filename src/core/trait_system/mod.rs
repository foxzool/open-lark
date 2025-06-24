/// Trait系统模块 - ExecutableBuilder trait和相关宏
///
/// 这个模块提供了统一的Builder执行接口，消除代码重复
pub mod executable_builder;
pub mod macros;

pub use executable_builder::ExecutableBuilder;