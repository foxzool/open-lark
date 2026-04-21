//! 即时通讯模块
//!
//! 提供 IM 业务入口与 v1/v2 版本 API 路由。

// 先按 meta.Project 建目录，后续逐个补齐
/// Card 模块。
pub mod card;
#[path = "im/mod.rs"]
mod project;
/// IM v1/v2 版本入口。
pub use project::{v1, v2};
#[allow(clippy::module_inception)]
#[deprecated(
    since = "0.15.0",
    note = "Use `openlark_communication::im::v1` or `::v2` directly; the nested `im::im` path is a legacy compatibility alias."
)]
/// 兼容历史嵌套路径的 IM 模块别名。
pub mod im {
    pub use super::project::{v1, v2};
}
/// Im Ephemeral 模块。
pub mod im_ephemeral;
/// IM Message 模块。
pub mod im_message;

#[cfg(test)]
mod tests {
    #[test]
    #[allow(deprecated)]
    fn nested_im_path_remains_a_compatibility_alias() {
        let canonical = std::any::type_name::<super::v1::message::create::CreateMessageBody>();
        let legacy = std::any::type_name::<super::im::v1::message::create::CreateMessageBody>();

        assert_eq!(canonical, legacy);
    }
}
