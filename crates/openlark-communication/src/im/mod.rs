//! 即时通讯（im）
//!
//! 按 bizTag=im 组织的 API 实现。

// 先按 meta.Project 建目录，后续逐个补齐
pub mod card;
#[path = "im/mod.rs"]
mod project;
pub use project::{v1, v2};
#[deprecated(
    since = "0.15.0",
    note = "Use `openlark_communication::im::v1` or `::v2` directly; the nested `im::im` path is a legacy compatibility alias."
)]
pub use project as im;
pub mod im_ephemeral;
pub mod im_message;

#[cfg(test)]
mod tests {
    #[test]
    #[allow(deprecated)]
    fn nested_im_path_remains_a_compatibility_alias() {
        let canonical =
            std::any::type_name::<super::v1::message::create::CreateMessageBody>();
        let legacy =
            std::any::type_name::<super::im::v1::message::create::CreateMessageBody>();

        assert_eq!(canonical, legacy);
    }
}
